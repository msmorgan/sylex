//! Structural rules on `Nominal`, `PreModifier` and `PostModifier` calls
//! (`docs/design/spelling.md` §2, `docs/tickets/wip/syl-nominal-descriptions.md`).
//!
//! Checks three invariants after names, sorts, argument binding and dependency
//! cycles have been checked:
//! - `nominal` at `Nominal` writes at least one modifier list (`pre` or `post`)
//!   and never wraps a bare `nominal` directly as its `head` (coordinations of
//!   `nominal`s are allowed);
//! - `stacked` at `PreModifier` stands only as a direct arm of a `PreModifier`
//!   coordination (`call` already refuses `< 2` elements via `WrittenForms`);
//! - an Agreement `PostModifier` (`PostModifier = Agree`) stands only inside a
//!   `Nominal` consumed by a determiner or slice that forms a plural group.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use sylex_syl::Arg;
use sylex_syl::Call;
use sylex_syl::Item;
use sylex_syl::Shape;
use sylex_syl::Span;
use sylex_syl::Value;
use sylex_syl::ValueKind;

use super::Decl;
use super::Refusal;
use super::RefusalKind;
use super::Registry;
use super::Resolutions;
use super::schema;
use super::sort::SortKey;

pub(super) fn check(
    decls: &[Decl<'_>],
    registry: &Registry,
    resolutions: &Resolutions,
    acyclic: bool,
    refusals: &mut Vec<Refusal>,
) {
    let checker = NominalChecker {
        decls,
        registry,
        resolutions,
    };
    for decl in decls {
        checker.check_syntax(decl, refusals);
    }
    if acyclic && checker.has_post_modifier_agree() {
        for decl in decls {
            checker.check_agreement(decl, refusals);
        }
    }
}

struct NominalChecker<'a, 'd> {
    decls: &'d [Decl<'a>],
    registry: &'d Registry,
    resolutions: &'d Resolutions,
}

impl<'a> NominalChecker<'a, '_> {
    fn has_post_modifier_agree(&self) -> bool {
        self.decls
            .iter()
            .any(|d| Self::is_primitive_ctor(d, "PostModifier", "Agree"))
    }

    fn is_primitive_ctor(decl: &Decl<'_>, sort: &str, ctor: &str) -> bool {
        if decl.kind.word != schema::PRIMITIVE
            || !decl.sort.args.is_empty()
            || decl.sort.head != sort
        {
            return false;
        }
        let Shape::Header(header) = &decl.declaration.shape else {
            return false;
        };
        header
            .constructor
            .is_some_and(|ident| ident.ident.as_str() == ctor)
    }

    /// Whether `call` resolved to a `PreModifier` coordination (`coord`, or a
    /// non-`stacked` `PreModifier` Template expanding to `coord`, such as `or`,
    /// `and` or `andOr`).
    fn is_pre_modifier_coordination(&self, call: &Call) -> bool {
        if call.head.ident == "stacked" {
            return false;
        }
        let Some(sort) = self.resolutions.call(call) else {
            return false;
        };
        if !sort.args.is_empty() || sort.head != "PreModifier" {
            return false;
        }
        let Some(sig) = self
            .registry
            .get(&(call.head.ident.to_string(), sort.clone()))
        else {
            return false;
        };
        if sig.secondary {
            return false;
        }
        let mut cur = sig.decl;
        let mut visited = BTreeSet::new();
        while visited.insert(cur) {
            let decl = &self.decls[cur];
            if Self::is_primitive_ctor(decl, "PreModifier", "Coordination") {
                return true;
            }
            if decl.kind.word != "template" || decl.name.as_deref() == Some("stacked") {
                return false;
            }
            let Some(body) = decl.field("body") else {
                return false;
            };
            let ValueKind::Call(inner) = &body.kind else {
                return false;
            };
            let Some(inner_sort) = self.resolutions.call(inner) else {
                return false;
            };
            let Some(inner_sig) = self
                .registry
                .get(&(inner.head.ident.to_string(), inner_sort.clone()))
            else {
                return false;
            };
            if inner_sig.secondary {
                return false;
            }
            cur = inner_sig.decl;
        }
        false
    }

    fn check_syntax(&self, decl: &Decl<'a>, refusals: &mut Vec<Refusal>) {
        match &decl.declaration.shape {
            Shape::Value(value) => self.walk_syntax(value, false, refusals),
            Shape::Header(header) => {
                for param in header.params.iter().filter_map(Item::node) {
                    if let Some(default) = &param.default {
                        self.walk_syntax(default, false, refusals);
                    }
                }
                if let Some(block) = &header.body {
                    for field in block.nodes() {
                        if !sylex_syl::METADATA_FIELDS.contains(&field.name.ident.as_str()) {
                            self.walk_syntax(&field.value, false, refusals);
                        }
                    }
                }
            }
        }
    }

    fn walk_syntax(&self, value: &Value, in_pre_coord_arm: bool, refusals: &mut Vec<Refusal>) {
        match &value.kind {
            ValueKind::List(items) => {
                for item in items.iter().filter_map(Item::node) {
                    self.walk_syntax(item, false, refusals);
                }
            }
            ValueKind::Record(record) => {
                for field in record.nodes() {
                    self.walk_syntax(&field.value, false, refusals);
                }
            }
            ValueKind::Call(call) => {
                let resolved_sort = self.resolutions.call(call);
                let coerced = self.resolutions.coerced(value).is_some();

                if !coerced
                    && let Some(sort) = resolved_sort
                    && sort.args.is_empty()
                {
                    if sort.head == "Nominal" && call.head.ident == "nominal" {
                        self.check_nominal_call(call, sort, refusals);
                    } else if sort.head == "PreModifier"
                        && call.head.ident == "stacked"
                        && !in_pre_coord_arm
                    {
                        refusals.push(Refusal {
                            span: call.span,
                            kind: RefusalKind::UnplacedStacked,
                        });
                    }
                }

                let is_pre_coord = !coerced && self.is_pre_modifier_coordination(call);
                for arg in call.args.iter().filter_map(Item::node) {
                    if is_pre_coord && let ValueKind::List(items) = &arg.value.kind {
                        for item in items.iter().filter_map(Item::node) {
                            self.walk_syntax(item, true, refusals);
                        }
                    } else {
                        self.walk_syntax(&arg.value, false, refusals);
                    }
                }
            }
            ValueKind::String(_) | ValueKind::Integer(_) | ValueKind::Parameter(_) => {}
        }
    }

    fn check_nominal_call(&self, call: &Call, sort: &SortKey, refusals: &mut Vec<Refusal>) {
        let Some(sig) = self
            .registry
            .get(&(call.head.ident.to_string(), sort.clone()))
        else {
            return;
        };
        let mut has_pre_or_post = false;
        let mut head_arg: Option<&Arg> = None;
        for arg in call.args.iter().filter_map(Item::node) {
            let Some(idx) = self.resolutions.arg(arg) else {
                // Incomplete/invalid call already refused by `Resolver::call`.
                return;
            };
            match sig.params[idx].name.as_str() {
                "pre" | "post" => {
                    let is_empty_list = matches!(
                        &arg.value.kind,
                        ValueKind::List(items) if !items.iter().any(|i| i.node().is_some())
                    );
                    if !is_empty_list {
                        has_pre_or_post = true;
                    }
                }
                "head" => head_arg = Some(arg),
                _ => {}
            }
        }
        let Some(head_arg) = head_arg else {
            return;
        };
        if !has_pre_or_post {
            refusals.push(Refusal {
                span: call.span,
                kind: RefusalKind::RedundantNominal,
            });
        }
        if self.resolutions.coerced(&head_arg.value).is_none()
            && let ValueKind::Call(inner) = &head_arg.value.kind
            && let Some(inner_sort) = self.resolutions.call(inner)
            && inner_sort.args.is_empty()
            && inner_sort.head == "Nominal"
            && inner.head.ident == "nominal"
        {
            refusals.push(Refusal {
                span: inner.span,
                kind: RefusalKind::NestedNominal,
            });
        }
    }

    fn check_agreement(&self, decl: &Decl<'a>, refusals: &mut Vec<Refusal>) {
        if matches!(decl.kind.word, schema::PRIMITIVE | schema::COERCION) {
            return;
        }
        let mut env: BTreeMap<String, Term<'a>> = BTreeMap::new();
        for param in &decl.params {
            env.insert(param.name.clone(), Term::Param);
        }
        for (implicit, _) in decl.kind.implicit {
            env.insert((*implicit).to_owned(), Term::Param);
        }
        match &decl.declaration.shape {
            Shape::Value(value) => {
                if let Some(term) = self.elaborate(value, &decl.sort, &env, None, refusals) {
                    self.walk_agreement(&term, refusals);
                }
            }
            Shape::Header(header) => {
                for (index, param) in header.params.iter().filter_map(Item::node).enumerate() {
                    if let Some(default) = &param.default
                        && let Some(term) =
                            self.elaborate(default, &decl.params[index].sort, &env, None, refusals)
                    {
                        self.walk_agreement(&term, refusals);
                    }
                }
                let Some(block) = &header.body else {
                    return;
                };
                for field in block.nodes() {
                    let name = field.name.ident.as_str();
                    if sylex_syl::METADATA_FIELDS.contains(&name) {
                        continue;
                    }
                    let Some(spec) = decl.kind.fields.iter().find(|s| s.name == name) else {
                        continue;
                    };
                    let sort = spec.sort.map_or_else(|| decl.sort.clone(), SortKey::table);
                    if let Some(term) = self.elaborate(&field.value, &sort, &env, None, refusals) {
                        self.walk_agreement(&term, refusals);
                    }
                }
            }
        }
    }

    fn elaborate(
        &self,
        value: &'a Value,
        expected: &SortKey,
        env: &BTreeMap<String, Term<'a>>,
        span_override: Option<Span>,
        refusals: &mut Vec<Refusal>,
    ) -> Option<Term<'a>> {
        let span = span_override.unwrap_or(value.span);
        let term = match &value.kind {
            ValueKind::Parameter(name) => env.get(name.ident.as_str())?.clone(),
            ValueKind::Integer(n) => Term::Int(*n),
            ValueKind::String(_) => Term::Other,
            ValueKind::List(items) => {
                let inner = expected.option_inner().unwrap_or(expected).list_inner()?;
                let mut out = Vec::new();
                for item in items.iter().filter_map(Item::node) {
                    out.push(self.elaborate(item, inner, env, span_override, refusals)?);
                }
                Term::List(out)
            }
            ValueKind::Record(record) => {
                let base = expected.option_inner().unwrap_or(expected);
                let spec = schema::RECORDS
                    .iter()
                    .find(|s| base.args.is_empty() && s.sort == base.head)?;
                let mut fields = Vec::new();
                for field in record.nodes() {
                    let fname = field.name.ident.as_str();
                    if sylex_syl::METADATA_FIELDS.contains(&fname) {
                        continue;
                    }
                    let (_, fsort, _) = spec.fields.iter().find(|(n, _, _)| *n == fname)?;
                    let fterm = self.elaborate(
                        &field.value,
                        &SortKey::table(fsort),
                        env,
                        span_override,
                        refusals,
                    )?;
                    fields.push((fname, fterm));
                }
                Term::Record {
                    sort: base.clone(),
                    fields,
                }
            }
            ValueKind::Call(call) => {
                if call.head.ident == "none" && call.is_bare() {
                    return Some(Term::None);
                }
                let call_sort = self.resolutions.call(call)?;
                let sig = self
                    .registry
                    .get(&(call.head.ident.to_string(), call_sort.clone()))?;
                if sig.secondary {
                    return Some(Term::Other);
                }
                let decl = &self.decls[sig.decl];
                let mut written: Vec<Option<&'a Value>> = vec![None; decl.params.len()];
                for arg in call.args.iter().filter_map(Item::node) {
                    let idx = self.resolutions.arg(arg)?;
                    written[idx] = Some(&arg.value);
                }
                let Shape::Header(header) = &decl.declaration.shape else {
                    return None;
                };
                let header_params: Vec<_> = header.params.iter().filter_map(Item::node).collect();
                let mut callee_env: BTreeMap<String, Term<'a>> = BTreeMap::new();
                let mut bound_args: Vec<(String, SortKey, Term<'a>)> =
                    Vec::with_capacity(decl.params.len());
                let call_span = span_override.unwrap_or(call.span);
                for ((hparam, slot), arg_val) in
                    header_params.into_iter().zip(&decl.params).zip(written)
                {
                    let arg_term = match (arg_val, &hparam.default) {
                        (Some(val), _) => {
                            self.elaborate(val, &slot.sort, env, span_override, refusals)?
                        }
                        (None, Some(def)) => {
                            self.elaborate(def, &slot.sort, &callee_env, Some(call_span), refusals)?
                        }
                        (None, None) => return None,
                    };
                    callee_env.insert(slot.name.clone(), arg_term.clone());
                    bound_args.push((slot.name.clone(), slot.sort.clone(), arg_term));
                }

                // `forEach` is distributive singular ("for each card"), even though
                // its body expands through `countOf` (`described(all, ...)`).
                if decl.kind.word == "template" && decl.name.as_deref() == Some("forEach") {
                    let mut refused_here = false;
                    for (_, psort, aterm) in &bound_args {
                        if Self::is_nominal_sort(psort)
                            && let Some(bad_span) = self.nominal_agree_span(aterm)
                        {
                            refusals.push(Refusal {
                                span: bad_span,
                                kind: RefusalKind::AgreementWithoutGroup,
                            });
                            refused_here = true;
                        }
                    }
                    if refused_here {
                        return Some(Term::Other);
                    }
                }

                if decl.kind.word == "template" {
                    let body = decl.field("body")?;
                    self.elaborate(body, &decl.sort, &callee_env, Some(call_span), refusals)?
                } else {
                    Term::Call {
                        decl: sig.decl,
                        span: call_span,
                        args: bound_args,
                    }
                }
            }
        };

        if let Some(coercion_idx) = self.resolutions.coerced(value) {
            let coercion_decl = &self.decls[coercion_idx];
            let [param] = coercion_decl.params.as_slice() else {
                return None;
            };
            let body = coercion_decl.field("body")?;
            let mut cenv = BTreeMap::new();
            cenv.insert(param.name.clone(), term);
            return self.elaborate(body, &coercion_decl.sort, &cenv, Some(span), refusals);
        }
        Some(term)
    }

    fn is_nominal_sort(sort: &SortKey) -> bool {
        let inner = sort.option_inner().unwrap_or(sort);
        let elem = inner.list_inner().unwrap_or(inner);
        elem.args.is_empty() && elem.head == "Nominal"
    }

    fn walk_agreement(&self, term: &Term<'a>, refusals: &mut Vec<Refusal>) {
        match term {
            Term::List(items) => {
                for item in items {
                    self.walk_agreement(item, refusals);
                }
            }
            Term::Record { sort, fields, .. } => {
                let spec = schema::RECORDS
                    .iter()
                    .find(|s| sort.args.is_empty() && s.sort == sort.head);
                for (fname, fterm) in fields {
                    if let Some(spec) = spec
                        && let Some((_, fsort, _)) = spec.fields.iter().find(|(n, _, _)| n == fname)
                        && Self::is_nominal_sort(&SortKey::table(fsort))
                        && let Some(bad_span) = self.nominal_agree_span(fterm)
                    {
                        refusals.push(Refusal {
                            span: bad_span,
                            kind: RefusalKind::AgreementWithoutGroup,
                        });
                    }
                    self.walk_agreement(fterm, refusals);
                }
            }
            Term::Call {
                decl: decl_idx,
                args,
                ..
            } => {
                let decl = &self.decls[*decl_idx];
                if Self::is_primitive_ctor(decl, "Nominal", "Nominal")
                    || Self::is_primitive_ctor(decl, "Nominal", "Coordination")
                {
                    // Nominal builders propagate Agreement to their enclosing consumer.
                    for (_, _, aterm) in args {
                        self.walk_agreement(aterm, refusals);
                    }
                    return;
                }
                if Self::is_primitive_ctor(decl, "NounPhrase", "Described") {
                    if let [(_, _, det), (_, _, nom)] = args.as_slice()
                        && !self.determiner_forms_group(det)
                        && let Some(bad_span) = self.nominal_agree_span(nom)
                    {
                        refusals.push(Refusal {
                            span: bad_span,
                            kind: RefusalKind::AgreementWithoutGroup,
                        });
                    }
                    for (_, _, aterm) in args {
                        self.walk_agreement(aterm, refusals);
                    }
                    return;
                }
                if Self::is_primitive_ctor(decl, "NounPhrase", "SomeOf") {
                    let count = args
                        .iter()
                        .find(|(n, _, _)| n == "count")
                        .map(|(_, _, t)| t);
                    let desc = args
                        .iter()
                        .find(|(n, _, _)| n == "description")
                        .map(|(_, _, t)| t);
                    if let (Some(count), Some(desc)) = (count, desc)
                        && !self.slice_count_forms_group(count)
                        && let Some(bad_span) = self.nominal_agree_span(desc)
                    {
                        refusals.push(Refusal {
                            span: bad_span,
                            kind: RefusalKind::AgreementWithoutGroup,
                        });
                    }
                    for (_, _, aterm) in args {
                        self.walk_agreement(aterm, refusals);
                    }
                    return;
                }

                // Every other consumer of `Nominal` tests individual members or names
                // a single head noun, forming no group for Agreement.
                for (_, psort, aterm) in args {
                    if Self::is_nominal_sort(psort)
                        && let Some(bad_span) = self.nominal_agree_span(aterm)
                    {
                        refusals.push(Refusal {
                            span: bad_span,
                            kind: RefusalKind::AgreementWithoutGroup,
                        });
                    }
                    self.walk_agreement(aterm, refusals);
                }
            }
            Term::Int(_) | Term::Param | Term::None | Term::Other => {}
        }
    }

    /// Returns the span of the first `PostModifier = Agree` inside `term`'s own
    /// `Nominal` structure (without crossing into nested `NounPhrase` or
    /// sub-domain arguments).
    fn nominal_agree_span(&self, term: &Term<'a>) -> Option<Span> {
        match term {
            Term::List(items) => items.iter().find_map(|it| self.nominal_agree_span(it)),
            Term::Call {
                decl: decl_idx,
                args,
                ..
            } => {
                let decl = &self.decls[*decl_idx];
                if Self::is_primitive_ctor(decl, "Nominal", "Nominal") {
                    for (pname, _, aterm) in args {
                        if pname == "head"
                            && let Some(sp) = self.nominal_agree_span(aterm)
                        {
                            return Some(sp);
                        }
                        if pname == "post"
                            && let Some(sp) = self.post_modifier_agree_span(aterm)
                        {
                            return Some(sp);
                        }
                    }
                    None
                } else if Self::is_primitive_ctor(decl, "Nominal", "Coordination") {
                    args.iter()
                        .find_map(|(_, _, aterm)| self.nominal_agree_span(aterm))
                } else {
                    None
                }
            }
            Term::Record { .. } | Term::Int(_) | Term::Param | Term::None | Term::Other => None,
        }
    }

    fn post_modifier_agree_span(&self, term: &Term<'a>) -> Option<Span> {
        match term {
            Term::List(items) => items
                .iter()
                .find_map(|it| self.post_modifier_agree_span(it)),
            Term::Call {
                decl: decl_idx,
                span,
                args,
                ..
            } => {
                let decl = &self.decls[*decl_idx];
                if Self::is_primitive_ctor(decl, "PostModifier", "Agree") {
                    Some(*span)
                } else if Self::is_primitive_ctor(decl, "PostModifier", "Coordination")
                    || Self::is_primitive_ctor(decl, "PostModifier", "Not")
                {
                    args.iter()
                        .find_map(|(_, _, aterm)| self.post_modifier_agree_span(aterm))
                } else {
                    None
                }
            }
            Term::Record { .. } | Term::Int(_) | Term::Param | Term::None | Term::Other => None,
        }
    }

    fn determiner_forms_group(&self, det: &Term<'a>) -> bool {
        match det {
            Term::Param => true,
            Term::Call {
                decl: decl_idx,
                args,
                ..
            } => {
                let decl = &self.decls[*decl_idx];
                if Self::is_primitive_ctor(decl, "DetPhrase", "All") {
                    true
                } else if Self::is_primitive_ctor(decl, "DetPhrase", "The") {
                    args.iter()
                        .find(|(n, _, _)| n == "plur")
                        .is_some_and(|(_, _, plur)| self.plurality_forms_group(plur))
                } else if Self::is_primitive_ctor(decl, "DetPhrase", "Count")
                    || Self::is_primitive_ctor(decl, "DetPhrase", "Target")
                {
                    args.iter()
                        .find(|(n, _, _)| n == "quantity")
                        .is_some_and(|(_, _, q)| !self.quantity_is_at_most_one(q))
                } else {
                    // `DetPhrase.A`, `DetPhrase.Each`, or non-group determiner.
                    false
                }
            }
            _ => false,
        }
    }

    fn slice_count_forms_group(&self, count: &Term<'a>) -> bool {
        match count {
            Term::Param => true,
            Term::Call {
                decl: decl_idx,
                args,
                ..
            } => {
                let decl = &self.decls[*decl_idx];
                if Self::is_primitive_ctor(decl, "SliceCount", "Whole")
                    || Self::is_primitive_ctor(decl, "SliceCount", "Totaling")
                {
                    true
                } else if Self::is_primitive_ctor(decl, "SliceCount", "Counted") {
                    args.iter()
                        .find(|(n, _, _)| n == "quantity")
                        .is_some_and(|(_, _, q)| !self.quantity_is_at_most_one(q))
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn plurality_forms_group(&self, plur: &Term<'a>) -> bool {
        match plur {
            Term::Param => true,
            Term::Call { decl: decl_idx, .. } => {
                Self::is_primitive_ctor(&self.decls[*decl_idx], "Plurality", "Many")
            }
            _ => false,
        }
    }

    fn quantity_is_at_most_one(&self, q: &Term<'a>) -> bool {
        let Term::Call {
            decl: decl_idx,
            args,
            ..
        } = q
        else {
            return false;
        };
        let decl = &self.decls[*decl_idx];
        if Self::is_primitive_ctor(decl, "Quantity", "Range") {
            let low = args.iter().find(|(n, _, _)| n == "low").map(|(_, _, t)| t);
            let high = args.iter().find(|(n, _, _)| n == "high").map(|(_, _, t)| t);
            matches!(high, Some(Term::Int(0 | 1)))
                && matches!(low, Some(Term::None | Term::Int(0 | 1)))
        } else if Self::is_primitive_ctor(decl, "Quantity", "ExactlyOf") {
            args.iter()
                .find(|(n, _, _)| n == "amount")
                .is_some_and(|(_, _, amt)| self.amount_is_at_most_one(amt))
        } else {
            false
        }
    }

    fn amount_is_at_most_one(&self, amt: &Term<'a>) -> bool {
        match amt {
            Term::Int(n) => *n <= 1,
            Term::Call {
                decl: decl_idx,
                args,
                ..
            } => {
                Self::is_primitive_ctor(&self.decls[*decl_idx], "Amount", "Literal")
                    && args
                        .first()
                        .is_some_and(|(_, _, v)| matches!(v, Term::Int(n) if *n <= 1))
            }
            _ => false,
        }
    }
}

#[derive(Clone)]
enum Term<'a> {
    Call {
        decl: usize,
        span: Span,
        args: Vec<(String, SortKey, Term<'a>)>,
    },
    List(Vec<Term<'a>>),
    Record {
        sort: SortKey,
        fields: Vec<(&'a str, Term<'a>)>,
    },
    Int(i64),
    Param,
    None,
    Other,
}

//! Structural rules on `Nominal`, `PreModifier` and `PostModifier` terms
//! (`docs/design/spelling.md` §2, `docs/design/semantics.md` §2.4).
//!
//! Runs only on bundles every earlier rule accepted, so every call, argument
//! and coercion it reads is resolved; a missing resolution is a defect in this
//! validator and panics. Checks four invariants:
//! - `nominal` writes at least one modifier list (`pre` or `post`), read on the
//!   written call;
//! - a nominal has one tree: no `Nominal = Nominal` stands as the `head` of
//!   another once Templates and their arguments are expanded (a coordination of
//!   nominals may);
//! - a `PreModifier` `and` coordination (`stacked`) stands only as an item of a
//!   `PreModifier` `or` or `and/or` coordination; a Template's own body is
//!   checked where the Template is called;
//! - an Agreement `PostModifier` (`PostModifier = Agree`) stands only inside a
//!   `Nominal` consumed by a determiner or slice that forms a plural group.
//!
//! The last three read each declaration's values with every Template call
//! replaced by its body, its parameters bound to the call's arguments, and
//! with each call of a Definition that has parameters checking that
//! Definition's body with the call's arguments. A defect a body has whatever
//! its arguments is refused at the declaration and again at each call.

use std::collections::BTreeMap;

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
use super::Signature;
use super::schema;
use super::sort::SortKey;

/// Checks every declaration. Call only when every earlier rule accepted the
/// bundles.
pub(super) fn check(
    decls: &[Decl<'_>],
    registry: &Registry,
    resolutions: &Resolutions,
    refusals: &mut Vec<Refusal>,
) {
    let checker = NominalChecker {
        decls,
        registry,
        resolutions,
    };
    for decl in decls {
        checker.check_written(decl, refusals);
        checker.check_expanded(decl, refusals);
    }
}

struct NominalChecker<'a, 'd> {
    decls: &'d [Decl<'a>],
    registry: &'d Registry,
    resolutions: &'d Resolutions,
}

/// Where an expanded term stands, for the `stacked` rule.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Place {
    /// A direct item of a `PreModifier` `or` or `and/or` coordination.
    DisjunctionItem,
    /// The root of a Template's own `body`, or an item of a coordination whose
    /// coordinator is a declaration's parameter: checked where the declaration
    /// is called.
    Deferred,
    /// Anywhere else.
    Elsewhere,
}

/// Whether a determiner, slice count, plurality or quantity lets its
/// `Nominal` stand for more than one member.
#[derive(Clone, Copy)]
enum Number {
    AtMostOne,
    Group,
    /// A declaration's parameter: the term is checked again with the argument
    /// at each call of the declaration, a Template's or a Definition's.
    Parameter,
}

impl Number {
    /// Whether Agreement may stand under it here.
    fn forms_group(self) -> bool {
        match self {
            Number::Group | Number::Parameter => true,
            Number::AtMostOne => false,
        }
    }
}

/// A call's bound arguments: each parameter's name, sort and term.
type Args<'a> = [(String, SortKey, Term<'a>)];

impl<'a> NominalChecker<'a, '_> {
    /// The signature `call` resolved to.
    fn signature(&self, call: &Call) -> &Signature {
        let sort = self
            .resolutions
            .call(call)
            .unwrap_or_else(|| panic!("`{}` is resolved", call.head.ident));
        self.registry
            .get(&(call.head.ident.to_string(), sort.clone()))
            .unwrap_or_else(|| panic!("`{}` at {sort} is registered", call.head.ident))
    }

    /// The `(sort, constructor)` a primitive with no sort arguments builds.
    fn constructor(&self, decl: usize) -> Option<(&str, &str)> {
        let decl = &self.decls[decl];
        if decl.kind.word != schema::PRIMITIVE || !decl.sort.args.is_empty() {
            return None;
        }
        let Shape::Header(header) = &decl.declaration.shape else {
            panic!("a primitive is written as a header");
        };
        header
            .constructor
            .as_ref()
            .map(|ident| (decl.sort.head.as_str(), ident.ident.as_str()))
    }

    fn builds(&self, decl: usize, sort: &str, ctor: &str) -> bool {
        self.constructor(decl) == Some((sort, ctor))
    }

    // ---- The written call: `nominal` with a modifier list ----

    fn check_written(&self, decl: &Decl<'a>, refusals: &mut Vec<Refusal>) {
        for (value, _) in decl_values(decl) {
            self.walk_written(value, refusals);
        }
    }

    fn walk_written(&self, value: &Value, refusals: &mut Vec<Refusal>) {
        match &value.kind {
            ValueKind::List(items) => {
                for item in items.iter().filter_map(Item::node) {
                    self.walk_written(item, refusals);
                }
            }
            ValueKind::Record(record) => {
                for field in record.nodes() {
                    self.walk_written(&field.value, refusals);
                }
            }
            ValueKind::Call(call) => {
                if call.head.ident != "none" || !call.is_bare() {
                    let sig = self.signature(call);
                    if self.resolutions.coerced(value).is_none()
                        && !sig.secondary
                        && self.builds(sig.decl, "Nominal", "Nominal")
                        && !self.writes_a_modifier(call, sig)
                    {
                        refusals.push(Refusal {
                            span: call.span,
                            kind: RefusalKind::RedundantNominal,
                        });
                    }
                }
                for arg in call.args.iter().filter_map(Item::node) {
                    self.walk_written(&arg.value, refusals);
                }
            }
            ValueKind::String(_) | ValueKind::Integer(_) | ValueKind::Parameter(_) => {}
        }
    }

    /// Whether a `nominal` call writes a `pre` or `post` argument other than
    /// an empty list.
    fn writes_a_modifier(&self, call: &Call, sig: &Signature) -> bool {
        call.args.iter().filter_map(Item::node).any(|arg| {
            let index = self.resolutions.arg(arg).expect("every argument is bound");
            let is_list = matches!(sig.params[index].name.as_str(), "pre" | "post");
            let is_empty = matches!(
                &arg.value.kind,
                ValueKind::List(items) if !items.iter().any(|i| i.node().is_some())
            );
            is_list && !is_empty
        })
    }

    // ---- The expanded term: one tree, `stacked`, Agreement ----

    fn check_expanded(&self, decl: &Decl<'a>, refusals: &mut Vec<Refusal>) {
        let mut env: BTreeMap<String, Term<'a>> = BTreeMap::new();
        for param in &decl.params {
            env.insert(param.name.clone(), Term::Param);
        }
        for (implicit, _) in decl.kind.implicit {
            env.insert((*implicit).to_owned(), Term::Param);
        }
        for (value, slot) in decl_values(decl) {
            let sort = match slot {
                ValueSlot::Declaration => decl.sort.clone(),
                ValueSlot::Default(index) => decl.params[index].sort.clone(),
                ValueSlot::Field(name) => {
                    let spec = decl
                        .kind
                        .fields
                        .iter()
                        .find(|spec| spec.name == name)
                        .unwrap_or_else(|| {
                            panic!("field `{name}` of {} is declared", decl.label())
                        });
                    spec.sort.map_or_else(|| decl.sort.clone(), SortKey::table)
                }
            };
            let place =
                if decl.kind.word == schema::TEMPLATE && matches!(slot, ValueSlot::Field("body")) {
                    Place::Deferred
                } else {
                    Place::Elsewhere
                };
            let term = self.elaborate(value, &sort, &env, None, refusals);
            self.walk(&term, place, refusals);
        }
    }

    /// `value` at `expected`, every Template call replaced by its body.
    fn elaborate(
        &self,
        value: &'a Value,
        expected: &SortKey,
        env: &BTreeMap<String, Term<'a>>,
        span_override: Option<Span>,
        refusals: &mut Vec<Refusal>,
    ) -> Term<'a> {
        let span = span_override.unwrap_or(value.span);
        let term = match &value.kind {
            ValueKind::Parameter(name) => env
                .get(name.ident.as_str())
                .unwrap_or_else(|| panic!("`${}` is a parameter in scope", name.ident))
                .clone(),
            ValueKind::Integer(n) => Term::Int(*n),
            ValueKind::String(_) => Term::Other,
            ValueKind::List(items) => {
                let inner = expected
                    .option_inner()
                    .unwrap_or(expected)
                    .list_inner()
                    .unwrap_or_else(|| panic!("a list stands at a List sort, not {expected}"));
                Term::List(
                    items
                        .iter()
                        .filter_map(Item::node)
                        .map(|item| self.elaborate(item, inner, env, span_override, refusals))
                        .collect(),
                )
            }
            ValueKind::Record(record) => {
                let base = expected.option_inner().unwrap_or(expected);
                let spec = schema::RECORDS
                    .iter()
                    .find(|s| base.args.is_empty() && s.sort == base.head)
                    .unwrap_or_else(|| panic!("a record stands at a record sort, not {base}"));
                let mut fields = Vec::new();
                for field in record.nodes() {
                    let fname = field.name.ident.as_str();
                    if sylex_syl::METADATA_FIELDS.contains(&fname) {
                        continue;
                    }
                    let (_, fsort, _) = spec
                        .fields
                        .iter()
                        .find(|(n, _, _)| *n == fname)
                        .unwrap_or_else(|| panic!("`{fname}` is a field of {base}"));
                    let fterm = self.elaborate(
                        &field.value,
                        &SortKey::table(fsort),
                        env,
                        span_override,
                        refusals,
                    );
                    fields.push((fname, fterm));
                }
                Term::Record {
                    sort: base.clone(),
                    fields,
                }
            }
            ValueKind::Call(call) => {
                if call.head.ident == "none" && call.is_bare() {
                    return Term::None;
                }
                self.elaborate_call(call, env, span_override, refusals)
            }
        };

        if let Some(coercion_idx) = self.resolutions.coerced(value) {
            let coercion_decl = &self.decls[coercion_idx];
            let [param] = coercion_decl.params.as_slice() else {
                panic!("a coercion takes one parameter");
            };
            let body = coercion_decl.field("body").expect("a coercion has a body");
            let mut cenv = BTreeMap::new();
            cenv.insert(param.name.clone(), term);
            return self.elaborate(body, &coercion_decl.sort, &cenv, Some(span), refusals);
        }
        term
    }

    fn elaborate_call(
        &self,
        call: &'a Call,
        env: &BTreeMap<String, Term<'a>>,
        span_override: Option<Span>,
        refusals: &mut Vec<Refusal>,
    ) -> Term<'a> {
        let sig = self.signature(call);
        if sig.secondary {
            return Term::Other;
        }
        let decl = &self.decls[sig.decl];
        let mut written: Vec<Option<&'a Value>> = vec![None; decl.params.len()];
        for arg in call.args.iter().filter_map(Item::node) {
            let index = self.resolutions.arg(arg).expect("every argument is bound");
            written[index] = Some(&arg.value);
        }
        let Shape::Header(header) = &decl.declaration.shape else {
            panic!("a called declaration is written as a header");
        };
        let call_span = span_override.unwrap_or(call.span);
        let mut callee_env: BTreeMap<String, Term<'a>> = BTreeMap::new();
        let mut bound_args: Vec<(String, SortKey, Term<'a>)> =
            Vec::with_capacity(decl.params.len());
        for ((hparam, slot), arg_val) in header
            .params
            .iter()
            .filter_map(Item::node)
            .zip(&decl.params)
            .zip(written)
        {
            let arg_term = match (arg_val, &hparam.default) {
                (Some(val), _) => self.elaborate(val, &slot.sort, env, span_override, refusals),
                (None, Some(def)) => {
                    self.elaborate(def, &slot.sort, &callee_env, Some(call_span), refusals)
                }
                (None, None) => panic!("required argument `{}` is written", slot.name),
            };
            callee_env.insert(slot.name.clone(), arg_term.clone());
            bound_args.push((slot.name.clone(), slot.sort.clone(), arg_term));
        }

        if decl.kind.word == schema::TEMPLATE {
            let body = decl.field("body").expect("a Template has a body");
            return self.elaborate(body, &decl.sort, &callee_env, Some(call_span), refusals);
        }
        // A Definition (`action`, `ability`, …) stays one call, but its body is
        // checked here too with this call's arguments, as a Template's is: a
        // parameter its body leaves open is settled only at the call.
        let body = match (decl.params.is_empty(), decl.field("body")) {
            (false, Some(body)) => {
                let mut body_env = callee_env.clone();
                for (implicit, _) in decl.kind.implicit {
                    body_env.insert((*implicit).to_owned(), Term::Param);
                }
                let sort = decl
                    .kind
                    .fields
                    .iter()
                    .find(|spec| spec.name == "body")
                    .and_then(|spec| spec.sort)
                    .map_or_else(|| decl.sort.clone(), SortKey::table);
                Some(Box::new(self.elaborate(
                    body,
                    &sort,
                    &body_env,
                    Some(call_span),
                    refusals,
                )))
            }
            (true, _) | (false, None) => None,
        };
        Term::Call {
            decl: sig.decl,
            span: call_span,
            args: bound_args,
            body,
        }
    }

    fn walk(&self, term: &Term<'a>, place: Place, refusals: &mut Vec<Refusal>) {
        match term {
            Term::List(items) => {
                for item in items {
                    self.walk(item, place, refusals);
                }
            }
            Term::Record { sort, fields } => {
                let spec = schema::RECORDS
                    .iter()
                    .find(|s| sort.args.is_empty() && s.sort == sort.head)
                    .unwrap_or_else(|| panic!("{sort} is a record sort"));
                for (fname, fterm) in fields {
                    let (_, fsort, _) = spec
                        .fields
                        .iter()
                        .find(|(n, _, _)| n == fname)
                        .unwrap_or_else(|| panic!("`{fname}` is a field of {sort}"));
                    if is_nominal_sort(&SortKey::table(fsort)) {
                        self.refuse_agreement_in(fterm, refusals);
                    }
                    self.walk(fterm, Place::Elsewhere, refusals);
                }
            }
            Term::Call {
                decl,
                span,
                args,
                body,
            } => {
                self.walk_call(*decl, *span, args, place, refusals);
                if let Some(body) = body {
                    self.walk(body, place, refusals);
                }
            }
            Term::Int(_) | Term::Param | Term::None | Term::Other => {}
        }
    }

    fn walk_call(
        &self,
        decl: usize,
        span: Span,
        args: &Args<'a>,
        place: Place,
        refusals: &mut Vec<Refusal>,
    ) {
        let arg = |name: &str| named(args, name);
        match self.constructor(decl) {
            Some(("Nominal", "Nominal")) => {
                if let Term::Call {
                    decl: head, span, ..
                } = arg("head")
                    && self.builds(*head, "Nominal", "Nominal")
                {
                    refusals.push(Refusal {
                        span: *span,
                        kind: RefusalKind::NestedNominal,
                    });
                }
                // A nominal hands Agreement to its enclosing consumer.
                for (_, _, aterm) in args {
                    self.walk(aterm, Place::Elsewhere, refusals);
                }
            }
            Some(("PreModifier", "Coordination")) => {
                let item_place = match arg("coordinator") {
                    Term::Param => Place::Deferred,
                    Term::Call {
                        decl: coordinator,
                        span: coordinator_span,
                        ..
                    } => match self.constructor(*coordinator) {
                        Some(("Coordinator", "And")) => {
                            if place == Place::Elsewhere {
                                refusals.push(Refusal {
                                    span,
                                    kind: RefusalKind::UnplacedStacked,
                                });
                            }
                            Place::Elsewhere
                        }
                        Some(("Coordinator", "Or" | "AndOr")) => Place::DisjunctionItem,
                        _ => {
                            refusals.push(Refusal {
                                span: *coordinator_span,
                                kind: RefusalKind::UnclassifiedCoordinator(self.word(*coordinator)),
                            });
                            Place::Elsewhere
                        }
                    },
                    other => panic!("a Coordinator is a call or a parameter, not {other:?}"),
                };
                self.walk(arg("items"), item_place, refusals);
            }
            Some(("NounPhrase", "Described")) => {
                let nominal = arg("nominal");
                if !self
                    .determiner_number(arg("determiner"), refusals)
                    .forms_group()
                {
                    self.refuse_agreement_in(nominal, refusals);
                }
                for (_, _, aterm) in args {
                    self.walk(aterm, Place::Elsewhere, refusals);
                }
            }
            Some(("NounPhrase", "SomeOf")) => {
                if !self
                    .slice_count_number(arg("count"), refusals)
                    .forms_group()
                {
                    self.refuse_agreement_in(arg("description"), refusals);
                }
                for (_, _, aterm) in args {
                    self.walk(aterm, Place::Elsewhere, refusals);
                }
            }
            // A coordination of nominals hands Agreement to its enclosing
            // consumer. An aggregate, a superlative or a comparison ranges over
            // its whole domain at once, as "all" does (`countOf` is `numberOf`
            // over a bare plural), so an Agreement in the domain constrains
            // that group.
            Some(
                ("Nominal", "Coordination" | "CompareOver")
                | ("Amount", "AggregateOver")
                | ("PostModifier", "Superlative"),
            ) => {
                for (_, _, aterm) in args {
                    self.walk(aterm, Place::Elsewhere, refusals);
                }
            }
            // The rule, not a fallback: every other consumer of a `Nominal`
            // (`this`, `attachHost`, `run`, `addsTarget`, `everyCardIs`,
            // `noCardIs`, the `Reach` words, a rules row's scope, …) tests each
            // member on its own or names a head noun, so it forms no group for
            // Agreement, whatever its constructor.
            _ => {
                for (_, psort, aterm) in args {
                    if is_nominal_sort(psort) {
                        self.refuse_agreement_in(aterm, refusals);
                    }
                    self.walk(aterm, Place::Elsewhere, refusals);
                }
            }
        }
    }

    fn refuse_agreement_in(&self, term: &Term<'a>, refusals: &mut Vec<Refusal>) {
        if let Some(span) = self.nominal_agree_span(term) {
            refusals.push(Refusal {
                span,
                kind: RefusalKind::AgreementWithoutGroup,
            });
        }
    }

    /// Returns the span of the first `PostModifier = Agree` inside `term`'s own
    /// `Nominal` structure (without crossing into nested `NounPhrase` or
    /// sub-domain arguments).
    fn nominal_agree_span(&self, term: &Term<'a>) -> Option<Span> {
        match term {
            Term::List(items) => items.iter().find_map(|it| self.nominal_agree_span(it)),
            Term::Call { decl, args, .. } => {
                if self.builds(*decl, "Nominal", "Nominal") {
                    args.iter()
                        .find_map(|(pname, _, aterm)| match pname.as_str() {
                            // An Agreement phrase is a postmodifier, never a
                            // premodifier.
                            "pre" => None,
                            "head" => self.nominal_agree_span(aterm),
                            "post" => self.post_modifier_agree_span(aterm),
                            other => panic!("`nominal` has no parameter `{other}`"),
                        })
                } else if self.builds(*decl, "Nominal", "Coordination") {
                    args.iter()
                        .find_map(|(_, _, aterm)| self.nominal_agree_span(aterm))
                } else {
                    // A head word, or a Nominal built over a domain of its own
                    // (`compareOver`): any Agreement inside belongs to that
                    // domain's own consumer, which the walk checks there.
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
                decl, span, args, ..
            } => {
                if self.builds(*decl, "PostModifier", "Agree") {
                    Some(*span)
                } else if self.builds(*decl, "PostModifier", "Coordination")
                    || self.builds(*decl, "PostModifier", "Not")
                {
                    args.iter()
                        .find_map(|(_, _, aterm)| self.post_modifier_agree_span(aterm))
                } else {
                    // Any other postmodifier is no Agreement and holds the
                    // nominal's postmodifiers in no argument; a nominal inside
                    // one is checked at its own consumer.
                    None
                }
            }
            Term::Record { .. } | Term::Int(_) | Term::Param | Term::None | Term::Other => None,
        }
    }

    // ---- Number: each form of each sort, by constructor ----

    /// A `DetPhrase`'s number: "all", the bare plural and a possessive form a
    /// group, "a" and distributive "each" do not, "the" is its plurality's, and
    /// a count or a target is its quantity's.
    fn determiner_number(&self, det: &Term<'a>, refusals: &mut Vec<Refusal>) -> Number {
        let Some((decl, span, args)) = call_of(det, "DetPhrase") else {
            return Number::Parameter;
        };
        match self.constructor(decl) {
            Some(("DetPhrase", "All" | "Bare" | "Possessive")) => Number::Group,
            Some(("DetPhrase", "A" | "Each")) => Number::AtMostOne,
            Some(("DetPhrase", "The")) => self.plurality_number(named(args, "plur"), refusals),
            Some(("DetPhrase", "Count" | "Target")) => {
                self.quantity_number(named(args, "quantity"), refusals)
            }
            _ => self.unclassified(decl, span, refusals),
        }
    }

    /// A `SliceCount`'s number: the whole group and a slice totaling a bound
    /// form a group, and a counted slice is its quantity's.
    fn slice_count_number(&self, count: &Term<'a>, refusals: &mut Vec<Refusal>) -> Number {
        let Some((decl, span, args)) = call_of(count, "SliceCount") else {
            return Number::Parameter;
        };
        match self.constructor(decl) {
            Some(("SliceCount", "Whole" | "Totaling")) => Number::Group,
            Some(("SliceCount", "Counted")) => {
                self.quantity_number(named(args, "quantity"), refusals)
            }
            _ => self.unclassified(decl, span, refusals),
        }
    }

    fn plurality_number(&self, plur: &Term<'a>, refusals: &mut Vec<Refusal>) -> Number {
        let Some((decl, span, _)) = call_of(plur, "Plurality") else {
            return Number::Parameter;
        };
        match self.constructor(decl) {
            Some(("Plurality", "Many")) => Number::Group,
            Some(("Plurality", "One")) => Number::AtMostOne,
            _ => self.unclassified(decl, span, refusals),
        }
    }

    /// A `Quantity`'s number: at most one when it can never exceed one.
    fn quantity_number(&self, q: &Term<'a>, refusals: &mut Vec<Refusal>) -> Number {
        let Some((decl, span, args)) = call_of(q, "Quantity") else {
            return Number::Parameter;
        };
        match self.constructor(decl) {
            // "one", "two", "up to one", "up to three": bounded by the amount.
            Some(("Quantity", "ExactlyOf" | "UpToOf")) => self.amount_number(named(args, "amount")),
            // "N or more", "any number", "one or two": bounded by `high`.
            Some(("Quantity", "Range")) => match named(args, "high") {
                Term::Int(n) if *n <= 1 => Number::AtMostOne,
                Term::Int(_) | Term::None => Number::Group,
                Term::Param => Number::Parameter,
                other => panic!("`high` of `range` is a Nat or none, not {other:?}"),
            },
            // "more than N" has no upper bound.
            Some(("Quantity", "MoreThanOf")) => Number::Group,
            // "an additional N" counts N.
            Some(("Quantity", "Additional")) => {
                self.quantity_number(named(args, "quantity"), refusals)
            }
            // "the last" one of its kind.
            Some(("Quantity", "Last")) => Number::AtMostOne,
            // "N worth of": items weighing at least one each, so at most N.
            Some(("Quantity", "Worth")) => match named(args, "n") {
                Term::Int(n) if *n <= 1 => Number::AtMostOne,
                Term::Int(_) => Number::Group,
                Term::Param => Number::Parameter,
                other => panic!("`n` of `worth` is a Nat, not {other:?}"),
            },
            _ => self.unclassified(decl, span, refusals),
        }
    }

    /// An `Amount`'s number: a literal is its value; any other amount is a
    /// number the game supplies, which can exceed one.
    fn amount_number(&self, amount: &Term<'a>) -> Number {
        match amount {
            Term::Call { decl, args, .. } if self.builds(*decl, "Amount", "Literal") => {
                match args.as_slice() {
                    [(_, _, Term::Int(n))] if *n <= 1 => Number::AtMostOne,
                    [(_, _, Term::Int(_))] => Number::Group,
                    [(_, _, Term::Param)] => Number::Parameter,
                    other => panic!("`lit` takes one integer, not {other:?}"),
                }
            }
            Term::Call { .. } => Number::Group,
            Term::Param => Number::Parameter,
            other => panic!("an Amount is a call or a parameter, not {other:?}"),
        }
    }

    fn unclassified(&self, decl: usize, span: Span, refusals: &mut Vec<Refusal>) -> Number {
        refusals.push(Refusal {
            span,
            kind: RefusalKind::UnclassifiedNumber {
                word: self.word(decl),
                sort: self.decls[decl].sort.to_string(),
            },
        });
        Number::Group
    }

    /// The word a declaration declares, or its label when it declares none.
    fn word(&self, decl: usize) -> String {
        let decl = &self.decls[decl];
        decl.name.clone().unwrap_or_else(|| decl.label())
    }
}

/// The argument named `name`.
fn named<'t, 'a>(args: &'t Args<'a>, name: &str) -> &'t Term<'a> {
    args.iter()
        .find(|(n, _, _)| n == name)
        .map_or_else(|| panic!("`{name}` is a parameter"), |(_, _, t)| t)
}

/// `term` as a call at `sort`: its declaration, span and arguments, or `None`
/// for a Template parameter.
fn call_of<'t, 'a>(term: &'t Term<'a>, sort: &str) -> Option<(usize, Span, &'t Args<'a>)> {
    match term {
        Term::Call {
            decl, span, args, ..
        } => Some((*decl, *span, args)),
        Term::Param => None,
        other => panic!("a {sort} is a call or a parameter, not {other:?}"),
    }
}

fn is_nominal_sort(sort: &SortKey) -> bool {
    let inner = sort.option_inner().unwrap_or(sort);
    let elem = inner.list_inner().unwrap_or(inner);
    elem.args.is_empty() && elem.head == "Nominal"
}

/// Where a value of a declaration stands.
#[derive(Clone, Copy)]
enum ValueSlot {
    /// A value-shaped declaration's whole value.
    Declaration,
    /// The default of the parameter at this index.
    Default(usize),
    /// A non-metadata field of the declaration's block.
    Field(&'static str),
}

/// Every value a declaration writes, with where it stands.
fn decl_values<'a>(decl: &Decl<'a>) -> Vec<(&'a Value, ValueSlot)> {
    let mut out = Vec::new();
    match &decl.declaration.shape {
        Shape::Value(value) => out.push((value, ValueSlot::Declaration)),
        Shape::Header(header) => {
            for (index, param) in header.params.iter().filter_map(Item::node).enumerate() {
                if let Some(default) = &param.default {
                    out.push((default, ValueSlot::Default(index)));
                }
            }
            if let Some(block) = &header.body {
                for field in block.nodes() {
                    let name = field.name.ident.as_str();
                    if sylex_syl::METADATA_FIELDS.contains(&name) {
                        continue;
                    }
                    let spec = decl
                        .kind
                        .fields
                        .iter()
                        .find(|spec| spec.name == name)
                        .unwrap_or_else(|| {
                            panic!("field `{name}` of {} is declared", decl.label())
                        });
                    out.push((&field.value, ValueSlot::Field(spec.name)));
                }
            }
        }
    }
    out
}

#[derive(Debug, Clone)]
enum Term<'a> {
    Call {
        decl: usize,
        span: Span,
        args: Vec<(String, SortKey, Term<'a>)>,
        /// A Definition's body with this call's arguments; `None` for a
        /// primitive and for a declaration with no parameters.
        body: Option<Box<Term<'a>>>,
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

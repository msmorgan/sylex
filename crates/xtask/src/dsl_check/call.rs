//! A call's arguments (`syl.md` §2) and written-form rules (`spelling.md` §4,
//! §7).
//!
//! A required argument is positional and binds the next required parameter
//! not yet bound; a defaulted argument is passed by name, and never with the
//! value its parameter declares as its default. Arguments are written in
//! declaration order.
//!
//! Conjunction and disjunction constructors
//! (`schema::CONJUNCTION_AND_DISJUNCTION_CONSTRUCTORS`), and every Template
//! forwarding a `List` parameter into one, take two or more
//! items. At `TriggerCondition`, `and` and `or` are flat and `triggered` is
//! written only over a disjunction, never over a single `when`, `whenever` or
//! `at` outside that `Ability` Template's own body.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use sylex_syl::Arg;
use sylex_syl::Call;
use sylex_syl::Item;
use sylex_syl::Shape;
use sylex_syl::Value;
use sylex_syl::ValueKind;

use super::Decl;
use super::RefusalKind;
use super::Registry;
use super::Resolver;
use super::Signature;
use super::Slot;
use super::membership::Mode;
use super::membership::Sorts;
use super::schema;
use super::sort::SortKey;

/// Call-site written-form rules derived from the declarations and [`schema`].
#[derive(Default)]
pub(super) struct WrittenForms {
    /// `(decl, param)` pairs whose list argument takes two or more items:
    /// `None` when the single item itself stands at `decl.sort`, or
    /// `Some(form)` when `decl` wraps the conjunction or disjunction inside an
    /// outer call (such as `establishFor` wrapping `conjunction` in
    /// `establish`).
    two_or_more: BTreeMap<(usize, usize), Option<String>>,
    /// `(decl, param)` pairs where `decl` builds `Triggered` at `Ability` and
    /// `param` is its `TriggerCondition` parameter.
    triggered_condition: BTreeSet<(usize, usize)>,
    /// Declarations at `TriggerCondition` that build or expand to a trigger
    /// condition disjunction (`or` and `and` at `TriggerCondition`).
    trigger_disjunctions: BTreeSet<usize>,
}

pub(super) fn written_forms(
    decls: &[Decl<'_>],
    registry: &Registry,
    sorts: &Sorts,
) -> WrittenForms {
    let mut forms = WrittenForms::default();
    let triggered_sort = SortKey::named(schema::TRIGGERED_ABILITY.0);
    let trig_cond_sort = SortKey::named(schema::TRIGGER_CONDITION_SORT);
    for (decl_idx, decl) in decls.iter().enumerate() {
        if decl.kind.word != schema::PRIMITIVE {
            continue;
        }
        let Shape::Header(header) = &decl.declaration.shape else {
            continue;
        };
        let Some(ctor) = header.constructor.map(|c| c.ident.as_str()) else {
            continue;
        };
        if schema::CONJUNCTION_AND_DISJUNCTION_CONSTRUCTORS.contains(&ctor) {
            for (param_idx, param) in decl.params.iter().enumerate() {
                if param.sort.list_inner() == Some(&decl.sort) {
                    forms.two_or_more.insert((decl_idx, param_idx), None);
                }
            }
        }
        if decl.sort == triggered_sort && ctor == schema::TRIGGERED_ABILITY.1 {
            for (param_idx, param) in decl.params.iter().enumerate() {
                if param.sort == trig_cond_sort {
                    forms.triggered_condition.insert((decl_idx, param_idx));
                }
            }
        }
    }
    while propagate_templates(decls, registry, sorts, &mut forms) {}
    for &(decl_idx, param_idx) in forms.two_or_more.keys() {
        let decl = &decls[decl_idx];
        if decl.sort == trig_cond_sort
            && decl.params[param_idx].sort.list_inner() == Some(&trig_cond_sort)
        {
            forms.trigger_disjunctions.insert(decl_idx);
        }
    }
    forms
}

fn propagate_templates(
    decls: &[Decl<'_>],
    registry: &Registry,
    sorts: &Sorts,
    forms: &mut WrittenForms,
) -> bool {
    let mut changed = false;
    for (decl_idx, decl) in decls.iter().enumerate() {
        if decl.kind.word != schema::TEMPLATE {
            continue;
        }
        let Some(body) = decl.field("body") else {
            continue;
        };
        let mut prop = Propagator {
            decl_idx,
            decl,
            root_body: body,
            registry,
            sorts,
            forms,
            changed: &mut changed,
        };
        // Every sort of an `at` list has the one declaration's parameters, so
        // its first sort stands for all.
        let own = decl.at.first().unwrap_or(&decl.sort);
        prop.walk(body, own);
    }
    changed
}

struct Propagator<'a, 'd> {
    decl_idx: usize,
    decl: &'a Decl<'d>,
    root_body: &'a Value,
    registry: &'a Registry,
    sorts: &'a Sorts,
    forms: &'a mut WrittenForms,
    changed: &'a mut bool,
}

impl Propagator<'_, '_> {
    /// Walks a Template's `body` before the main resolution pass so every
    /// `Resolver::call` sees the transitive closure regardless of declaration
    /// order. Unresolved heads and unbound or duplicate arguments are skipped
    /// here because `Resolver::word` and `Resolver::call` check and refuse
    /// them on the main pass.
    fn walk(&mut self, value: &Value, expected: &SortKey) {
        let own = self.decl.at.first();
        let expected = &match own {
            Some(own) => expected.instantiate(own),
            None => expected.clone(),
        };
        let at = expected.option_inner().unwrap_or(expected);
        match &value.kind {
            ValueKind::Call(call) => {
                let head = call.head.ident.as_str();
                if head == "none" || (head == "some" && expected.option_inner().is_some()) {
                    return;
                }
                let signature = signature_in(self.registry, self.sorts, head, expected);
                let Some(sig) = signature.filter(|sig| !sig.secondary) else {
                    return;
                };
                let mut bound_indices = vec![false; sig.params.len()];
                for arg in call.args.iter().filter_map(Item::node) {
                    let index = match arg.name {
                        Some(name) => sig
                            .params
                            .iter()
                            .position(|p| p.name == name.ident.as_str()),
                        None => (0..sig.params.len())
                            .find(|&i| !sig.params[i].optional && !bound_indices[i]),
                    };
                    let Some(callee_param) = index.filter(|&i| !bound_indices[i]) else {
                        continue;
                    };
                    bound_indices[callee_param] = true;
                    if let ValueKind::Parameter(pname) = &arg.value.kind
                        && let Some(callee_instead) = self
                            .forms
                            .two_or_more
                            .get(&(sig.decl, callee_param))
                            .cloned()
                        && let Some(caller_param) = self
                            .decl
                            .params
                            .iter()
                            .position(|p| p.name == pname.ident.as_str())
                        && self.decl.params[caller_param].sort.list_inner().is_some()
                        && !self
                            .forms
                            .two_or_more
                            .contains_key(&(self.decl_idx, caller_param))
                    {
                        let instead = if self.decl.params[caller_param].sort.list_inner()
                            == Some(&self.decl.sort)
                        {
                            None
                        } else {
                            callee_instead.or_else(|| Some(single_item_form(self.root_body)))
                        };
                        self.forms
                            .two_or_more
                            .insert((self.decl_idx, caller_param), instead);
                        *self.changed = true;
                    }
                    self.walk(&arg.value, &sig.params[callee_param].sort);
                }
            }
            ValueKind::List(items) => {
                if let Some(inner) = at.list_inner() {
                    for item in items.iter().filter_map(Item::node) {
                        self.walk(item, inner);
                    }
                }
            }
            ValueKind::Record(record) => {
                if let Some(spec) = schema::RECORDS
                    .iter()
                    .find(|spec| at.args.is_empty() && spec.sort == at.head)
                {
                    for field in record.nodes() {
                        if let Some((_, fsort, _)) = spec
                            .fields
                            .iter()
                            .find(|(name, _, _)| *name == field.name.ident.as_str())
                        {
                            self.walk(&field.value, &SortKey::table(fsort));
                        }
                    }
                }
            }
            ValueKind::Parameter(_) | ValueKind::String(_) | ValueKind::Integer(_) => {}
        }
    }
}

/// The signature a call of `head` binds its arguments against where
/// `expected` is expected in a body: the declaration at `expected` itself, at
/// the sort `expected` is `Option` of, or at a sort of that sort's Reaches.
/// Every sort of an `at` list has the one declaration's parameter names and
/// order, so the first found stands for all; `None` when the word stands at no
/// such sort, or at sorts of two declarations.
fn signature_in<'r>(
    registry: &'r Registry,
    sorts: &Sorts,
    head: &str,
    expected: &SortKey,
) -> Option<&'r Signature> {
    if let Some(signature) = registry.get(&(head.to_owned(), expected.clone())) {
        return Some(signature);
    }
    let at = expected.option_inner().unwrap_or(expected);
    let mut found: Vec<&Signature> = sorts
        .reaches(at, Mode::Body)
        .into_iter()
        .filter_map(|sort| registry.get(&(head.to_owned(), sort)))
        .collect();
    found.dedup_by_key(|signature| signature.decl);
    match found.as_slice() {
        [signature] => Some(signature),
        _ => None,
    }
}

fn single_item_form(value: &Value) -> String {
    match &value.kind {
        ValueKind::Parameter(name) => format!("${}", name.ident.as_str()),
        ValueKind::String(text) => format!("{text:?}"),
        ValueKind::Integer(number) => number.to_string(),
        ValueKind::List(_) => "[...]".to_owned(),
        ValueKind::Record(_) => "{...}".to_owned(),
        ValueKind::Call(call) => {
            let head = call.head.ident.as_str();
            if call.is_bare() { head.to_owned() } else { format!("{head}(...)") }
        }
    }
}

impl Resolver<'_> {
    /// Binds a call's arguments to its signature's parameters and resolves
    /// each at its parameter's sort.
    pub(super) fn call(&mut self, call: &Call, at: &SortKey, signature: &Signature) {
        let word = call.head.ident.as_str();
        // Temporary: `syl-ring-tempts-you-raw-scope` deletes this exception.
        if signature.ring_only && !self.ring {
            let reserved = schema::RING_EXCEPTION.1.to_owned();
            self.refuse(call.head.span, RefusalKind::ReservedConstructor(reserved));
        }
        self.edges.extend(signature.node);
        let params = &signature.params;
        // Each bound parameter's index, with its argument, in written order.
        let mut bound: Vec<(usize, &Arg)> = Vec::new();
        let is_bound =
            |bound: &[(usize, &Arg)], index: usize| bound.iter().any(|(i, _)| *i == index);
        // The latest-declared parameter bound so far.
        let mut latest: Option<usize> = None;
        // A positional argument past the required parameters binds nothing:
        // it is refused once, as a defaulted argument while the call leaves
        // a defaulted parameter unnamed, else as one too many.
        let named: Vec<&str> = call
            .args
            .iter()
            .filter_map(Item::node)
            .filter_map(|arg| arg.name.map(|name| name.ident.as_str()))
            .collect();
        let mut unnamed_defaulted = params
            .iter()
            .filter(|p| p.optional && !named.contains(&p.name.as_str()))
            .count();
        for arg in call.args.iter().filter_map(Item::node) {
            let index = if let Some(name) = arg.name {
                let Some(index) = params.iter().position(|p| p.name == name.ident.as_str()) else {
                    self.refuse(
                        name.span,
                        RefusalKind::UnknownArgument {
                            word: word.to_owned(),
                            argument: name.ident.as_str().to_owned(),
                        },
                    );
                    continue;
                };
                if !params[index].optional {
                    self.refuse(
                        arg.span,
                        RefusalKind::NamedRequired {
                            word: word.to_owned(),
                            argument: params[index].name.clone(),
                        },
                    );
                }
                index
            } else {
                let required =
                    (0..params.len()).find(|&i| !params[i].optional && !is_bound(&bound, i));
                let Some(index) = required else {
                    let kind = if unnamed_defaulted > 0 {
                        unnamed_defaulted -= 1;
                        RefusalKind::PositionalDefaulted(word.to_owned())
                    } else {
                        RefusalKind::TooManyArguments(word.to_owned())
                    };
                    self.refuse(arg.span, kind);
                    continue;
                };
                index
            };
            if is_bound(&bound, index) {
                self.refuse(
                    arg.span,
                    RefusalKind::DuplicateArgument {
                        word: word.to_owned(),
                        argument: params[index].name.clone(),
                    },
                );
                continue;
            }
            if let Some(latest) = latest
                && index < latest
            {
                self.refuse(
                    arg.span,
                    RefusalKind::OutOfOrder {
                        word: word.to_owned(),
                        argument: params[index].name.clone(),
                        after: params[latest].name.clone(),
                    },
                );
            }
            latest = Some(latest.map_or(index, |latest| latest.max(index)));
            bound.push((index, arg));
            self.record.args.insert(std::ptr::from_ref(arg), index);
            self.value(&arg.value, &params[index].sort);
        }
        for (index, param) in params.iter().enumerate() {
            if !param.optional && !is_bound(&bound, index) {
                self.refuse(
                    call.span,
                    RefusalKind::MissingArgument {
                        word: word.to_owned(),
                        argument: param.name.clone(),
                    },
                );
            }
        }
        self.redundant_defaults(word, params, &bound);
        self.written_form(call, at, signature, &bound);
    }

    fn written_form(
        &mut self,
        call: &Call,
        at: &SortKey,
        signature: &Signature,
        bound: &[(usize, &Arg)],
    ) {
        if signature.secondary {
            return;
        }
        let word = call.head.ident.as_str();
        for &(param_index, arg) in bound {
            if self
                .forms
                .triggered_condition
                .contains(&(signature.decl, param_index))
                && let ValueKind::Call(cond_call) = &arg.value.kind
            {
                let cond_word = cond_call.head.ident.as_str();
                let trig_sort = SortKey::named(schema::TRIGGER_CONDITION_SORT);
                if schema::SINGLE_TRIGGER_WORDS.contains(&cond_word)
                    && self
                        .registry
                        .contains_key(&(cond_word.to_owned(), trig_sort))
                {
                    let is_defining_template_body =
                        self.ability_template_body
                            .is_some_and(|(tpl_name, body_call)| {
                                tpl_name == cond_word && std::ptr::eq(call, body_call)
                            });
                    if !is_defining_template_body {
                        self.refuse(
                            call.span,
                            RefusalKind::SingleConditionTriggered(cond_word.to_owned()),
                        );
                    }
                }
            }
            if let Some(wrapper_instead) =
                self.forms.two_or_more.get(&(signature.decl, param_index))
                && let ValueKind::List(items) = &arg.value.kind
            {
                let nodes: Vec<&Value> = items.iter().filter_map(Item::node).collect();
                match nodes.as_slice() {
                    [] => self.refuse(
                        call.span,
                        RefusalKind::EmptyCoordination {
                            word: word.to_owned(),
                            sort: at.to_string(),
                        },
                    ),
                    [only] => {
                        let instead = wrapper_instead
                            .clone()
                            .unwrap_or_else(|| single_item_form(only));
                        self.refuse(
                            call.span,
                            RefusalKind::SingleItemCoordination {
                                word: word.to_owned(),
                                sort: at.to_string(),
                                instead,
                            },
                        );
                    }
                    [_, _, ..] => {
                        if self.forms.trigger_disjunctions.contains(&signature.decl) {
                            let trig_sort = SortKey::named(schema::TRIGGER_CONDITION_SORT);
                            for item in nodes {
                                if let ValueKind::Call(item_call) = &item.kind
                                    && let Some(item_sig) = self.registry.get(&(
                                        item_call.head.ident.as_str().to_owned(),
                                        trig_sort.clone(),
                                    ))
                                    && !item_sig.secondary
                                    && self.forms.trigger_disjunctions.contains(&item_sig.decl)
                                {
                                    self.refuse(
                                        item.span,
                                        RefusalKind::NestedTriggerCondition {
                                            outer: word.to_owned(),
                                            inner: item_call.head.ident.as_str().to_owned(),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Refuses every named argument whose value is its parameter's default.
    /// A default reading an earlier parameter reads that parameter's value
    /// in this call.
    fn redundant_defaults(&mut self, word: &str, params: &[Slot], bound: &[(usize, &Arg)]) {
        // Each parameter's value in this call, in canonical form, for the
        // defaults that read it.
        let mut values: BTreeMap<&str, String> = BTreeMap::new();
        for (index, param) in params.iter().enumerate() {
            let written = bound.iter().find(|(i, _)| *i == index).map(|(_, arg)| *arg);
            let default = param
                .default
                .as_ref()
                .map(|default| self.canonical(default, Some(&param.sort), &values));
            // An argument reads the caller's parameters, never the callee's.
            let written_value =
                written.map(|arg| self.canonical(&arg.value, Some(&param.sort), &BTreeMap::new()));
            if let (Some(arg), Some(value), Some(default)) = (written, &written_value, &default)
                && arg.name.is_some()
                && value == default
            {
                self.refuse(
                    arg.span,
                    RefusalKind::RedundantDefault {
                        word: word.to_owned(),
                        argument: param.name.clone(),
                    },
                );
            }
            // A required parameter left unbound is refused above; a default
            // reading it reads the callee's parameter, never one of the
            // caller's of the same name.
            let value = written_value
                .or(default)
                .unwrap_or_else(|| format!("{word}.${}", param.name));
            values.insert(&param.name, value);
        }
    }

    /// `value` printed so that two values print alike exactly when they
    /// mean the same: layout and spans dropped, arguments named and in
    /// declaration order, record fields sorted, and each `$name` in `params`
    /// replaced by its value. `expected` is the sort `value` stands at, when
    /// known.
    fn canonical(
        &self,
        value: &Value,
        expected: Option<&SortKey>,
        params: &BTreeMap<&str, String>,
    ) -> String {
        let at = expected.map(|sort| sort.option_inner().unwrap_or(sort));
        match &value.kind {
            ValueKind::Parameter(name) => {
                let name = name.ident.as_str();
                params
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| format!("${name}"))
            }
            ValueKind::String(text) => format!("{text:?}"),
            ValueKind::Integer(number) => number.to_string(),
            ValueKind::List(items) => {
                let inner = at.and_then(SortKey::list_inner);
                let items: Vec<String> = items
                    .iter()
                    .filter_map(Item::node)
                    .map(|item| self.canonical(item, inner, params))
                    .collect();
                format!("[{}]", items.join(", "))
            }
            ValueKind::Record(record) => {
                let spec = at.and_then(|at| {
                    schema::RECORDS
                        .iter()
                        .find(|spec| at.args.is_empty() && spec.sort == at.head)
                });
                let mut fields: Vec<String> = record
                    .nodes()
                    .map(|field| {
                        let name = field.name.ident.as_str();
                        let sort = spec.and_then(|spec| {
                            spec.fields
                                .iter()
                                .find(|(field, _, _)| *field == name)
                                .map(|(_, sort, _)| SortKey::table(sort))
                        });
                        let value = self.canonical(&field.value, sort.as_ref(), params);
                        format!("{name} = {value}")
                    })
                    .collect();
                fields.sort();
                format!("{{{}}}", fields.join(", "))
            }
            ValueKind::Call(call) => self.canonical_call(call, at, params),
        }
    }

    fn canonical_call(
        &self,
        call: &Call,
        at: Option<&SortKey>,
        params: &BTreeMap<&str, String>,
    ) -> String {
        let head = call.head.ident.as_str();
        let args: Vec<&Arg> = call.args.iter().filter_map(Item::node).collect();
        if call.is_bare() {
            return head.to_owned();
        }
        if let Some(at) = at
            && let Some(signature) = signature_in(self.registry, self.sorts, head, at)
        {
            let mut bound: Vec<Option<&Arg>> = vec![None; signature.params.len()];
            let mut unbound = Vec::new();
            for arg in args {
                let index = match arg.name {
                    Some(name) => signature
                        .params
                        .iter()
                        .position(|p| p.name == name.ident.as_str()),
                    None => (0..signature.params.len())
                        .find(|&i| !signature.params[i].optional && bound[i].is_none()),
                };
                match index {
                    Some(index) if bound[index].is_none() => bound[index] = Some(arg),
                    // Refused when the call itself is resolved.
                    Some(_) | None => unbound.push(arg),
                }
            }
            let mut written: Vec<String> = signature
                .params
                .iter()
                .zip(&bound)
                .filter_map(|(param, arg)| {
                    arg.map(|arg| {
                        format!(
                            "{} = {}",
                            param.name,
                            self.canonical(&arg.value, Some(&param.sort), params)
                        )
                    })
                })
                .collect();
            written.extend(
                unbound
                    .into_iter()
                    .map(|arg| self.canonical_arg(arg, params)),
            );
            return format!("{head}({})", written.join(", "));
        }
        let written: Vec<String> = args
            .into_iter()
            .map(|arg| self.canonical_arg(arg, params))
            .collect();
        format!("{head}({})", written.join(", "))
    }

    /// An argument whose parameter is unknown, printed as written.
    fn canonical_arg(&self, arg: &Arg, params: &BTreeMap<&str, String>) -> String {
        let value = self.canonical(&arg.value, None, params);
        match arg.name {
            Some(name) => format!("{} = {value}", name.ident.as_str()),
            None => value,
        }
    }
}

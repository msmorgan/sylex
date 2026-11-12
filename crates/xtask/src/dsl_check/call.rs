//! A call's arguments (`syl.md` §2).
//!
//! A required argument is positional and binds the next required parameter
//! not yet bound; a defaulted argument is passed by name, and never with the
//! value its parameter declares as its default. Arguments are written in
//! declaration order.

use std::collections::BTreeMap;

use sylex_syl::Arg;
use sylex_syl::Call;
use sylex_syl::Item;
use sylex_syl::Value;
use sylex_syl::ValueKind;

use super::RefusalKind;
use super::Resolver;
use super::Signature;
use super::Slot;
use super::schema;
use super::sort::SortKey;

impl Resolver<'_> {
    /// Binds a call's arguments to its signature's parameters and resolves
    /// each at its parameter's sort.
    pub(super) fn call(&mut self, call: &Call, signature: &Signature) {
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
    /// mean the same: layout and spans dropped, a coercion's wrap looked
    /// through (`lit(0)` prints as `0` at Amount), arguments named and in
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
        if let Some(at) = at {
            if let [arg] = args.as_slice()
                && arg.name.is_none()
                && let Some(coercion) = self
                    .coercions
                    .iter()
                    .find(|c| c.to == *at && c.wrap == head)
            {
                return self.canonical(&arg.value, Some(&coercion.from), params);
            }
            let key = (head.to_owned(), at.clone());
            if !self.registry.contains_key(&key) {
                // A call that stands at `at` only through a coercion is
                // printed at the coercion's from-sort.
                let mut from = self.coercions.iter().filter(|c| {
                    c.to == *at
                        && self
                            .registry
                            .contains_key(&(head.to_owned(), c.from.clone()))
                });
                if let (Some(coercion), None) = (from.next(), from.next()) {
                    return self.canonical_call(call, Some(&coercion.from), params);
                }
            }
            if let Some(signature) = self.registry.get(&key) {
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

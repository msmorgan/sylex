//! Coercions: the declared conversions from one sort into another
//! (`syl.md` §3).
//!
//! A `coercion` declaration has no name. Its one parameter's sort is the
//! from-sort, its result sort the to-sort, and its body applies a primitive at
//! the to-sort to the parameter. [`coercions`] reads them and refuses, at
//! load and across the whole registry, what would make a coerced word or
//! literal mean two things.

use std::collections::BTreeMap;

use sylex_syl::Span;
use sylex_syl::ValueKind;

use super::Decl;
use super::Refusal;
use super::RefusalKind;
use super::Registry;
use super::schema;
use super::sort::SortKey;

/// One coercion: a value at `from` stands at `to`.
#[derive(Debug)]
pub(super) struct Coercion {
    pub(super) from: SortKey,
    pub(super) to: SortKey,
    /// The primitive its body applies to the parameter.
    pub(super) wrap: String,
    /// The coercion's node in the dependency graph.
    pub(super) node: usize,
    pub(super) span: Span,
}

/// Whether a kind of literal stands at a sort.
type StandsAt = fn(&SortKey) -> bool;

/// Whether a string literal stands at `sort`.
pub(super) fn string_stands_at(sort: &SortKey) -> bool {
    *sort == SortKey::named("String")
}

/// Whether the integer literal `number` stands at `sort`.
pub(super) fn integer_stands_at(number: i64, sort: &SortKey) -> bool {
    *sort == SortKey::named("Int") || (*sort == SortKey::named("Nat") && number >= 0)
}

/// Whether a list literal stands at `sort`.
pub(super) fn list_stands_at(sort: &SortKey) -> bool {
    sort.list_inner().is_some()
}

/// Whether a record literal stands at `sort`.
pub(super) fn record_stands_at(sort: &SortKey) -> bool {
    sort.args.is_empty()
        && schema::RECORDS
            .iter()
            .any(|record| record.sort == sort.head)
}

/// Reads every coercion declaration, refusing a malformed one, a duplicate
/// and a chain, then refuses every word and literal that reaches one
/// coercion's to-sort from two sorts. Returns the coercions that are neither
/// malformed nor duplicates.
pub(super) fn coercions(
    decls: &[Decl<'_>],
    registry: &Registry,
    refusals: &mut Vec<Refusal>,
) -> Vec<Coercion> {
    let mut coercions: Vec<Coercion> = Vec::new();
    for (index, decl) in decls.iter().enumerate() {
        if decl.kind.word != schema::COERCION {
            continue;
        }
        let Some(coercion) = read(index, decl, decls, refusals) else {
            continue;
        };
        if coercions
            .iter()
            .any(|c| c.from == coercion.from && c.to == coercion.to)
        {
            refusals.push(Refusal {
                span: coercion.span,
                kind: RefusalKind::DuplicateCoercion {
                    from: coercion.from.to_string(),
                    to: coercion.to.to_string(),
                },
            });
            continue;
        }
        coercions.push(coercion);
    }
    for coercion in coercions.iter().filter(|c| c.from == c.to) {
        refusals.push(Refusal {
            span: coercion.span,
            kind: RefusalKind::CoercionToItself(coercion.to.to_string()),
        });
    }
    for first in &coercions {
        let chained = |then: &&Coercion| then.from == first.to && !std::ptr::eq(*then, first);
        for then in coercions.iter().filter(chained) {
            refusals.push(Refusal {
                span: then.span,
                kind: RefusalKind::ChainedCoercion {
                    from: first.from.to_string(),
                    via: first.to.to_string(),
                    to: then.to.to_string(),
                },
            });
        }
    }
    let mut targets: Vec<&SortKey> = coercions.iter().map(|c| &c.to).collect();
    targets.sort();
    targets.dedup();
    for target in targets {
        ambiguities(target, &coercions, registry, refusals);
    }
    coercions
}

/// Reads one coercion declaration, refusing it unless it takes one parameter
/// with no default and its body is a primitive at its sort applied to that
/// parameter alone.
fn read(
    index: usize,
    decl: &Decl<'_>,
    decls: &[Decl<'_>],
    refusals: &mut Vec<Refusal>,
) -> Option<Coercion> {
    let span = decl.declaration.span;
    let [param] = decl.params.as_slice() else {
        refusals.push(Refusal {
            span,
            kind: RefusalKind::CoercionParameters,
        });
        return None;
    };
    if param.optional {
        refusals.push(Refusal {
            span,
            kind: RefusalKind::CoercionParameters,
        });
        return None;
    }
    // The resolver refuses a missing `body`, a required field.
    let body = decl.field("body")?;
    let wrap = match &body.kind {
        ValueKind::Call(call) => {
            let head = call.head.ident.as_str();
            let primitive = decls.iter().any(|other| {
                other.kind.word == schema::PRIMITIVE
                    && other.name.as_deref() == Some(head)
                    && other.sort == decl.sort
            });
            let mut args = call.args.iter().filter_map(sylex_syl::Item::node);
            let reads_param = match (args.next(), args.next()) {
                (Some(arg), None) => {
                    matches!(&arg.value.kind, ValueKind::Parameter(name) if name.ident == param.name.as_str())
                }
                (None | Some(_), _) => false,
            };
            (primitive && reads_param).then(|| head.to_owned())
        }
        ValueKind::String(_)
        | ValueKind::Integer(_)
        | ValueKind::List(_)
        | ValueKind::Record(_)
        | ValueKind::Parameter(_) => None,
    };
    let Some(wrap) = wrap else {
        refusals.push(Refusal {
            span: body.span,
            kind: RefusalKind::CoercionBody(decl.sort.to_string()),
        });
        return None;
    };
    Some(Coercion {
        from: param.sort.clone(),
        to: decl.sort.clone(),
        wrap,
        node: index,
        span,
    })
}

/// Refuses every word declared at two sorts of Reaches(`target`), `target`
/// and every from-sort of a coercion into it, or at `Option(target)` and a
/// from-sort (at an `Option(target)` position the first would shadow the
/// coercion), and every kind of literal that stands at two sorts of
/// Reaches(`target`).
fn ambiguities(
    target: &SortKey,
    coercions: &[Coercion],
    registry: &Registry,
    refusals: &mut Vec<Refusal>,
) {
    let into: Vec<&Coercion> = coercions.iter().filter(|c| c.to == *target).collect();
    let froms = into.iter().map(|c| &c.from);
    let reaches: Vec<&SortKey> = std::iter::once(target).chain(froms.clone()).collect();
    words(&reaches, target, None, registry, refusals);
    let optional = SortKey {
        head: "Option".to_owned(),
        args: vec![target.clone()],
    };
    let shadowing: Vec<&SortKey> = std::iter::once(&optional).chain(froms).collect();
    words(&shadowing, &optional, Some(&optional), registry, refusals);
    // Only `String` takes a string literal, so no string reaches a target
    // from two sorts.
    let literals: [(&str, StandsAt); 3] = [
        ("an integer", |sort| integer_stands_at(0, sort)),
        ("a list", list_stands_at),
        ("a record", record_stands_at),
    ];
    for (literal, stands_at) in literals {
        let sorts: Vec<&SortKey> = reaches.iter().copied().filter(|s| stands_at(s)).collect();
        if sorts.len() < 2 {
            continue;
        }
        let span = into
            .iter()
            .map(|c| c.span)
            .max_by_key(|span| (span.file, span.start))
            .expect("two sorts reach the target, so a coercion goes into it");
        refusals.push(Refusal {
            span,
            kind: RefusalKind::AmbiguousLiteral {
                literal: literal.to_owned(),
                sorts: sorts.iter().map(ToString::to_string).collect(),
                target: target.to_string(),
            },
        });
    }
}

/// Refuses every word declared at two of `sorts`, all of which reach
/// `target`; with `including`, only a word one of whose sorts it is.
fn words(
    sorts: &[&SortKey],
    target: &SortKey,
    including: Option<&SortKey>,
    registry: &Registry,
    refusals: &mut Vec<Refusal>,
) {
    let mut words: BTreeMap<&str, Vec<(&SortKey, Span)>> = BTreeMap::new();
    for ((name, sort), signature) in registry {
        if sorts.contains(&sort) {
            words
                .entry(name.as_str())
                .or_default()
                .push((sort, signature.span));
        }
    }
    for (word, declared) in words {
        let included = including.is_none_or(|sort| declared.iter().any(|(s, _)| *s == sort));
        if declared.len() < 2 || !included {
            continue;
        }
        let span = declared
            .iter()
            .map(|(_, span)| *span)
            .max_by_key(|span| (span.file, span.start))
            .expect("two declarations");
        refusals.push(Refusal {
            span,
            kind: RefusalKind::AmbiguousWord {
                word: word.to_owned(),
                sorts: declared.iter().map(|(sort, _)| sort.to_string()).collect(),
                target: target.to_string(),
            },
        });
    }
}

//! Properties of expansions over today's declarations. When the proper
//! expansion on the Rust AST replaces this expander, these are rewritten
//! against it; what they state is what lasts.
//!
//! The opaque test value X is written `$x`: a free parameter of the Template
//! [`probe`] writes around a test's term, which expands to
//! `Node::Param("x")`. Binding the subject word to X is plain substitution
//! of that node for every subject word in a tree.

use std::path::PathBuf;

use sylex_syl::Item;
use sylex_syl::Shape;
use sylex_syl::Value;
use sylex_syl::ValueKind;

use super::Expander;
use super::Node;
use super::Role;
use super::bundles;
use super::check;
use super::probe;
use super::role;
use crate::dsl_check::Checked;

/// The word a body reads its clause's subject through, at `NounPhrase`. It
/// means the subject of the clause; `syl-misnamed-declarations` renames it
/// `subject`, a change to this line alone.
const SUBJECT_WORD: &str = "actor";

/// The sort a clause's participants stand at.
const PARTICIPANT_SORT: &str = "NounPhrase";

/// The Handoff: an instruction to someone to do a deed.
const INSTRUCT_WORD: &str = "instruct";

/// Checks the plugins with `probes` added and runs `test` on the result.
fn with_probes<T>(
    probes: Vec<(PathBuf, String)>,
    test: impl FnOnce(&Checked<'_>, &Expander<'_, '_>) -> T,
) -> T {
    let bundles = bundles(probes).expect("the plugins and probes load");
    let checked = check(&bundles).expect("the plugins and probes check");
    let expander = Expander::new(&checked);
    test(&checked, &expander)
}

fn is_subject_word(node: &Node) -> bool {
    matches!(node, Node::Call { word, sort, args, .. }
        if word == SUBJECT_WORD && sort == PARTICIPANT_SORT && args.is_empty())
}

/// `node` with the subject word bound to `x`.
fn bind_subject(node: &Node, x: &Node) -> Node {
    node.replace(&is_subject_word, x)
}

fn x() -> Node {
    Node::Param("x".to_owned())
}

/// The expansion of `destroy(X)` contains, inside its Definition mark, the
/// expansion of `die` with the subject word bound to X: destroying moves a
/// permanent from the battlefield to its owner's graveyard [CR#701.8a],
/// which is what dying is [CR#700.4].
#[test]
fn destroy_contains_die_with_its_subject_bound() {
    let probes = vec![
        probe(
            "probeDestroy",
            &[("x", "NounPhrase")],
            "Action",
            "destroy($x)",
        ),
        probe("probeDie", &[], "Action", "die"),
    ];
    with_probes(probes, |_, expander| {
        let destroy = expander.term("probeDestroy", "Action").unwrap();
        let die = bind_subject(&expander.term("probeDie", "Action").unwrap(), &x());
        let Node::Mark {
            definition, body, ..
        } = &destroy
        else {
            panic!("destroy(X) expands to no mark:\n{destroy}");
        };
        assert_eq!(definition, "destroy");
        assert!(
            body.descendants().contains(&&die),
            "destroy(X):\n{destroy}\ndie, its subject bound to X:\n{die}"
        );
    });
}

/// Every Definition a value reaches, written in it or in the body of a
/// Template it calls; a Definition's own body is not entered.
fn definitions_reached(checked: &Checked<'_>, value: &Value, out: &mut Vec<String>) {
    match &value.kind {
        ValueKind::Call(call) => {
            if let Some(sort) = checked.resolutions.call(call) {
                let word = call.head.ident.as_str();
                let signature = &checked.registry[&(word.to_owned(), sort.clone())];
                let decl = &checked.decls[signature.decl];
                if !signature.secondary {
                    match role(decl.kind.word).unwrap() {
                        Role::Definition => out.push(word.to_owned()),
                        Role::Template => {
                            definitions_reached(checked, decl.field("body").unwrap(), out);
                        }
                        Role::Primitive | Role::Named => {}
                    }
                }
            }
            for arg in call.args.iter().filter_map(Item::node) {
                definitions_reached(checked, &arg.value, out);
            }
        }
        ValueKind::List(items) => {
            for item in items.iter().filter_map(Item::node) {
                definitions_reached(checked, item, out);
            }
        }
        ValueKind::Record(record) => {
            for field in record.nodes() {
                definitions_reached(checked, &field.value, out);
            }
        }
        ValueKind::String(_) | ValueKind::Integer(_) | ValueKind::Parameter(_) => {}
    }
}

fn marks(node: &Node) -> Vec<&Node> {
    node.descendants()
        .into_iter()
        .filter(|node| matches!(node, Node::Mark { .. }))
        .collect()
}

/// Expanding a call to a Definition yields exactly one mark for that call,
/// carrying the Deed its declaration states, and a Definition reached inside
/// its body keeps its own mark nested inside. Over every `action`
/// declaration, each called with its required parameters free.
#[test]
fn a_definition_call_is_one_mark_with_its_deed() {
    let builtin = bundles(Vec::new()).unwrap();
    let actions: Vec<(String, Vec<(String, String)>)> = check(&builtin)
        .unwrap()
        .decls
        .iter()
        .filter(|decl| decl.kind.word == "action")
        .map(|decl| {
            let Shape::Header(header) = &decl.declaration.shape else {
                unreachable!("an action has a header");
            };
            let required = header
                .params
                .iter()
                .filter_map(Item::node)
                .zip(&decl.params)
                .filter(|(param, _)| param.default.is_none())
                .map(|(_, slot)| (slot.name.clone(), slot.sort.to_string()))
                .collect();
            (decl.name.clone().unwrap(), required)
        })
        .collect();
    assert_ne!(actions.len(), 0, "the plugins declare actions");
    let probe_name = |action: &str| format!("probeCall_{action}");
    let probes = actions
        .iter()
        .map(|(action, params)| {
            let reads: Vec<String> = params.iter().map(|(name, _)| format!("${name}")).collect();
            let params: Vec<(&str, &str)> = params
                .iter()
                .map(|(name, sort)| (name.as_str(), sort.as_str()))
                .collect();
            let call = if reads.is_empty() {
                action.clone()
            } else {
                format!("{action}({})", reads.join(", "))
            };
            probe(&probe_name(action), &params, "Action", &call)
        })
        .collect();
    with_probes(probes, |checked, expander| {
        let mut failures = Vec::new();
        for (action, _) in &actions {
            let expansion = expander.term(&probe_name(action), "Action").unwrap();
            let decl = expander.declaration(action, "Action").unwrap();
            let stated = Expander::deed(decl).unwrap();
            let Node::Mark {
                definition,
                deed,
                body,
                ..
            } = &expansion
            else {
                failures.push(format!("`{action}` expands to no mark:\n{expansion}"));
                continue;
            };
            if definition != action || *deed != stated {
                failures.push(format!(
                    "`{action}` expands to the wrong mark (states {stated:?}):\n{expansion}"
                ));
            }
            let own = marks(&expansion)
                .into_iter()
                .filter(
                    |mark| matches!(mark, Node::Mark { definition, .. } if definition == action),
                )
                .count();
            if own != 1 {
                failures.push(format!(
                    "`{action}` expands to {own} marks of its own:\n{expansion}"
                ));
            }
            let mut reached = Vec::new();
            definitions_reached(checked, decl.field("body").unwrap(), &mut reached);
            for inner in reached {
                let nested = marks(body).into_iter().any(
                    |mark| matches!(mark, Node::Mark { definition, .. } if *definition == inner),
                );
                if !nested {
                    failures.push(format!(
                        "`{action}` reaches `{inner}`, whose mark is not inside:\n{expansion}"
                    ));
                }
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n\n"));
    });
}

/// An observation of something occurring expands to a tree with no
/// `instruct` in it and no participant word other than the subject word,
/// both with its defaults filled and with its parameters free.
#[test]
fn an_observation_has_no_instruction_and_no_other_participant() {
    let observations = [
        ("die", &[][..]),
        (
            "enter",
            &[("from", "Location"), ("riders", "List(MoveRider)")][..],
        ),
        ("leave", &[("from", "Location")][..]),
    ];
    let mut probes = Vec::new();
    for (word, params) in observations {
        probes.push(probe(&format!("probeDefault_{word}"), &[], "Action", word));
        if !params.is_empty() {
            let named: Vec<String> = params
                .iter()
                .map(|(name, _)| format!("{name} = ${name}"))
                .collect();
            let term = format!("{word}({})", named.join(", "));
            probes.push(probe(&format!("probeFree_{word}"), params, "Action", &term));
        }
    }
    let names: Vec<String> = probes
        .iter()
        .map(|(path, _)| path.file_stem().unwrap().to_string_lossy().into_owned())
        .collect();
    with_probes(probes, |_, expander| {
        let mut failures = Vec::new();
        for name in names {
            let expansion = expander.term(&name, "Action").unwrap();
            for node in expansion.descendants() {
                let offends = match node {
                    Node::Call { word, .. } if word == INSTRUCT_WORD => true,
                    Node::Call { sort, .. } | Node::Word { sort, .. } => {
                        sort == PARTICIPANT_SORT && !is_subject_word(node)
                    }
                    Node::Mark { .. }
                    | Node::Param(_)
                    | Node::String(_)
                    | Node::Integer(_)
                    | Node::List(_)
                    | Node::Record(_)
                    | Node::None => false,
                };
                if offends {
                    failures.push(format!("`{name}` holds {node}:\n{expansion}"));
                }
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n\n"));
    });
}

/// Every declaration's fields, each parameter free, and every card expand.
#[test]
fn every_declaration_and_card_expands() {
    let bundles = bundles(Vec::new()).unwrap();
    let checked = check(&bundles).unwrap();
    let expander = Expander::new(&checked);
    let mut failures = Vec::new();
    for decl in &checked.decls {
        let result = if decl.kind.word == "card" {
            expander.card(decl).map(drop)
        } else {
            expander.fields(decl).map(drop)
        };
        if let Err(error) = result {
            failures.push(format!("{}: {error:#}", decl.label()));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

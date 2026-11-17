//! The settled spine (`sylex_syntax::ForkForest::settled_spine`) on real
//! Corpus Units, against two oracles:
//!
//! - on the unit's forest, the same walk computed from the Packed Forest's own
//!   counts (`Parse::counts`): from the one root with Derivations, down through
//!   each node with one alternative with Derivations and one family with
//!   Derivations at each step of its item chain;
//! - on the forest and each forest narrowed from it by picking the first option
//!   of the first fork until none is left, the enumerated Readings: every
//!   Reading builds each spine node as the spine says, and where the spine
//!   stops, the Readings differ;
//! - on the same forests, `settled_readings`: the Readings of `readings`, each
//!   with a tree that builds it, and for a forest of one Derivation the spine
//!   itself.

#[path = "support/unit_texts.rs"]
mod unit_texts;

use sylex_english::corpus::Corpus;
use sylex_syntax::ChildRef;
use sylex_syntax::Counts;
use sylex_syntax::ForkForest;
use sylex_syntax::Parse;
use sylex_syntax::Reading;
use sylex_syntax::SettledChild;
use sylex_syntax::SettledNode;
use sylex_syntax::SettledRest;
use sylex_syntax::SettledSpine;
use sylex_vocabulary::LexicalEnvironment;

/// Real units, each for what it exercises:
/// - *Draugr Necromancer* (3,312 Derivations), *Maralen, Fae Ascendant* and
///   *Remove Enchantments* (3,472): large forests, and on narrowing a spine
///   that grows to the whole tree;
/// - *Crook of Condemnation*: a spine that stops where one dependency fork
///   begins;
/// - *Decimate*: more than one live root;
/// - *Up the Beanstalk*: a node that packs two lexical readings of one word.
const SAMPLE: &[&str] = &[
    "Draugr Necromancer",
    "Maralen, Fae Ascendant",
    "Remove Enchantments",
    "Crook of Condemnation",
    "Decimate",
    "Up the Beanstalk",
];

/// Above this many Derivations a forest's Readings are not enumerated.
const ENUMERATED: u128 = 2_000;

/// The spine computed from the Packed Forest's own counts.
fn counted_spine(parse: &Parse<'_>, counts: &Counts<'_>) -> SettledSpine {
    let live: Vec<usize> = parse
        .forest()
        .roots()
        .iter()
        .copied()
        .filter(|&r| counts.node(r) > 0)
        .collect();
    match live[..] {
        [root] => SettledSpine::Root(counted_node(parse, counts, root)),
        _ => SettledSpine::Roots(live.len()),
    }
}

fn counted_node(parse: &Parse<'_>, counts: &Counts<'_>, node: usize) -> SettledNode {
    let forest = parse.forest();
    let grammar = parse.grammar();
    assert!(counts.settled(node), "the walk reached an unsettled node");
    let alternatives: Vec<(usize, usize)> = forest
        .node_alternatives(node)
        .enumerate()
        .filter(|&(index, _)| counts.node_family(node, index) > 0)
        .map(|(_, alternative)| alternative)
        .collect();
    let rest = match alternatives[..] {
        [(production, item)] => {
            let id = grammar.production_id(production);
            let mut children = Vec::new();
            let mut at = Some(item);
            let mut split = true;
            while let Some(i) = at {
                let families: Vec<(Option<usize>, ChildRef)> = forest
                    .item_families(i)
                    .enumerate()
                    .filter(|&(index, _)| counts.item_family(i, index) > 0)
                    .map(|(_, family)| family)
                    .collect();
                let [(previous, child)] = families[..] else {
                    split = false;
                    break;
                };
                let name =
                    grammar.production_functions(forest.item_production(i))[forest.item_dot(i) - 1];
                children.push((name, child));
                at = previous;
            }
            if split {
                children.reverse();
                SettledRest::Children {
                    production: id,
                    children: children
                        .into_iter()
                        .map(|(name, child)| {
                            let child = match child {
                                ChildRef::Node(c) => {
                                    SettledChild::Node(counted_node(parse, counts, c))
                                }
                                ChildRef::Leaf(l) => SettledChild::Leaf {
                                    word: sylex_syntax::ForkWord {
                                        start: forest.leaf_span(l).start,
                                        surface: String::new(),
                                        occurrence: 0,
                                    },
                                    span: forest.leaf_span(l),
                                    reading: forest.leaf_reading(l).clone(),
                                },
                                ChildRef::Punct => SettledChild::Punct,
                            };
                            (name, child)
                        })
                        .collect(),
                }
            } else {
                SettledRest::Splits { production: id }
            }
        }
        _ => SettledRest::Alternatives(alternatives.len()),
    };
    SettledNode {
        category: grammar.category_names()[forest.node_category(node)],
        span: forest.node_span(node),
        rest,
    }
}

/// The spine with each leaf's word reduced to its start, as the counted
/// spine names it.
fn by_start(spine: &SettledSpine) -> SettledSpine {
    fn node(n: &SettledNode) -> SettledNode {
        let rest = match &n.rest {
            SettledRest::Children {
                production,
                children,
            } => SettledRest::Children {
                production: *production,
                children: children
                    .iter()
                    .map(|(name, child)| {
                        let child = match child {
                            SettledChild::Node(c) => SettledChild::Node(node(c)),
                            SettledChild::Leaf {
                                word,
                                span,
                                reading,
                            } => SettledChild::Leaf {
                                word: sylex_syntax::ForkWord {
                                    start: word.start,
                                    surface: String::new(),
                                    occurrence: 0,
                                },
                                span: span.clone(),
                                reading: reading.clone(),
                            },
                            SettledChild::Punct => SettledChild::Punct,
                        };
                        (*name, child)
                    })
                    .collect(),
            },
            other => other.clone(),
        };
        SettledNode {
            category: n.category,
            span: n.span.clone(),
            rest,
        }
    }
    match spine {
        SettledSpine::Root(root) => SettledSpine::Root(node(root)),
        SettledSpine::Roots(n) => SettledSpine::Roots(*n),
    }
}

/// Whether every Reading builds `node` as the spine says, and the Readings
/// differ where it stops.
fn agrees(node: &SettledNode, readings: &[&Reading]) -> Result<(), String> {
    let nodes: Vec<_> = readings
        .iter()
        .map(|r| r.node().ok_or("a spine node is a leaf of a Reading"))
        .collect::<Result<_, _>>()?;
    if nodes.iter().any(|n| n.category != node.category) {
        return Err(format!(
            "a Reading's {:?} is no {}",
            node.span, node.category
        ));
    }
    let differ = || readings.iter().any(|r| r != &readings[0]);
    match &node.rest {
        SettledRest::Children {
            production,
            children,
        } => {
            if nodes.iter().any(|n| n.production != *production) {
                return Err(format!(
                    "a Reading's {} is not by {production}",
                    node.category
                ));
            }
            let built: Vec<&SettledChild> = children
                .iter()
                .map(|(_, child)| child)
                .filter(|child| !matches!(child, SettledChild::Punct))
                .collect();
            for (index, child) in built.iter().enumerate() {
                let below: Vec<&Reading> = nodes
                    .iter()
                    .map(|n| n.children.get(index).ok_or("a Reading lacks a child"))
                    .collect::<Result<_, _>>()?;
                match child {
                    SettledChild::Node(child) => agrees(child, &below)?,
                    SettledChild::Leaf { reading, .. } => {
                        if below.iter().any(|r| *r != &Reading::Leaf(reading.clone())) {
                            return Err(format!("a Reading's leaf differs from {reading:?}"));
                        }
                    }
                    SettledChild::Punct => unreachable!("punctuation is filtered out"),
                }
            }
            if nodes.iter().any(|n| n.children.len() != built.len()) {
                return Err(format!("a Reading's {} has other children", node.category));
            }
            Ok(())
        }
        SettledRest::Splits { production } => {
            if nodes.iter().any(|n| n.production != *production) {
                return Err(format!(
                    "a Reading's {} is not by {production}",
                    node.category
                ));
            }
            if differ() {
                Ok(())
            } else {
                Err(format!(
                    "the spine stops at {} where no Reading differs",
                    node.category
                ))
            }
        }
        SettledRest::Alternatives(_) => {
            if differ() {
                Ok(())
            } else {
                Err(format!(
                    "the spine stops at {} where no Reading differs",
                    node.category
                ))
            }
        }
    }
}

fn check_enumerated(forest: &ForkForest<'_>) -> Result<(), String> {
    let readings = forest
        .readings(ENUMERATED)
        .expect("checked only at or below the enumeration bound");
    let readings: Vec<&Reading> = readings.iter().collect();
    match forest.settled_spine() {
        SettledSpine::Root(root) => agrees(&root, &readings),
        SettledSpine::Roots(0) if readings.is_empty() => Ok(()),
        SettledSpine::Roots(n) if n > 1 && readings.iter().any(|r| r != &readings[0]) => Ok(()),
        SettledSpine::Roots(n) => Err(format!("{n} live roots over {} Readings", readings.len())),
    }
}

/// `settled_readings` gives the forest's Readings, each with a settled tree
/// that builds it and has no stop; a forest of one Derivation gives its
/// spine.
fn check_settled_readings(forest: &ForkForest<'_>) -> Result<(), String> {
    let mut readings = forest
        .readings(ENUMERATED)
        .expect("checked only at or below the enumeration bound");
    let mut settled = forest
        .settled_readings(ENUMERATED)
        .expect("checked only at or below the enumeration bound");
    if forest.total() > 0 && forest.settled_readings(forest.total() - 1).is_some() {
        return Err("`settled_readings` ignores its limit".to_owned());
    }
    for (reading, node) in &settled {
        agrees(node, &[reading])?;
    }
    if let [(_, node)] = &settled[..]
        && forest.settled_spine() != SettledSpine::Root(node.clone())
    {
        return Err("the one Derivation's tree is not the spine".to_owned());
    }
    readings.sort();
    settled.sort_by(|a, b| a.0.cmp(&b.0));
    let settled: Vec<Reading> = settled.into_iter().map(|(reading, _)| reading).collect();
    if readings == settled {
        Ok(())
    } else {
        Err("`settled_readings` are not the Readings".to_owned())
    }
}

#[test]
fn the_settled_spine_agrees_with_the_counts_and_the_readings() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let corpus = Corpus::load(|face| SAMPLE.contains(&face.name.as_str())).unwrap();
    let mut failures = Vec::new();
    let mut several_roots = false;
    let mut whole_trees = 0;
    for (face, text) in unit_texts::texts(&corpus, &env) {
        let name = |why: String| format!("{} | {text}: {why}", face.name);
        let analyzed = env
            .lexicon
            .analyze_source_scoped(&text, None, &face.scope());
        let parse = grammar.parse_analyzed(&analyzed);
        let counts = parse.counts().unwrap();
        let forest = parse.fork_forest().unwrap();
        let spine = forest.settled_spine();
        if by_start(&spine) != counted_spine(&parse, &counts) {
            failures.push(name("the spine is not the counted walk".to_owned()));
        }
        several_roots |= matches!(spine, SettledSpine::Roots(n) if n > 1);
        let mut narrowed = forest;
        loop {
            if narrowed.total() <= ENUMERATED
                && let Err(why) = check_enumerated(&narrowed).and(check_settled_readings(&narrowed))
            {
                failures.push(name(why));
                break;
            }
            let forks = narrowed.forks();
            let Some(fork) = forks.first() else {
                break;
            };
            narrowed = narrowed
                .pick(fork.kind, &fork.word, &fork.options[0].answer)
                .unwrap();
        }
        if narrowed.total() == 1 {
            whole_trees += 1;
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert!(several_roots, "Decimate has more than one live root");
    assert!(whole_trees > 0);
}

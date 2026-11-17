//! The settled spine of a [`ForkForest`]: its settled Constituents from the
//! root down, as far as every Derivation builds them alike.
//!
//! A settled node with several alternatives is still a point of choice
//! ([`crate::Counts::settled`]): the spine names it and stops there. A node
//! with one alternative whose children split its span in more than one way
//! stops it too. Below a node with one alternative and one split, every
//! child is settled, so the spine is a tree of settled nodes.
//!
//! A narrowed forest of few Derivations also gives each Derivation as such a
//! tree ([`ForkForest::settled_readings`]), so one printer serves both.

use std::collections::HashMap;
use std::ops::Range;

use dpsi::Ident;
use sylex_lexicon::LexicalReading;

use crate::chart::ChildRef;
use crate::fork_graph::PackedCounts;
use crate::fork_graph::Tree;
use crate::forks::ForkForest;
use crate::forks::ForkWord;
use crate::reading::Reading;

/// The settled spine of a forest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettledSpine {
    /// The forest's one root node with Derivations.
    Root(SettledNode),
    /// No root node is settled: this many have Derivations (none when the
    /// forest has no Derivation).
    Roots(usize),
}

/// A settled node: one every Derivation of the forest uses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettledNode {
    pub category: Ident,
    /// Its extent, in chars of the analyzed text.
    pub span: Range<usize>,
    pub rest: SettledRest,
}

/// How far every Derivation builds a settled node alike.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettledRest {
    /// By one Production and one split into children: the children, in
    /// order, each with its name in the Production.
    Children {
        production: Ident,
        children: Vec<(Ident, SettledChild)>,
    },
    /// By one Production, whose children split the span in more than one
    /// way.
    Splits { production: Ident },
    /// By this many alternatives (each a Production and completed item).
    Alternatives(usize),
}

/// A child below a settled node with one alternative and one split.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettledChild {
    Node(SettledNode),
    /// A leaf: the word it starts at, its extent in chars and its lexical
    /// reading.
    Leaf {
        word: ForkWord,
        span: Range<usize>,
        reading: LexicalReading,
    },
}

impl ForkForest<'_> {
    /// The settled spine (see the module doc).
    ///
    /// # Panics
    /// When a node the spine reaches is not used by every Derivation, which
    /// would be a defect of the counts.
    #[must_use]
    pub fn settled_spine(&self) -> SettledSpine {
        let counts = self.graph.counts();
        let packed = self.graph.packed_counts(&counts);
        let mut roots: Vec<usize> = packed.roots.keys().copied().collect();
        roots.sort_unstable();
        match roots[..] {
            [root] => SettledSpine::Root(self.settled(root, &packed, counts.total)),
            _ => SettledSpine::Roots(roots.len()),
        }
    }

    /// The forest's Readings, each with its tree as a settled node (every
    /// node of it has the one Production and split its Derivation has), or
    /// `None` when the forest has more than `limit` Derivations.
    #[must_use]
    pub fn settled_readings(&self, limit: u128) -> Option<Vec<(Reading, SettledNode)>> {
        (self.total() <= limit).then(|| {
            let mut features = HashMap::new();
            self.graph
                .derivation_trees(self.parse)
                .iter()
                .map(|tree| {
                    (
                        tree.reading(self.parse, &mut features),
                        self.settled_tree(tree),
                    )
                })
                .collect()
        })
    }

    fn settled_tree(&self, tree: &Tree) -> SettledNode {
        let Tree::Node {
            node,
            production,
            children,
        } = tree
        else {
            panic!("a Derivation's root and its nodes are nodes");
        };
        let grammar = self.parse.grammar;
        let packed_node = &self.parse.forest.nodes[*node];
        let mut named = Vec::new();
        let mut at = children.as_deref();
        while let Some(sequence) = at {
            let child = match &*sequence.last {
                tree @ Tree::Node { .. } => SettledChild::Node(self.settled_tree(tree)),
                Tree::Leaf(leaf) => self.leaf(*leaf),
            };
            named.push((sequence.name, child));
            at = sequence.prefix.as_deref();
        }
        named.reverse();
        SettledNode {
            category: grammar.categories[packed_node.category],
            span: packed_node.start..packed_node.end,
            rest: SettledRest::Children {
                production: grammar.production_id(*production),
                children: named,
            },
        }
    }

    fn settled(&self, node: usize, packed: &PackedCounts, total: u128) -> SettledNode {
        let parse = self.parse;
        let grammar = parse.grammar;
        let forest = &parse.forest;
        let packed_node = &forest.nodes[node];
        let category = grammar.categories[packed_node.category];
        let span = packed_node.start..packed_node.end;
        let live: Vec<(usize, usize, u128)> = packed_node
            .families
            .iter()
            .filter_map(|&(production, item)| {
                packed
                    .alternatives
                    .get(&(node, item))
                    .map(|&n| (production, item, n))
            })
            .collect();
        let rest = match live[..] {
            [] => panic!("the settled {category} at {span:?} has no alternative with Derivations"),
            [(production, item, n)] => {
                assert_eq!(
                    n, total,
                    "the spine reached {category} at {span:?}, which not every Derivation uses"
                );
                let id = grammar.production_id(production);
                match self.split(item, packed) {
                    Some(children) => SettledRest::Children {
                        production: id,
                        children: children
                            .into_iter()
                            .map(|(name, child)| (name, self.child(child, packed, total)))
                            .collect(),
                    },
                    None => SettledRest::Splits { production: id },
                }
            }
            _ => SettledRest::Alternatives(live.len()),
        };
        SettledNode {
            category,
            span,
            rest,
        }
    }

    /// A completed item's children with their names, when one split has
    /// Derivations.
    fn split(&self, item: usize, packed: &PackedCounts) -> Option<Vec<(Ident, ChildRef)>> {
        let forest = &self.parse.forest;
        let mut children = Vec::new();
        let mut at = Some(item);
        while let Some(i) = at {
            let partial = &forest.items[i];
            let live: Vec<(Option<usize>, ChildRef)> = partial
                .families
                .iter()
                .copied()
                .filter(|&(previous, child)| packed.families.contains_key(&(i, previous, child)))
                .collect();
            let [(previous, child)] = live[..] else {
                return None;
            };
            let name =
                self.parse.grammar.productions[partial.production].names[self.parse.item_child(i)];
            children.push((name, child));
            at = previous;
        }
        children.reverse();
        Some(children)
    }

    fn child(&self, child: ChildRef, packed: &PackedCounts, total: u128) -> SettledChild {
        match child {
            ChildRef::Node(c) => SettledChild::Node(self.settled(c, packed, total)),
            ChildRef::Leaf(l) => self.leaf(l),
        }
    }

    fn leaf(&self, leaf: usize) -> SettledChild {
        let leaf = &self.parse.forest.leaves[leaf];
        let word = self
            .unit
            .words
            .iter()
            .find(|w| w.start == leaf.start)
            .expect("a leaf some Derivation uses starts a word");
        SettledChild::Leaf {
            word: word.clone(),
            span: leaf.start..leaf.end,
            reading: leaf.reading.clone(),
        }
    }
}

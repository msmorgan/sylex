//! Exact Derivation counts over a Packed Forest: for every node, item and
//! edge, the number of the unit's Derivations that use it.
//!
//! The inside count of a node or item is the number of its own Derivations;
//! its outside count is the number of ways the rest of a whole Derivation can
//! surround it. Their product is the number of the unit's Derivations that
//! use it, because no node, item or edge occurs twice in one Derivation:
//!
//! - No Constituent spans nothing. A period may be unwritten and match with
//!   zero width; the compiler refuses a Production whose first child is one
//!   (`Binder::no_zero_width_start`), and the chart asserts that every lexical
//!   alternative spans at least one char. So two occurrences of one node would
//!   have the same nonempty span and could not be disjoint.
//! - No partial Production past its first child spans nothing, by the same
//!   refusal: its first child spans at least one char. So two occurrences of
//!   one such item would have the same nonempty span too, and an edge occurs
//!   once per occurrence of its item.
//! - The forest has no unary cycle, so nested occurrences of a node or item
//!   cannot happen either. A grammar may contain a cycle of span-preserving
//!   Productions, closed or kept open only by features, so [`Parse::counts`]
//!   checks the forest it is given and returns a [`ForestCycle`] naming the
//!   cycle rather than a wrong count.
//!
//! The counts are of Derivations. Two Derivations that build the same tree
//! would be one Reading.

use crate::chart::ChildRef;
use crate::chart::PackedForest;
use crate::chart::Parse;

/// The exact Derivation counts of one Parse (see the module doc).
#[derive(Debug, Clone)]
pub struct Counts<'p> {
    forest: &'p PackedForest,
    total: u128,
    node_inside: Vec<u128>,
    node_outside: Vec<u128>,
    item_inside: Vec<u128>,
    item_outside: Vec<u128>,
}

/// A unary cycle among the nodes a unit's roots reach: a Constituent that
/// could contain itself. Its message names each step's Category, span (in
/// chars) and Production.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unary cycle in the Packed Forest: {0}")]
pub struct ForestCycle(String);

/// A forest vertex.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Vertex {
    Node(usize),
    Item(usize),
}

impl Parse<'_> {
    /// The exact Derivation counts of every node, item and edge, by an inside
    /// pass and a top-down outside pass over the nodes the roots reach.
    ///
    /// # Errors
    ///
    /// A [`ForestCycle`] when the reachable forest has a unary cycle.
    ///
    /// # Panics
    ///
    /// When a count overflows `u128`.
    pub fn counts(&self) -> Result<Counts<'_>, ForestCycle> {
        let forest = &self.forest;
        let order = self.children_first()?;
        let mut node_inside = vec![0u128; forest.nodes.len()];
        let mut item_inside = vec![0u128; forest.items.len()];
        for &vertex in &order {
            match vertex {
                Vertex::Node(node) => {
                    node_inside[node] = forest.nodes[node]
                        .families
                        .iter()
                        .map(|&(_, item)| item_inside[item])
                        .fold(0, add);
                }
                Vertex::Item(item) => {
                    item_inside[item] = forest.items[item]
                        .families
                        .iter()
                        .map(|&(previous, child)| {
                            mul(
                                previous.map_or(1, |p| item_inside[p]),
                                child_inside(&node_inside, child),
                            )
                        })
                        .fold(0, add);
                }
            }
        }
        let mut node_outside = vec![0u128; forest.nodes.len()];
        let mut item_outside = vec![0u128; forest.items.len()];
        for &root in &forest.roots {
            node_outside[root] = 1;
        }
        for &vertex in order.iter().rev() {
            match vertex {
                Vertex::Node(node) => {
                    let outside = node_outside[node];
                    for &(_, item) in &forest.nodes[node].families {
                        item_outside[item] = add(item_outside[item], outside);
                    }
                }
                Vertex::Item(item) => {
                    let outside = item_outside[item];
                    for &(previous, child) in &forest.items[item].families {
                        if let Some(p) = previous {
                            item_outside[p] = add(
                                item_outside[p],
                                mul(outside, child_inside(&node_inside, child)),
                            );
                        }
                        if let ChildRef::Node(c) = child {
                            let before = previous.map_or(1, |p| item_inside[p]);
                            node_outside[c] = add(node_outside[c], mul(outside, before));
                        }
                    }
                }
            }
        }
        let total = forest.roots.iter().map(|&r| node_inside[r]).fold(0, add);
        Ok(Counts {
            forest,
            total,
            node_inside,
            node_outside,
            item_inside,
            item_outside,
        })
    }

    /// The nodes and items the roots reach, each after everything it
    /// derives from, by an iterative depth-first search.
    pub(crate) fn children_first(&self) -> Result<Vec<Vertex>, ForestCycle> {
        #[derive(Clone, Copy, PartialEq)]
        enum Mark {
            New,
            Open,
            Done,
        }
        let forest = &self.forest;
        let mut nodes = vec![Mark::New; forest.nodes.len()];
        let mut items = vec![Mark::New; forest.items.len()];
        let mut order = Vec::new();
        for &root in &forest.roots {
            if nodes[root] != Mark::New {
                continue;
            }
            nodes[root] = Mark::Open;
            // Each frame: a vertex and the index of its next successor.
            let mut stack = vec![(Vertex::Node(root), 0usize)];
            while let Some(top) = stack.last_mut() {
                let (vertex, next) = (top.0, top.1);
                let Some(successor) = self.successor(vertex, next) else {
                    match vertex {
                        Vertex::Node(n) => nodes[n] = Mark::Done,
                        Vertex::Item(i) => items[i] = Mark::Done,
                    }
                    order.push(vertex);
                    stack.pop();
                    continue;
                };
                top.1 += 1;
                let mark = match successor {
                    Vertex::Node(n) => &mut nodes[n],
                    Vertex::Item(i) => &mut items[i],
                };
                match *mark {
                    Mark::New => {
                        *mark = Mark::Open;
                        stack.push((successor, 0));
                    }
                    Mark::Open => {
                        let from = stack
                            .iter()
                            .position(|&(v, _)| v == successor)
                            .expect("an open vertex is on the stack");
                        return Err(self.cycle(&stack[from..]));
                    }
                    Mark::Done => {}
                }
            }
        }
        Ok(order)
    }

    /// The `index`th vertex a vertex derives from: a node's items, then an
    /// item's previous items and child nodes, family by family.
    fn successor(&self, vertex: Vertex, index: usize) -> Option<Vertex> {
        match vertex {
            Vertex::Node(node) => self.forest.nodes[node]
                .families
                .get(index)
                .map(|&(_, item)| Vertex::Item(item)),
            Vertex::Item(item) => self.forest.items[item]
                .families
                .iter()
                .flat_map(|&(previous, child)| {
                    let child = match child {
                        ChildRef::Node(n) => Some(Vertex::Node(n)),
                        ChildRef::Leaf(_) | ChildRef::Punct => None,
                    };
                    previous.map(Vertex::Item).into_iter().chain(child)
                })
                .nth(index),
        }
    }

    /// Names a cycle: each node with its Category and span, and each item
    /// with its Production and span.
    fn cycle(&self, stack: &[(Vertex, usize)]) -> ForestCycle {
        let steps: Vec<String> = stack
            .iter()
            .map(|&(vertex, _)| match vertex {
                Vertex::Node(n) => {
                    let node = &self.forest.nodes[n];
                    format!(
                        "{} {}..{}",
                        self.grammar.categories[node.category], node.start, node.end
                    )
                }
                Vertex::Item(i) => {
                    let item = &self.forest.items[i];
                    format!(
                        "`{}` {}..{}",
                        self.grammar.productions[item.production].id, item.start, item.end
                    )
                }
            })
            .collect();
        ForestCycle(steps.join(" -> "))
    }
}

impl Counts<'_> {
    /// The unit's Derivations.
    #[must_use]
    pub fn total(&self) -> u128 {
        self.total
    }

    /// The Derivations that use a node.
    #[must_use]
    pub fn node(&self, node: usize) -> u128 {
        mul(self.node_inside[node], self.node_outside[node])
    }

    /// The ways the rest of a whole Derivation can surround a node.
    #[must_use]
    pub fn node_outside(&self, node: usize) -> u128 {
        self.node_outside[node]
    }

    /// An item's own Derivations: the ways to build its children.
    #[must_use]
    pub fn item_inside(&self, item: usize) -> u128 {
        self.item_inside[item]
    }

    /// Whether a node is in every Derivation of a unit that has one. A
    /// settled node with several alternatives is still a point of choice:
    /// its children vary independently only within one alternative.
    #[must_use]
    pub fn settled(&self, node: usize) -> bool {
        self.total > 0 && self.node(node) == self.total
    }

    /// The Derivations that use an item.
    #[must_use]
    pub fn item(&self, item: usize) -> u128 {
        mul(self.item_inside[item], self.item_outside[item])
    }

    /// The Derivations that use a node's `index`th (Production, item)
    /// alternative, in the order of [`PackedForest::node_items`].
    #[must_use]
    pub fn node_family(&self, node: usize, index: usize) -> u128 {
        let (_, item) = self.forest.nodes[node].families[index];
        mul(self.node_outside[node], self.item_inside[item])
    }

    /// The Derivations that use an item's `index`th (previous item, child)
    /// family, in the order of [`PackedForest::item_families`].
    #[must_use]
    pub fn item_family(&self, item: usize, index: usize) -> u128 {
        let (previous, child) = self.forest.items[item].families[index];
        mul(
            mul(
                self.item_outside[item],
                previous.map_or(1, |p| self.item_inside[p]),
            ),
            child_inside(&self.node_inside, child),
        )
    }
}

fn child_inside(node_inside: &[u128], child: ChildRef) -> u128 {
    match child {
        ChildRef::Node(n) => node_inside[n],
        ChildRef::Leaf(_) | ChildRef::Punct => 1,
    }
}

fn add(a: u128, b: u128) -> u128 {
    a.checked_add(b).expect("a Derivation count overflows u128")
}

fn mul(a: u128, b: u128) -> u128 {
    a.checked_mul(b).expect("a Derivation count overflows u128")
}

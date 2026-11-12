//! Facts of a Packed Forest's alternatives, each split exactly over the
//! alternative's Derivations: its head word, its conjunct count and its
//! Embedding.
//!
//! A node's alternative is a (Production, completed item) pair, and one
//! completed item packs prefixes whose children differ, so one alternative
//! can reach more than one head word, and one node can stand under more than
//! one Embedding. Each fact is therefore a distribution.
//!
//! A head word and a conjunct count are settled inside the alternative: their
//! distribution splits the alternative's own Derivations (its completed
//! item's inside count, [`crate::Counts::item_inside`]). An Embedding is
//! settled outside it: its distribution splits the ways to surround the node
//! ([`crate::Counts::node_outside`]). The Derivations of the unit that use an
//! alternative with a head word and an Embedding are the product of the two
//! entries, since what is inside a node and what surrounds it vary
//! independently.

use std::collections::BTreeMap;

use dpsi::Ident;
use sylex_lexicon::LexemeId;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::WordForm;

use crate::chart::ChildRef;
use crate::chart::Parse;
use crate::counts::ForestCycle;
use crate::counts::Vertex;

/// A head word: the lexeme and Word Form of the leaf reached by following
/// `head` children down, as [`crate::CompiledGrammar::head_word`] follows
/// them in a Reading.
pub type HeadWord = (LexemeId, WordForm);

/// The number of [`EmbeddingStep`]s an Embedding keeps, nearest first.
pub const EMBEDDING_DEPTH: usize = 3;

/// One dependent function a Constituent stands under: going up from it
/// through `head` children to the top of its head line, the Production
/// there, and the function that projection bears in its governor.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmbeddingStep {
    /// The Production at the top of the head line: the alternative of the
    /// dependent itself.
    pub projection: usize,
    /// The dependent's function in the governor: its child name.
    pub function: Ident,
    /// The governor's Production.
    pub governor: usize,
    /// The governor's head word, if it has one.
    pub governor_head: Option<HeadWord>,
}

/// The Embedding of an occurrence of a Constituent: its [`EmbeddingStep`]s,
/// nearest first, at most [`EMBEDDING_DEPTH`] of them. A root's own head
/// line stands under none.
pub type Embedding = Vec<EmbeddingStep>;

/// For every node, for each of its alternatives (in the order of
/// [`crate::PackedForest::node_alternatives`]), a distribution: a count for
/// each value (see the module doc for what it splits).
pub type AlternativeDistribution<K> = Vec<Vec<BTreeMap<K, u128>>>;

type Map<K> = BTreeMap<K, u128>;

/// The inside distribution of every node and item.
struct Inside<K> {
    nodes: Vec<Map<K>>,
    items: Vec<Map<K>>,
}

/// An Embedding step whose projection is not reached yet: the dependent's
/// own alternative fills it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Pending {
    function: Ident,
    governor: usize,
    governor_head: Option<HeadWord>,
}

/// What the outside pass carries to a node: its Embedding so far, and the
/// step its own alternative completes when it is a dependent's top.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Above {
    steps: Embedding,
    pending: Option<Pending>,
}

impl Parse<'_> {
    /// The head words of each alternative ([`HeadWord`]), split over its own
    /// Derivations; `None` where the head line ends in no word, or there is
    /// no `head` child.
    ///
    /// # Errors
    /// A [`ForestCycle`] when the reachable forest has a unary cycle.
    pub fn alternative_heads(
        &self,
    ) -> Result<AlternativeDistribution<Option<HeadWord>>, ForestCycle> {
        let inside = self.head_inside(&self.children_first()?);
        Ok(self.per_alternative(&inside))
    }

    /// The conjunct count of each alternative, split over its own
    /// Derivations: its children named `first` or `last`, its idiomatic
    /// `coordinate` (*and so on*), and those of the serial or polysyndetic
    /// tails it continues in through `rest`.
    ///
    /// # Errors
    /// A [`ForestCycle`] when the reachable forest has a unary cycle.
    pub fn alternative_conjuncts(&self) -> Result<AlternativeDistribution<usize>, ForestCycle> {
        let grammar = self.grammar;
        let inside = self.inside(
            &self.children_first()?,
            0,
            |_| 0,
            |production, at, before, child| match grammar.productions[production].names[at].as_str()
            {
                "first" | "last" | "coordinate" => before + 1,
                "rest" => before + child,
                _ => *before,
            },
        );
        Ok(self.per_alternative(&inside))
    }

    /// The [`Embedding`]s of each alternative, split over the ways to
    /// surround its node.
    ///
    /// # Errors
    /// A [`ForestCycle`] when the reachable forest has a unary cycle.
    ///
    /// # Panics
    /// When a count overflows `u128`.
    pub fn alternative_embeddings(
        &self,
    ) -> Result<AlternativeDistribution<Embedding>, ForestCycle> {
        let order = self.children_first()?;
        let heads = self.head_inside(&order);
        let forest = &self.forest;
        let grammar = self.grammar;
        let item_inside = |item: Option<usize>| item.map_or(1, |i| total(&heads.items[i]));
        let child_inside = |child: ChildRef| match child {
            ChildRef::Node(n) => total(&heads.nodes[n]),
            ChildRef::Leaf(_) | ChildRef::Punct => 1,
        };
        let child_heads = |child: ChildRef| -> Map<Option<HeadWord>> {
            match child {
                ChildRef::Node(n) => heads.nodes[n].clone(),
                leaf => BTreeMap::from([(self.leaf_head(leaf), 1)]),
            }
        };
        let mut result: AlternativeDistribution<Embedding> = forest
            .nodes
            .iter()
            .map(|node| vec![BTreeMap::new(); node.families.len()])
            .collect();
        let mut above: Vec<BTreeMap<Above, u128>> = vec![BTreeMap::new(); forest.nodes.len()];
        // Each item's ways to be surrounded, by its Embedding and by the head
        // word its suffix holds when the head comes after its dot.
        let mut item_above: Vec<BTreeMap<(Embedding, Option<HeadWord>), u128>> =
            vec![BTreeMap::new(); forest.items.len()];
        for &root in &forest.roots {
            above[root].insert(
                Above {
                    steps: Vec::new(),
                    pending: None,
                },
                1,
            );
        }
        for &vertex in order.iter().rev() {
            match vertex {
                Vertex::Node(node) => {
                    for (index, &(production, item)) in
                        forest.nodes[node].families.iter().enumerate()
                    {
                        for (label, &ways) in &above[node] {
                            let steps = complete(label, production);
                            result[node][index].accumulate(steps.clone(), ways);
                            item_above[item].accumulate((steps, None), ways);
                        }
                    }
                }
                Vertex::Item(item) => {
                    let it = &forest.items[item];
                    let production = it.production;
                    let head = grammar.head_index(production);
                    let at = it.dot - 1;
                    let function = grammar.productions[production].names[at];
                    let surrounding = std::mem::take(&mut item_above[item]);
                    for &(previous, child) in &it.families {
                        for ((steps, suffix_head), &ways) in &surrounding {
                            if let Some(p) = previous {
                                let suffix: Map<Option<HeadWord>> = match head {
                                    Some(h) if h == at => child_heads(child),
                                    Some(h) if h > at => {
                                        BTreeMap::from([(*suffix_head, child_inside(child))])
                                    }
                                    _ => BTreeMap::from([(None, child_inside(child))]),
                                };
                                for (word, n) in suffix {
                                    item_above[p].accumulate((steps.clone(), word), mul(ways, n));
                                }
                            }
                            let ChildRef::Node(c) = child else { continue };
                            if head == Some(at) {
                                let label = Above {
                                    steps: steps.clone(),
                                    pending: None,
                                };
                                above[c].accumulate(label, mul(ways, item_inside(previous)));
                                continue;
                            }
                            let governor_heads: Map<Option<HeadWord>> = match head {
                                Some(h) if h < at => {
                                    let p = previous
                                        .expect("a child after the head has a previous item");
                                    heads.items[p].clone()
                                }
                                Some(_) => BTreeMap::from([(*suffix_head, item_inside(previous))]),
                                None => BTreeMap::from([(None, item_inside(previous))]),
                            };
                            for (governor_head, n) in governor_heads {
                                let label = Above {
                                    steps: steps.clone(),
                                    pending: Some(Pending {
                                        function,
                                        governor: production,
                                        governor_head,
                                    }),
                                };
                                above[c].accumulate(label, mul(ways, n));
                            }
                        }
                    }
                }
            }
        }
        Ok(result)
    }

    /// The inside distribution of head words.
    fn head_inside(&self, order: &[Vertex]) -> Inside<Option<HeadWord>> {
        let grammar = self.grammar;
        self.inside(
            order,
            None,
            |leaf| self.leaf_head(leaf),
            |production, at, before, child| {
                if grammar.head_index(production) == Some(at) { *child } else { *before }
            },
        )
    }

    /// The head word a leaf child is: a word's lexeme and form.
    fn leaf_head(&self, child: ChildRef) -> Option<HeadWord> {
        match child {
            ChildRef::Leaf(leaf) => match &self.forest.leaves[leaf].reading {
                LexicalReading::Word(value) => Some((value.lexeme, value.form)),
                LexicalReading::FlavorWord { .. } | LexicalReading::Numeral { .. } => None,
            },
            ChildRef::Punct => None,
            ChildRef::Node(_) => unreachable!("a node child is no leaf"),
        }
    }

    /// The inside distribution of a value each item settles from its
    /// previous item's value and its last child's: `start` before the first
    /// child, `leaf` for a leaf or punctuation child, and `step(production,
    /// child index, previous value, child value)` for each family.
    fn inside<K: Ord + Clone>(
        &self,
        order: &[Vertex],
        start: K,
        leaf: impl Fn(ChildRef) -> K,
        step: impl Fn(usize, usize, &K, &K) -> K,
    ) -> Inside<K> {
        let forest = &self.forest;
        let mut inside: Inside<K> = Inside {
            nodes: vec![Map::new(); forest.nodes.len()],
            items: vec![Map::new(); forest.items.len()],
        };
        let origin = BTreeMap::from([(start, 1)]);
        for &vertex in order {
            match vertex {
                Vertex::Node(node) => {
                    let mut map = Map::new();
                    for &(_, item) in &forest.nodes[node].families {
                        for (key, &n) in &inside.items[item] {
                            map.accumulate(key.clone(), n);
                        }
                    }
                    inside.nodes[node] = map;
                }
                Vertex::Item(item) => {
                    let it = &forest.items[item];
                    let at = it.dot - 1;
                    let mut map = Map::new();
                    for &(previous, child) in &it.families {
                        let before = previous.map_or(&origin, |p| &inside.items[p]);
                        let leaf_map;
                        let here = match child {
                            ChildRef::Node(n) => &inside.nodes[n],
                            other => {
                                leaf_map = BTreeMap::from([(leaf(other), 1)]);
                                &leaf_map
                            }
                        };
                        for (b, &nb) in before {
                            for (h, &nh) in here {
                                map.accumulate(step(it.production, at, b, h), mul(nb, nh));
                            }
                        }
                    }
                    inside.items[item] = map;
                }
            }
        }
        inside
    }

    /// Each alternative's completed item's inside distribution.
    fn per_alternative<K: Ord + Clone>(&self, inside: &Inside<K>) -> AlternativeDistribution<K> {
        self.forest
            .nodes
            .iter()
            .map(|packed| {
                packed
                    .families
                    .iter()
                    .map(|&(_, item)| inside.items[item].clone())
                    .collect()
            })
            .collect()
    }
}

/// A node's Embedding once its alternative is `production`: the pending
/// step, if any, completed and pushed in front, keeping the nearest
/// [`EMBEDDING_DEPTH`].
fn complete(label: &Above, production: usize) -> Embedding {
    let Some(pending) = &label.pending else {
        return label.steps.clone();
    };
    std::iter::once(EmbeddingStep {
        projection: production,
        function: pending.function,
        governor: pending.governor,
        governor_head: pending.governor_head,
    })
    .chain(label.steps.iter().cloned())
    .take(EMBEDDING_DEPTH)
    .collect()
}

fn total<K>(map: &Map<K>) -> u128 {
    map.values().copied().fold(0, add)
}

/// Adds Derivations to a distribution.
trait Accumulate<K> {
    fn accumulate(&mut self, key: K, n: u128);
}

impl<K: Ord> Accumulate<K> for Map<K> {
    fn accumulate(&mut self, key: K, n: u128) {
        let slot = self.entry(key).or_insert(0);
        *slot = add(*slot, n);
    }
}

fn add(a: u128, b: u128) -> u128 {
    a.checked_add(b).expect("a Derivation count overflows u128")
}

fn mul(a: u128, b: u128) -> u128 {
    a.checked_mul(b).expect("a Derivation count overflows u128")
}

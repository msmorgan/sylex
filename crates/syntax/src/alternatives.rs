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

/// A head leaf: the leaf reached by following `head` children down, by its
/// index in the forest; `None` where a `head` child is punctuation or there
/// is no `head` child.
pub type HeadLeaf = Option<usize>;

/// One dependent function a Constituent stands under: going up from it
/// through `head` children to the top of its head line, the Production
/// there, and the function that projection bears in its governor.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmbeddingStep {
    /// The Production at the top of the head line: the alternative of the
    /// dependent itself.
    pub projection: usize,
    /// The dependent's position among the governor's children.
    pub child: usize,
    /// The dependent's function in the governor: its child name.
    pub function: Ident,
    /// The governor's Production.
    pub governor: usize,
    /// The governor's head word, if it has one.
    pub governor_head: Option<HeadWord>,
}

/// The Embedding of an occurrence of a Constituent: its [`EmbeddingStep`]s,
/// nearest first, up to and including the `depth`th step that is no wrapper
/// ([`Parse::alternative_embeddings`]). A root's own head line stands under
/// none.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Embedding {
    pub steps: Vec<EmbeddingStep>,
    /// Steps above the kept ones were cut.
    pub truncated: bool,
}

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
    child: usize,
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
    /// The head leaf of each alternative ([`HeadLeaf`]), split over its own
    /// Derivations.
    ///
    /// # Errors
    /// A [`ForestCycle`] when the reachable forest has a unary cycle.
    pub fn alternative_heads(&self) -> Result<AlternativeDistribution<HeadLeaf>, ForestCycle> {
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
    /// surround its node. Each keeps its steps up to the `depth`th one for
    /// which `wrapper` does not hold: a wrapper step (such as a passive
    /// auxiliary's Complement, or a conjunct of a coordination) is kept but
    /// not counted.
    ///
    /// # Errors
    /// A [`ForestCycle`] when the reachable forest has a unary cycle.
    ///
    /// # Panics
    /// When a count overflows `u128`.
    pub fn alternative_embeddings(
        &self,
        depth: usize,
        wrapper: impl Fn(&EmbeddingStep) -> bool,
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
        let child_heads = |child: ChildRef| -> Map<HeadLeaf> {
            match child {
                ChildRef::Node(n) => heads.nodes[n].clone(),
                leaf => BTreeMap::from([(head_leaf(leaf), 1)]),
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
        let mut item_above: Vec<BTreeMap<(Embedding, HeadLeaf), u128>> =
            vec![BTreeMap::new(); forest.items.len()];
        for &root in &forest.roots {
            above[root].insert(
                Above {
                    steps: Embedding::default(),
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
                            let steps = complete(label, production, depth, &wrapper);
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
                                let suffix: Map<HeadLeaf> = match head {
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
                            let governor_heads: Map<HeadLeaf> = match head {
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
                                        child: at,
                                        function,
                                        governor: production,
                                        governor_head: self.leaf_word(governor_head),
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

    /// The inside distribution of head leaves.
    fn head_inside(&self, order: &[Vertex]) -> Inside<HeadLeaf> {
        let grammar = self.grammar;
        self.inside(order, None, head_leaf, |production, at, before, child| {
            if grammar.head_index(production) == Some(at) { *child } else { *before }
        })
    }

    /// The head word a head leaf is: a word's lexeme and form.
    fn leaf_word(&self, leaf: HeadLeaf) -> Option<HeadWord> {
        match &self.forest.leaves[leaf?].reading {
            LexicalReading::Word(value) => Some((value.lexeme, value.form)),
            LexicalReading::FlavorWord { .. } | LexicalReading::Numeral { .. } => None,
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

/// The head leaf a leaf or punctuation child is.
fn head_leaf(child: ChildRef) -> HeadLeaf {
    match child {
        ChildRef::Leaf(leaf) => Some(leaf),
        ChildRef::Punct => None,
        ChildRef::Node(_) => unreachable!("a node child is no leaf"),
    }
}

/// A node's Embedding once its alternative is `production`: the pending
/// step, if any, completed and pushed in front, keeping the steps up to the
/// `depth`th that is no wrapper.
fn complete(
    label: &Above,
    production: usize,
    depth: usize,
    wrapper: &impl Fn(&EmbeddingStep) -> bool,
) -> Embedding {
    let Some(pending) = &label.pending else {
        return label.steps.clone();
    };
    let mut steps = vec![EmbeddingStep {
        projection: production,
        child: pending.child,
        function: pending.function,
        governor: pending.governor,
        governor_head: pending.governor_head,
    }];
    let mut counted = usize::from(!wrapper(&steps[0]));
    let mut truncated = label.steps.truncated;
    for step in &label.steps.steps {
        if counted == depth {
            truncated = true;
            break;
        }
        counted += usize::from(!wrapper(step));
        steps.push(step.clone());
    }
    Embedding { steps, truncated }
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

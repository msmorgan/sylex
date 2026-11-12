//! Forks: where a unit's Derivations differ, read off the forest.
//!
//! A fork is a question about one word of a unit whose answer differs
//! between the unit's Derivations; each option is one answer, with the exact
//! number of Derivations that give it. Its context is the Derivations in which
//! the word has an answer, which its options partition; a fork whose context
//! is not every Derivation is conditional. A word's head line runs from its
//! leaf up through each mother of which it is the head-line child
//! ([`CompiledGrammar::head_line_child`]); its top is the word's maximal
//! projection. The questions ([`ForkKind`]):
//!
//! - lexical: which lexical alternative the leaf starting at the word is;
//! - dependency: what the maximal projection depends on (its function, its
//!   governor's Category and head word), or that it is a root;
//! - frame: the frame Productions of the head line
//!   ([`CompiledGrammar::is_frame_production`]), kept only where the lexical
//!   and dependency questions do not already determine them: a frame question
//!   whose partition of its context is a coarsening of the meet of the lexical
//!   and dependency questions' partitions there is no fork.
//!
//! Questions that induce one partition are one fork, named by the first in
//! display order (word position, then kind) with the others as aliases.
//!
//! The forks are read off a copy of the forest split by head
//! (`fork_graph.rs`), on which every answer is a set of edges: excluding an
//! answer removes its edges, and picking one keeps the Derivations that use
//! one of them. Both give a [`ForkForest`] whose counts and forks are
//! computed by the same code, under the unit's names.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use dpsi::Ident;
use sylex_lexicon::LexicalReading;

use crate::chart::Parse;
use crate::counts::ForestCycle;
use crate::fork_graph::Dep;
use crate::fork_graph::Edge;
pub use crate::fork_graph::ForkWord;
use crate::fork_graph::Graph;
use crate::fork_graph::Unit;
use crate::reading::Reading;

/// The question a fork asks about its word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ForkKind {
    /// Which lexical alternative the leaf starting at the word is.
    Lexical,
    /// What the word's maximal projection depends on.
    Dependency,
    /// The frame Productions of the word's head line.
    Frame,
}

/// One answer to a fork's question, named by its grammatical content.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Answer {
    /// The leaf's lexical reading (its Lexeme, Word Form and features),
    /// which also fixes how far it reaches.
    Lexical {
        reading: LexicalReading,
    },
    Dependency(Dependency),
    /// The ids of the head line's frame Productions, from the leaf up.
    Frame(Vec<Ident>),
}

/// What a maximal projection depends on.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dependency {
    /// It is the whole unit.
    Root,
    /// It is a child of a Constituent of another word's head line.
    Dependent {
        /// The governor's head-line word: `None` where the governor's head
        /// line ends in punctuation or in no child.
        governor: Option<ForkWord>,
        /// Its name in the governor's Production.
        function: Ident,
        /// The governor's Category.
        governor_category: Ident,
    },
}

impl Answer {
    /// Whether two answers have one name: the same content, a governor
    /// named by its surface and occurrence ([`ForkWord::names`]).
    #[must_use]
    pub fn names(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Lexical { reading: a }, Self::Lexical { reading: b }) => a == b,
            (Self::Frame(a), Self::Frame(b)) => a == b,
            (Self::Dependency(Dependency::Root), Self::Dependency(Dependency::Root)) => true,
            (
                Self::Dependency(Dependency::Dependent {
                    governor: mine,
                    function: my_function,
                    governor_category: my_category,
                }),
                Self::Dependency(Dependency::Dependent {
                    governor: theirs,
                    function: their_function,
                    governor_category: their_category,
                }),
            ) => {
                my_function == their_function
                    && my_category == their_category
                    && match (mine, theirs) {
                        (Some(mine), Some(theirs)) => mine.names(theirs),
                        (None, None) => true,
                        _ => false,
                    }
            }
            _ => false,
        }
    }
}

/// One option of a fork.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForkOption {
    pub answer: Answer,
    /// The same option's answer to each of the fork's aliases, in their
    /// order.
    pub aliases: Vec<Answer>,
    /// The Derivations that give this answer.
    pub derivations: u128,
}

/// A fork: one question and its options, in display order (by governor
/// position, then the rest of the answer; never by count).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fork {
    pub kind: ForkKind,
    pub word: ForkWord,
    /// Other questions that partition the Derivations the same way.
    pub aliases: Vec<(ForkKind, ForkWord)>,
    /// The Derivations in which the word has an answer: the sum of the
    /// options' counts.
    pub context: u128,
    /// The context is not every Derivation of the forest.
    pub conditional: bool,
    pub options: Vec<ForkOption>,
}

/// A narrowing that names no answer of the forest.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum NarrowError {
    #[error("no word `{surface}` #{occurrence}")]
    NoWord { surface: String, occurrence: usize },
    #[error(
        "no Derivation answers {answer:?} to the {kind:?} question at `{surface}` #{occurrence}"
    )]
    NoAnswer {
        kind: ForkKind,
        surface: String,
        occurrence: usize,
        answer: Answer,
    },
}

/// A unit's forest, split by head and possibly narrowed, from which forks
/// are read ([`Parse::fork_forest`]).
#[derive(Debug, Clone)]
pub struct ForkForest<'p> {
    parse: &'p Parse<'p>,
    unit: Rc<Unit>,
    graph: Graph,
}

impl<'p> Parse<'p> {
    /// The forest split by head, for reading forks and narrowing.
    ///
    /// # Errors
    /// A [`ForestCycle`] when the reachable forest has a unary cycle.
    pub fn fork_forest(&'p self) -> Result<ForkForest<'p>, ForestCycle> {
        let (unit, graph) = crate::fork_graph::split(self)?;
        Ok(ForkForest {
            parse: self,
            unit: Rc::new(unit),
            graph,
        })
    }
}

/// A question, by its word's index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Key {
    word: u32,
    kind: ForkKind,
}

/// An answer, by its interned content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Value {
    Leaf(u32),
    Dep(u32),
    Frames(u32),
}

/// A question's answers on one forest: each with its Derivations and edges.
#[derive(Debug, Default)]
struct Candidate {
    options: BTreeMap<Value, (u128, Vec<Edge>)>,
}

impl Candidate {
    fn context(&self) -> u128 {
        self.options
            .values()
            .map(|(n, _)| *n)
            .fold(0, |a, b| a.checked_add(b).expect("a count overflows u128"))
    }

    fn edges(&self, value: Value) -> HashSet<Edge> {
        self.options[&value].1.iter().copied().collect()
    }
}

/// Every question with its answers on a forest.
fn candidates(graph: &Graph) -> (u128, BTreeMap<Key, Candidate>) {
    let counts = graph.counts();
    let mut out: BTreeMap<Key, Candidate> = BTreeMap::new();
    let mut note = |key: Key, value: Value, edge: Edge, n: u128| {
        let slot = out
            .entry(key)
            .or_default()
            .options
            .entry(value)
            .or_insert((0, Vec::new()));
        slot.0 = slot.0.checked_add(n).expect("a count overflows u128");
        slot.1.push(edge);
    };
    for (edge, facts, n) in graph.edges(&counts) {
        assert!(
            n > 0,
            "the split forest keeps an edge no Derivation uses: {edge:?}"
        );
        if let Some((word, leaf)) = facts.lexical {
            note(
                Key {
                    word,
                    kind: ForkKind::Lexical,
                },
                Value::Leaf(leaf),
                edge,
                n,
            );
        }
        if let Some((word, dep, frames)) = facts.attachment {
            note(
                Key {
                    word,
                    kind: ForkKind::Dependency,
                },
                Value::Dep(dep),
                edge,
                n,
            );
            note(
                Key {
                    word,
                    kind: ForkKind::Frame,
                },
                Value::Frames(frames),
                edge,
                n,
            );
        }
    }
    (counts.total, out)
}

/// A question and the questions with its partition, each with its
/// answers' correspondence.
type Group = (Key, Vec<(Key, BTreeMap<Value, Value>)>);

/// The option counts of every question on a forest.
type Profile = BTreeMap<Key, BTreeMap<Value, u128>>;

impl ForkForest<'_> {
    /// The Derivations of the forest.
    #[must_use]
    pub fn total(&self) -> u128 {
        self.graph.counts().total
    }

    /// The unit's words, in order of position. Narrowing keeps them.
    #[must_use]
    pub fn words(&self) -> &[ForkWord] {
        &self.unit.words
    }

    /// The number of nodes of the split forest.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// The number of items of the split forest.
    #[must_use]
    pub fn item_count(&self) -> usize {
        self.graph.item_count()
    }

    /// The forest's Readings, or `None` when it has more than `limit`.
    #[must_use]
    pub fn readings(&self, limit: u128) -> Option<Vec<Reading>> {
        (self.total() <= limit).then(|| self.graph.trees(self.parse))
    }

    /// The forks of the forest, in display order (see the module doc).
    #[must_use]
    pub fn forks(&self) -> Vec<Fork> {
        let (total, candidates) = candidates(&self.graph);
        if total == 0 {
            return Vec::new();
        }
        let split = |kind: ForkKind| -> BTreeSet<Key> {
            candidates
                .iter()
                .filter(|(key, c)| key.kind == kind && c.options.len() >= 2)
                .map(|(key, _)| *key)
                .collect()
        };
        let mut questions = split(ForkKind::Lexical);
        questions.extend(split(ForkKind::Dependency));
        let frames = split(ForkKind::Frame);
        if !frames.is_empty() {
            questions.extend(self.undetermined(&self.graph, &frames));
        }
        // Group the questions that induce one partition.
        let mut joints: HashMap<(Key, Value), Profile> = HashMap::new();
        let mut groups: Vec<Group> = Vec::new();
        let signature = |key: Key| {
            let mut counts: Vec<u128> = candidates[&key].options.values().map(|o| o.0).collect();
            counts.sort_unstable();
            counts
        };
        for &key in &questions {
            let mine = signature(key);
            let joined = groups.iter_mut().find_map(|(primary, aliases)| {
                if signature(*primary) != mine {
                    return None;
                }
                let mapping = self.same_partition(&candidates, &mut joints, *primary, key)?;
                aliases.push((key, mapping));
                Some(())
            });
            if joined.is_none() {
                groups.push((key, Vec::new()));
            }
        }
        let mut forks: Vec<Fork> = groups
            .into_iter()
            .map(|(primary, aliases)| {
                let candidate = &candidates[&primary];
                let context = candidate.context();
                let mut options: Vec<ForkOption> = candidate
                    .options
                    .iter()
                    .map(|(&value, &(derivations, _))| ForkOption {
                        answer: self.answer(value),
                        aliases: aliases
                            .iter()
                            .map(|(_, mapping)| self.answer(mapping[&value]))
                            .collect(),
                        derivations,
                    })
                    .collect();
                options.sort_by(|a, b| a.answer.cmp(&b.answer));
                Fork {
                    kind: primary.kind,
                    word: self.word(primary.word).clone(),
                    aliases: aliases
                        .iter()
                        .map(|(key, _)| (key.kind, self.word(key.word).clone()))
                        .collect(),
                    context,
                    conditional: context != total,
                    options,
                }
            })
            .collect();
        forks.sort_by_key(|fork| (fork.word.start, fork.kind));
        forks
    }

    /// The forest without the Derivations that give `answer` to the `kind`
    /// question at `word`.
    ///
    /// # Errors
    /// A [`NarrowError`] when no Derivation of the forest gives that answer.
    pub fn exclude(
        &self,
        kind: ForkKind,
        word: &ForkWord,
        answer: &Answer,
    ) -> Result<Self, NarrowError> {
        let (value, candidate) = self.find(kind, word, answer)?;
        Ok(self.narrowed(self.graph.exclude(&candidate.edges(value))))
    }

    /// The forest's Derivations that give `answer` to the `kind` question at
    /// `word`. For a question every Derivation answers, this excludes every
    /// other answer; for a conditional one, it keeps the Derivations that use
    /// one of the answer's edges.
    ///
    /// # Errors
    /// A [`NarrowError`] when no Derivation of the forest gives that answer.
    pub fn pick(
        &self,
        kind: ForkKind,
        word: &ForkWord,
        answer: &Answer,
    ) -> Result<Self, NarrowError> {
        let (value, candidate) = self.find(kind, word, answer)?;
        let graph = if candidate.context() == self.total() {
            let others: HashSet<Edge> = candidate
                .options
                .iter()
                .filter(|(v, _)| **v != value)
                .flat_map(|(_, (_, edges))| edges.iter().copied())
                .collect();
            self.graph.exclude(&others)
        } else {
            self.graph.restrict(&candidate.edges(value))
        };
        Ok(self.narrowed(graph))
    }

    fn narrowed(&self, graph: Graph) -> Self {
        Self {
            parse: self.parse,
            unit: Rc::clone(&self.unit),
            graph,
        }
    }

    /// The question and answer a public name refers to, with the question's
    /// answers on this forest.
    fn find(
        &self,
        kind: ForkKind,
        word: &ForkWord,
        answer: &Answer,
    ) -> Result<(Value, Candidate), NarrowError> {
        let index = self
            .unit
            .words
            .iter()
            .position(|w| w.names(word))
            .ok_or_else(|| NarrowError::NoWord {
                surface: word.surface.clone(),
                occurrence: word.occurrence,
            })?;
        let key = Key {
            word: u32::try_from(index).expect("words fit u32"),
            kind,
        };
        let no_answer = || NarrowError::NoAnswer {
            kind,
            surface: word.surface.clone(),
            occurrence: word.occurrence,
            answer: answer.clone(),
        };
        let (_, mut candidates) = candidates(&self.graph);
        let candidate = candidates.remove(&key).ok_or_else(no_answer)?;
        let value = *candidate
            .options
            .keys()
            .find(|&&v| self.answer(v).names(answer))
            .ok_or_else(no_answer)?;
        Ok((value, candidate))
    }

    fn word(&self, word: u32) -> &ForkWord {
        &self.unit.words[word as usize]
    }

    /// An interned answer by its content.
    fn answer(&self, value: Value) -> Answer {
        let grammar = self.parse.grammar;
        match value {
            Value::Leaf(leaf) => {
                let leaf = &self.parse.forest.leaves[leaf as usize];
                Answer::Lexical {
                    reading: leaf.reading.clone(),
                }
            }
            Value::Dep(dep) => Answer::Dependency(match self.unit.dependencies[dep as usize] {
                Dep::Root => Dependency::Root,
                Dep::Dependent {
                    function,
                    category,
                    governor,
                } => Dependency::Dependent {
                    governor: governor.map(|w| self.word(w).clone()),
                    function,
                    governor_category: category,
                },
            }),
            Value::Frames(frames) => Answer::Frame(
                self.unit.frames[frames as usize]
                    .iter()
                    .map(|&p| grammar.production_id(p))
                    .collect(),
            ),
        }
    }

    /// Whether question `b` partitions the forest as `a` does: each option of
    /// `a` is, as a set of Derivations, one option of `b`. Returns that
    /// correspondence.
    fn same_partition(
        &self,
        candidates: &BTreeMap<Key, Candidate>,
        joints: &mut HashMap<(Key, Value), Profile>,
        a: Key,
        b: Key,
    ) -> Option<BTreeMap<Value, Value>> {
        let mut mapping = BTreeMap::new();
        for (&value, (n, _)) in &candidates[&a].options {
            let joint = joints.entry((a, value)).or_insert_with(|| {
                let narrowed = self.graph.restrict(&candidates[&a].edges(value));
                candidates_profile(&narrowed)
            });
            let options = joint.get(&b)?;
            let [(&other, &m)] = options.iter().collect::<Vec<_>>()[..] else {
                return None;
            };
            if m != *n || candidates[&b].options[&other].0 != *n {
                return None;
            }
            mapping.insert(value, other);
        }
        Some(mapping)
    }

    /// The frame questions of `wanted` that the lexical and dependency
    /// questions do not determine on `graph`: those with two answers among
    /// Derivations that answer every lexical and dependency question alike.
    /// Splits the forest by lexical and dependency answers until each wanted
    /// question has one answer on each part, or a part's Derivations agree on
    /// every lexical and dependency question.
    fn undetermined(&self, graph: &Graph, wanted: &BTreeSet<Key>) -> BTreeSet<Key> {
        let (total, candidates) = candidates(graph);
        let varying: BTreeSet<Key> = wanted
            .iter()
            .filter(|key| candidates.get(key).is_some_and(|c| c.options.len() >= 2))
            .copied()
            .collect();
        if varying.is_empty() {
            return varying;
        }
        let governed_by = |candidate: &Candidate, word: u32| {
            candidate.options.keys().any(|v| match v {
                Value::Dep(d) => matches!(
                    self.unit.dependencies[*d as usize],
                    Dep::Dependent { governor: Some(g), .. } if g == word
                ),
                Value::Leaf(_) | Value::Frames(_) => false,
            })
        };
        let chosen = candidates
            .iter()
            .filter(|(key, c)| {
                key.kind != ForkKind::Frame && (c.options.len() >= 2 || c.context() != total)
            })
            .min_by_key(|(key, c)| {
                let nearness = varying
                    .iter()
                    .map(|v| {
                        if key.word == v.word {
                            0
                        } else if governed_by(c, v.word) {
                            1
                        } else {
                            2
                        }
                    })
                    .min()
                    .expect("some frame question varies");
                (nearness, c.options.len(), **key)
            });
        let Some((_, chosen)) = chosen else {
            return varying;
        };
        let mut found = BTreeSet::new();
        let mut parts: Vec<Graph> = chosen
            .options
            .keys()
            .map(|&value| graph.restrict(&chosen.edges(value)))
            .collect();
        if chosen.context() != total {
            let every: HashSet<Edge> = chosen
                .options
                .values()
                .flat_map(|(_, edges)| edges.iter().copied())
                .collect();
            parts.push(graph.exclude(&every));
        }
        for part in parts {
            let open: BTreeSet<Key> = varying.difference(&found).copied().collect();
            if open.is_empty() {
                break;
            }
            found.extend(self.undetermined(&part, &open));
        }
        found
    }
}

/// The option counts of every question on a forest.
fn candidates_profile(graph: &Graph) -> Profile {
    candidates(graph)
        .1
        .into_iter()
        .map(|(key, c)| {
            (
                key,
                c.options.into_iter().map(|(v, (n, _))| (v, n)).collect(),
            )
        })
        .collect()
}

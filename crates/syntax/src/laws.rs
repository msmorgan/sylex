//! Both round-trip laws, checked over the Packed Forest without building its
//! trees.

use sylex_lexicon::Binding;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::Lexicon;

use crate::chart::ChildRef;
use crate::chart::Parse;
use crate::reading::AdmissionError;
use crate::reading::Reading;
use crate::reading::glues_after;
use crate::reading::glues_before;
use crate::step::Opening;
use crate::walk::ForestMemo;
use crate::walk::Visit;

/// Where some Reading of a Parse would not realize as the analyzed text.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RealizationFault {
    #[error("the word at chars {start}..{end} realizes as `{realized}`, not `{source_text}`")]
    Spelling {
        start: usize,
        end: usize,
        realized: String,
        source_text: String,
    },
    #[error("the spacing at char {at} is not the spacing realization writes")]
    Spacing { at: usize },
    #[error("leading or trailing whitespace is not realized")]
    Edge,
    #[error("the word at char {at} is written in a case its position does not call for")]
    Case { at: usize },
}

/// The spacing behavior of the first and last atom of a Derivation: whether
/// the first attaches to what precedes it, and whether the last attaches to
/// what follows it.
type Edges = (bool, bool);

/// The four possible pairs of edge behaviors, without per-node allocation.
#[derive(Clone, Copy, Default)]
struct EdgeMask(u8);

impl EdgeMask {
    fn one(edges: Edges) -> Self {
        let mut mask = Self::default();
        mask.insert(edges);
        mask
    }

    fn insert(&mut self, (before, after): Edges) {
        self.0 |= 1 << (u8::from(before) * 2 + u8::from(after));
    }

    fn extend(&mut self, other: Self) {
        self.0 |= other.0;
    }

    fn iter(self) -> impl Iterator<Item = Edges> {
        (0..4).filter_map(move |bit| {
            (self.0 & (1 << bit) != 0).then_some((bit & 2 != 0, bit & 1 != 0))
        })
    }

    fn bind(self, binding: Binding) -> Self {
        let mut mask = Self::default();
        for (before, after) in self.iter() {
            mask.insert((
                before || glues_before(binding),
                after || glues_after(binding),
            ));
        }
        mask
    }
}

/// The ways a Constituent opens across its Derivations, as a mask over
/// [`Opening`].
#[derive(Clone, Copy, Default)]
struct Openings(u8);

impl Openings {
    const ALL: [Opening; 4] = [
        Opening::Empty,
        Opening::Lower,
        Opening::Capital,
        Opening::Fixed,
    ];

    fn one(opening: Opening) -> Self {
        let mut mask = Self::default();
        mask.insert(opening);
        mask
    }

    fn bit(opening: Opening) -> u8 {
        match opening {
            Opening::Empty => 1,
            Opening::Lower => 2,
            Opening::Capital => 4,
            Opening::Fixed => 8,
        }
    }

    fn insert(&mut self, opening: Opening) {
        self.0 |= Self::bit(opening);
    }

    fn extend(&mut self, other: Self) {
        self.0 |= other.0;
    }

    fn contains(self, opening: Opening) -> bool {
        self.0 & Self::bit(opening) != 0
    }

    fn iter(self) -> impl Iterator<Item = Opening> {
        Self::ALL.into_iter().filter(move |o| self.contains(*o))
    }
}

/// What law 1 keeps of a forest node or item: its edge behaviors and how it
/// opens.
#[derive(Clone, Copy, Default)]
struct Bounds {
    edges: EdgeMask,
    openings: Openings,
}

/// Memoized bounds of forest nodes and items.
type EdgeMemo = ForestMemo<Bounds, Bounds>;

impl Parse<'_> {
    /// Round-trip law 1 over the whole forest: every Reading realizes as the
    /// analyzed text. Each reachable leaf's spelling is checked once, the
    /// spacing between every pair of atoms adjacent in some Derivation once
    /// per pair of edge behaviors, and the case each child is written in
    /// once per way the children before it open their mother, against the
    /// case its position calls for, so the work is bounded by the forest's
    /// size, never by its number of trees.
    ///
    /// # Errors
    /// The first fault found.
    pub fn verify_realization(&self, lexicon: &Lexicon) -> Result<(), RealizationFault> {
        let n = self.tokens.len();
        let mut memo = EdgeMemo::default();
        // Only reachable leaves participate in the law. Cache their successful
        // checks even when a unary cycle prevents memoizing a surrounding node.
        let mut leaves = vec![None; self.forest.leaves.len()];
        for &root in &self.forest.roots {
            if self.tokens.first().is_some_and(|c| c.is_whitespace())
                || self.forest.nodes[root].end != n
            {
                return Err(RealizationFault::Edge);
            }
            // The whole stands where no capital is called for.
            if self
                .node_edges(root, lexicon, &mut memo, &mut leaves)?
                .openings
                .contains(Opening::Capital)
            {
                return Err(RealizationFault::Case {
                    at: self.forest.nodes[root].start,
                });
            }
        }
        Ok(())
    }

    fn node_edges(
        &self,
        node: usize,
        lexicon: &Lexicon,
        memo: &mut EdgeMemo,
        leaves: &mut [Option<Binding>],
    ) -> Result<Bounds, RealizationFault> {
        let frame = match memo.nodes.enter(node) {
            Visit::Known(bounds) => return Ok(bounds),
            Visit::Cycle => return Ok(Bounds::default()),
            Visit::Open(frame) => frame,
        };
        let mut bounds = Bounds::default();
        for &(_, item) in &self.forest.nodes[node].families {
            let item = self.item_edges(item, lexicon, memo, leaves)?;
            bounds.edges.extend(item.edges);
            bounds.openings.extend(item.openings);
        }
        memo.nodes.leave(frame, &bounds);
        Ok(bounds)
    }

    fn item_edges(
        &self,
        item: usize,
        lexicon: &Lexicon,
        memo: &mut EdgeMemo,
        leaves: &mut [Option<Binding>],
    ) -> Result<Bounds, RealizationFault> {
        if let Some(bounds) = memo.items.get(&item) {
            return Ok(*bounds);
        }
        let mark = memo.nodes.mark();
        let mut bounds = Bounds::default();
        let state = &self.forest.items[item];
        let production = &self.grammar.productions[state.production];
        let capital = production.slots[state.dot - 1].capital;
        for &(previous, child) in &self.forest.items[item].families {
            let (last, opening, at): (EdgeMask, Openings, usize) = match child {
                ChildRef::Node(n) => {
                    let child = self.node_edges(n, lexicon, memo, leaves)?;
                    let edges = if let crate::step::Symbol::Cat {
                        binding: Some(binding),
                        ..
                    } = &production.rhs[state.dot - 1]
                    {
                        child.edges.bind(*binding)
                    } else {
                        child.edges
                    };
                    (edges, child.openings, self.forest.nodes[n].start)
                }
                ChildRef::Leaf(l) => {
                    let binding = if let Some(binding) = leaves[l] {
                        binding
                    } else {
                        let binding = self.leaf_binding(l, lexicon)?;
                        leaves[l] = Some(binding);
                        binding
                    };
                    let leaf = &self.forest.leaves[l];
                    (
                        EdgeMask::one((glues_before(binding), glues_after(binding))),
                        Openings::one(Opening::of(leaf.case)),
                        leaf.start,
                    )
                }
                ChildRef::Punct => {
                    let crate::step::Symbol::Punct(_, binding, _) = &production.rhs[state.dot - 1]
                    else {
                        unreachable!()
                    };
                    (
                        EdgeMask::one((glues_before(*binding), glues_after(*binding))),
                        Openings::one(Opening::Fixed),
                        state.end,
                    )
                }
            };
            let before = match previous {
                Some(previous) => self.item_edges(previous, lexicon, memo, leaves)?,
                None => Bounds {
                    edges: EdgeMask::default(),
                    openings: Openings::one(Opening::Empty),
                },
            };
            for so_far in before.openings.iter() {
                for child in opening.iter() {
                    let Some(opened) = so_far.then(child, capital) else {
                        return Err(RealizationFault::Case { at });
                    };
                    bounds.openings.insert(opened);
                }
            }
            let Some(previous) = previous else {
                bounds.edges.extend(last);
                continue;
            };
            let at = self.forest.items[previous].end;
            let symbol = &self.grammar.productions[self.forest.items[item].production].rhs
                [self.forest.items[item].dot - 1];
            let space = if matches!(symbol, crate::step::Symbol::Punct(c, ..) if c.is_whitespace())
            {
                Some(false)
            } else {
                self.space_after(at)
            };
            for (first, glued) in before.edges.iter() {
                for (attaches, after) in last.iter() {
                    let glue = glued || attaches;
                    if space != Some(!glue) {
                        return Err(RealizationFault::Spacing { at });
                    }
                    bounds.edges.insert((first, after));
                }
            }
        }
        if memo.nodes.settle(mark) {
            memo.items.insert(item, bounds);
        }
        Ok(bounds)
    }

    /// Whether one space (`Some(true)`) or nothing (`Some(false)`) follows
    /// `at` before the next atom; `None` for any other whitespace.
    fn space_after(&self, at: usize) -> Option<bool> {
        let run = self.tokens[at..]
            .iter()
            .take_while(|c| c.is_whitespace())
            .count();
        match run {
            0 => Some(false),
            1 if self.tokens[at] == ' ' => Some(true),
            _ => None,
        }
    }

    /// Checks a leaf's spelling against the text and returns its binding.
    fn leaf_binding(&self, leaf: usize, lexicon: &Lexicon) -> Result<Binding, RealizationFault> {
        let leaf = &self.forest.leaves[leaf];
        let source = &self.tokens[leaf.start..leaf.end];
        // Its case is the one its position calls for, which the item that
        // takes it checks (`Opening::then`).
        let realized = lexicon
            .realize_written(&leaf.reading, leaf.case)
            .expect("a forest leaf is a lexical match, which its lexicon licenses");
        if !realized.chars().eq(source.iter().copied()) {
            return Err(RealizationFault::Spelling {
                start: leaf.start,
                end: leaf.end,
                realized,
                source_text: source.iter().collect(),
            });
        }
        Ok(match &leaf.reading {
            LexicalReading::Word(v) => {
                let lexeme = self
                    .grammar
                    .lexeme(&v.lexeme)
                    .expect("a forest leaf's Lexeme is declared by the grammar's lexicon");
                self.grammar.facts(lexeme).binding
            }
            LexicalReading::Numeral { .. } | LexicalReading::FlavorWord { .. } => Binding::Free,
        })
    }

    /// Round-trip law 2's membership test: whether `reading` is one of this
    /// forest's trees, found by walking the forest along the Reading rather
    /// than enumerating trees.
    #[must_use]
    pub fn contains(&self, reading: &Reading) -> bool {
        self.forest
            .roots
            .iter()
            .any(|&root| self.matches_node(root, reading))
    }

    fn matches_node(&self, node: usize, reading: &Reading) -> bool {
        let Reading::Node(wanted) = reading else {
            return false;
        };
        let grammar = self.grammar;
        let packed = &self.forest.nodes[node];
        grammar.categories[packed.category] == wanted.category
            && packed.summary.gap == wanted.gap
            && grammar.feature_map(packed.category, &packed.summary) == wanted.features
            && packed.families.iter().any(|&(production, item)| {
                grammar.productions[production].id == wanted.production
                    && self.matches_item(item, &wanted.children)
            })
    }

    /// Whether some family chain ending at `item` spells exactly `children`.
    fn matches_item(&self, item: usize, children: &[Reading]) -> bool {
        self.forest.items[item]
            .families
            .iter()
            .any(|&(previous, child)| {
                let rest = match child {
                    ChildRef::Punct => children,
                    ChildRef::Leaf(l) => match children.split_last() {
                        Some((Reading::Leaf(value), rest))
                            if *value == self.forest.leaves[l].reading =>
                        {
                            rest
                        }
                        _ => return false,
                    },
                    ChildRef::Node(n) => match children.split_last() {
                        Some((last, rest)) if self.matches_node(n, last) => rest,
                        _ => return false,
                    },
                };
                match previous {
                    Some(p) => self.matches_item(p, rest),
                    None => rest.is_empty(),
                }
            })
    }

    /// Admission of `reading` as the Category this Parse was made for, in the
    /// card-name scope its text was analyzed in.
    ///
    /// # Errors
    /// As [`crate::CompiledGrammar::admit`].
    pub fn admit(&self, reading: &Reading, lexicon: &Lexicon) -> Result<(), AdmissionError> {
        self.grammar
            .admit_as(reading, lexicon, self.root, &self.scope)
    }

    /// The Category this Parse was made for.
    #[must_use]
    pub fn category(&self) -> &str {
        &self.grammar.categories[self.root]
    }

    /// The analyzed text this Parse was made from, as written.
    #[must_use]
    pub fn text(&self) -> String {
        self.source.clone()
    }
}

//! Both round-trip laws, checked over the Packed Forest without building its
//! trees.

use std::ops::Range;

use sylex_lexicon::Binding;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::Lexicon;

use crate::chart::ChildRef;
use crate::chart::Parse;
use crate::chart::QUOTATION_MARK;
use crate::chart::is_quotation_mark;
use crate::chart::skip;
use crate::reading::AdmissionError;
use crate::reading::Reading;
use crate::reading::glues_after;
use crate::reading::glues_before;
use crate::step::Opening;
use crate::step::Production;
use crate::step::Symbol;
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

/// One atom of a chain, or a child Constituent's atoms: its edge behaviors,
/// how it opens and its extent in chars.
struct Atom {
    edges: EdgeMask,
    openings: Openings,
    start: usize,
    end: usize,
}

/// The atoms of a partial Production so far: their bounds, `None` before
/// the first, and where they end.
struct Chain {
    bounds: Option<Bounds>,
    end: usize,
}

impl Chain {
    /// The chain with `atom` after it, at a position that opens with a
    /// capital or not, `space` being the spacing the text has between them
    /// ([`Parse::space_after`]).
    fn then(
        self,
        atom: &Atom,
        capital: bool,
        space: Option<bool>,
    ) -> Result<Self, RealizationFault> {
        let mut bounds = Bounds::default();
        let before = self
            .bounds
            .map_or(Openings::one(Opening::Empty), |bounds| bounds.openings);
        for so_far in before.iter() {
            for child in atom.openings.iter() {
                let Some(opened) = so_far.then(child, capital) else {
                    return Err(RealizationFault::Case { at: atom.start });
                };
                bounds.openings.insert(opened);
            }
        }
        match self.bounds {
            None => bounds.edges = atom.edges,
            Some(before) => {
                for (first, glued) in before.edges.iter() {
                    for (attaches, after) in atom.edges.iter() {
                        if space != Some(!(glued || attaches)) {
                            return Err(RealizationFault::Spacing { at: self.end });
                        }
                        bounds.edges.insert((first, after));
                    }
                }
            }
        }
        Ok(Self {
            bounds: Some(bounds),
            end: atom.end,
        })
    }
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
        let at = self.item_child(item);
        for &(previous, child) in &state.families {
            // The atoms before the child: the previous item's, or the marks
            // the Production opens with.
            let before = match previous {
                Some(previous) => Chain {
                    bounds: Some(self.item_edges(previous, lexicon, memo, leaves)?),
                    end: self.forest.items[previous].end,
                },
                None => self.past_marks(
                    production,
                    0..at,
                    Chain {
                        bounds: None,
                        end: state.start,
                    },
                )?,
            };
            let atom = match child {
                ChildRef::Node(n) => {
                    let child = self.node_edges(n, lexicon, memo, leaves)?;
                    let edges = if let Symbol::Cat {
                        binding: Some(binding),
                        ..
                    } = &production.rhs[at]
                    {
                        child.edges.bind(*binding)
                    } else {
                        child.edges
                    };
                    let node = &self.forest.nodes[n];
                    Atom {
                        edges,
                        openings: child.openings,
                        start: node.start,
                        end: node.end,
                    }
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
                    Atom {
                        edges: EdgeMask::one((glues_before(binding), glues_after(binding))),
                        openings: Openings::one(Opening::of(leaf.case)),
                        start: leaf.start,
                        end: leaf.end,
                    }
                }
            };
            let space = self.space_after(before.end);
            let with_child = before.then(&atom, production.slots[at].capital, space)?;
            let after = self.past_marks(production, at + 1..state.dot, with_child)?;
            assert_eq!(
                after.end, state.end,
                "the marks of `{}` end where its item does",
                production.id
            );
            let found = after.bounds.expect("a chain with a child has bounds");
            bounds.edges.extend(found.edges);
            bounds.openings.extend(found.openings);
        }
        if memo.nodes.settle(mark) {
            memo.items.insert(item, bounds);
        }
        Ok(bounds)
    }

    /// A chain of atoms once it has the marks its Production has at
    /// `symbols`: a mark is no child of the forest, so it is read from the
    /// Production, and found in the text where the chain ends.
    fn past_marks(
        &self,
        production: &Production,
        symbols: Range<usize>,
        mut chain: Chain,
    ) -> Result<Chain, RealizationFault> {
        for symbol in symbols {
            let Symbol::Punct(c, binding, _) = production.rhs[symbol] else {
                unreachable!("only marks stand between the children of a Production")
            };
            let space = if c.is_whitespace() { Some(false) } else { self.space_after(chain.end) };
            let mark = Atom {
                edges: EdgeMask::one((glues_before(binding), glues_after(binding))),
                openings: Openings::one(Opening::Fixed),
                start: chain.end,
                end: self.mark_end(c, chain.end),
            };
            chain = chain.then(&mark, production.slots[symbol].capital, space)?;
        }
        Ok(chain)
    }

    /// Where the mark `c` that a Production has after an atom ending at
    /// `from` ends. A written mark is one char, after the space before it
    /// unless it is whitespace itself; a period the text does not write
    /// there (a quotation's own final period, or one after a word ending in
    /// terminal punctuation) ends where it starts.
    fn mark_end(&self, c: char, from: usize) -> usize {
        if c.is_whitespace() {
            return from + 1;
        }
        let at = skip(&self.tokens, from);
        let written = self.tokens.get(at).is_some_and(|&token| {
            if c == QUOTATION_MARK { is_quotation_mark(token) } else { token == c }
        });
        if written {
            at + 1
        } else {
            assert_eq!(c, '.', "only a period may be unwritten");
            from
        }
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

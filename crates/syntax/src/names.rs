//! Introduced Names in a parse: which names a Constituent introduces, which
//! it names again, and what a name still needs around it.
//!
//! No inventory lists an Introduced Name (`docs/design/lexicon.md` §6):
//! lexical analysis offers the title run at each capitalized word, and the
//! grammar admits it where a name is introduced (`SymbolDecl::IntroducedName`)
//! or, in a Reading that introduces it, named again
//! (`SymbolDecl::RepeatedName`). Whether a Reading that names a name again
//! also introduces it is no local fact, so each Constituent's summary records
//! both sets and the root compares them.

use std::collections::BTreeMap;

use sylex_lexicon::AnalyzedText;
use sylex_lexicon::LexicalReading;

use crate::Reading;
use crate::compile::CompiledGrammar;
use crate::step::ChildRole;
use crate::step::ChildView;
use crate::step::Production;
use crate::step::Symbol;
use crate::step::Test;

/// What the leaf of an Introduced Name carries beside its lexical facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NameLeaf {
    /// The leaf among its text's lexical alternatives.
    pub leaf: u32,
    /// Its name's bit ([`NameBits`]); 0 for a name its text writes once,
    /// which no [`Test::RepeatedName`] admits.
    pub bit: u32,
}

/// The names of one text that a summary tracks, each with a bit: a title run
/// written at more than one place, which may be introduced at one and named
/// again at another.
#[derive(Debug, Default)]
pub(crate) struct NameBits {
    bits: BTreeMap<String, u32>,
}

impl NameBits {
    /// A bit for each of `names`, or `None` for more than 32 of them.
    pub(crate) fn tracking<'a>(names: impl IntoIterator<Item = &'a str>) -> Option<Self> {
        let names: std::collections::BTreeSet<&str> = names.into_iter().collect();
        (names.len() <= 32).then(|| Self {
            bits: names
                .into_iter()
                .enumerate()
                .map(|(index, name)| (name.to_owned(), 1 << index))
                .collect(),
        })
    }

    /// The tracked names of an analyzed text: each title run written at more
    /// than one place.
    ///
    /// # Panics
    /// On more than 32 tracked names in one text.
    pub(crate) fn of(analyzed: &AnalyzedText) -> Self {
        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        for found in &analyzed.matches {
            match &found.reading {
                LexicalReading::IntroducedName { name } => *counts.entry(name).or_default() += 1,
                LexicalReading::Word(_)
                | LexicalReading::OwnName { .. }
                | LexicalReading::FlavorWord { .. }
                | LexicalReading::SetName { .. }
                | LexicalReading::Numeral { .. } => {}
            }
        }
        let tracked: Vec<&str> = counts
            .into_iter()
            .filter(|&(_, count)| count > 1)
            .map(|(name, _)| name)
            .collect();
        Self::tracking(tracked.iter().copied())
            .unwrap_or_else(|| panic!("a text repeats more than 32 title runs: {tracked:?}"))
    }

    /// A name's bit; 0 for a name that is not tracked.
    pub(crate) fn bit(&self, name: &str) -> u32 {
        self.bits.get(name).copied().unwrap_or(0)
    }
}

/// A phrase that stands only in a Complement of a phrase headed by a word a
/// Production names (`ProductionDecl::complement_of`,
/// `ProductionDecl::within`), or never as one
/// (`ProductionDecl::not_complement_of`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) struct Selection {
    /// The head, as an index into [`CompiledGrammar::name_heads`].
    pub head: u8,
    /// It is that Complement itself, through Productions of one child, not
    /// a phrase inside it.
    pub direct: bool,
    /// It is never that Complement: the selection ends, met, at the first
    /// Production of more than one child that does not refuse it.
    pub refused: bool,
}

/// What a Production says about Introduced Names (`ProductionDecl`).
#[derive(Debug, Clone, Default)]
pub(crate) struct NameRules {
    /// `ProductionDecl::undescribed`.
    pub undescribed: bool,
    /// `ProductionDecl::describes`, as a child position.
    pub describes: Option<usize>,
    /// `ProductionDecl::headed_by`: child positions, each with its head as
    /// an index into [`CompiledGrammar::name_heads`].
    pub headed_by: Vec<(usize, u8)>,
    /// `ProductionDecl::complement_of`, `ProductionDecl::within` or
    /// `ProductionDecl::not_complement_of`.
    pub selection: Option<Selection>,
}

/// The Introduced Names of a Constituent, or of a partial Production's
/// children so far. A text that writes no title run twice and holds no such
/// Production leaves every field empty.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) struct Names {
    /// The tracked names it introduces, as a mask ([`NameBits`]).
    pub introduced: u32,
    /// The tracked names it names again.
    pub repeated: u32,
    /// The `undescribed` name it ends in, through its last child: its leaf
    /// and bit. Only the description of a Production that `describes` it
    /// follows it.
    pub undescribed: Option<(u32, u32)>,
    /// The selection a phrase in it still needs.
    pub selection: Option<Selection>,
    /// The words of [`CompiledGrammar::name_heads`] its lexical head is, as
    /// a mask.
    pub headed_by: u16,
    /// The words of [`CompiledGrammar::name_heads`] the Production's own
    /// head is, as a mask. A Constituent's summary holds none.
    selector: u16,
}

impl Names {
    /// The names after child `position` of `production`, or `None` where
    /// the child does not fit them.
    pub(crate) fn accept(
        self,
        grammar: &CompiledGrammar,
        production: &Production,
        position: usize,
        child: &ChildView<'_>,
    ) -> Option<Self> {
        let rules = &production.name_rules;
        let mut next = self;
        let described = position
            .checked_sub(1)
            .is_some_and(|before| rules.describes == Some(before));
        match (self.undescribed, described) {
            (Some((_, bit)), true) => {
                next.introduced |= bit;
                next.undescribed = None;
            }
            (None, false) => {}
            // Only its description follows an undescribed name, and a
            // description describes one.
            (Some(_), false) | (None, true) => return None,
        }
        match child {
            ChildView::Punct => {}
            ChildView::Leaf(leaf, ..) => {
                if let Some(name) = leaf.introduced_name {
                    match &production.rhs[position] {
                        Symbol::Leaf(Test::RepeatedName) => next.repeated |= name.bit,
                        Symbol::Leaf(Test::IntroducedName) if rules.undescribed => {
                            next.undescribed = Some((name.leaf, name.bit));
                        }
                        Symbol::Leaf(Test::IntroducedName) => next.introduced |= name.bit,
                        symbol => {
                            unreachable!("{symbol:?} admitted the leaf of an Introduced Name")
                        }
                    }
                }
                if production.head == Some(position) || production.lexical_head == Some(position) {
                    let heads = grammar
                        .name_heads
                        .iter()
                        .enumerate()
                        .filter(|(_, test)| grammar.leaf_matches(test, leaf))
                        .fold(0, |mask, (index, _)| mask | 1 << index);
                    if production.head == Some(position) {
                        next.selector = heads;
                    }
                    if production.lexical_head == Some(position) {
                        next.headed_by = heads;
                    }
                }
            }
            ChildView::Node(summary) => {
                let names = summary.names;
                next.introduced |= names.introduced;
                next.repeated |= names.repeated;
                next.undescribed = names.undescribed;
                if production.lexical_head == Some(position) {
                    next.headed_by = names.headed_by;
                }
                if rules
                    .headed_by
                    .iter()
                    .any(|&(at, head)| at == position && names.headed_by & 1 << head == 0)
                {
                    return None;
                }
                if let Some(selection) = names.selection {
                    let complement = matches!(
                        production.roles[position],
                        ChildRole::Object
                            | ChildRole::Complement
                            | ChildRole::PredicativeComplement
                    );
                    let selected = complement && self.selector & 1 << selection.head != 0;
                    let alone = production.rhs.len() == 1;
                    match (selection.refused, selected) {
                        // The head selects it.
                        (false, true) => {}
                        (true, true) => return None,
                        (true, false) if !alone => {}
                        (false, false) if selection.direct && !alone => return None,
                        (_, false) => next.selection = merge(next.selection, Some(selection))?,
                    }
                }
            }
        }
        Some(next)
    }

    /// The names of the Constituent `production` builds from these
    /// children, or `None` where its own selection is not theirs.
    pub(crate) fn finish(self, production: &Production) -> Option<Self> {
        Some(Self {
            selection: merge(self.selection, production.name_rules.selection)?,
            selector: 0,
            ..self
        })
    }

    /// Whether a root may have these names: nothing in it is left
    /// undescribed or unselected, and it introduces each name it names
    /// again.
    pub(crate) fn complete(self) -> bool {
        self.undescribed.is_none()
            && self.selection.is_none_or(|selection| selection.refused)
            && self.repeated & !self.introduced == 0
    }
}

/// Two selections as one, or `None` where they differ.
#[expect(clippy::option_option, reason = "the outer level is the mismatch")]
fn merge(a: Option<Selection>, b: Option<Selection>) -> Option<Option<Selection>> {
    match (a, b) {
        (Some(a), Some(b)) if a != b => None,
        (a, b) => Some(a.or(b)),
    }
}

/// The name of each Introduced Name leaf of a Reading, in order.
pub(crate) fn introduced_names<'r>(reading: &'r Reading, names: &mut Vec<&'r str>) {
    match reading {
        Reading::Leaf(LexicalReading::IntroducedName { name }) => names.push(name),
        Reading::Leaf(
            LexicalReading::Word(_)
            | LexicalReading::OwnName { .. }
            | LexicalReading::Numeral { .. }
            | LexicalReading::FlavorWord { .. }
            | LexicalReading::SetName { .. },
        ) => {}
        Reading::Node(node) => {
            for child in &node.children {
                introduced_names(child, names);
            }
        }
    }
}

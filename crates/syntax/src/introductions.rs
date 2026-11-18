//! The words a text introduces, in a parse: which of them a Constituent
//! introduces, which it uses, which choices it offers, and what a name still
//! needs around it.
//!
//! No inventory lists an Introduced Name, an Anchor Word or a Vote Option
//! (`docs/design/lexicon.md` §2, §6): lexical analysis offers each wherever
//! its shape is written, and the grammar admits it where the text
//! introduces it (`SymbolDecl::IntroducedName`, or any leaf of a Production
//! that `offers`) or uses it (`SymbolDecl::RepeatedName`, or its leaf in any
//! other Production). A use needs an introduction: by a line before the
//! text, which its Card Scope records, or in the same Reading, before the
//! use or after it. That is no local fact, so each Constituent's summary
//! records both sets and the root compares them.
//!
//! The words a Production `offers` are a choice the text gives (*choose
//! Khans or Dragons*; *votes for sickness or psychosis*), and the choice is
//! one only where the face goes on to use a word of it: in the same
//! Reading, or on a line after the text, which the Card Scope records too.
//! A summary records the choices it offers, and a root that leaves one
//! undischarged is no Reading. The lines after a text are read after it, so
//! a parse keeps the roots that only they can discharge
//! ([`crate::Parse::discharge`]).

use std::collections::BTreeMap;

use sylex_lexicon::AnalyzedText;
use sylex_lexicon::CardScope;
use sylex_lexicon::LexicalReading;

use crate::Reading;
use crate::compile::CompiledGrammar;
use crate::step::ChildRole;
use crate::step::ChildView;
use crate::step::Production;
use crate::step::Symbol;
use crate::step::Test;

/// What the leaf of an Introduced Name, an Anchor Word or a Vote Option
/// carries beside its lexical facts.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct IntroducedLeaf {
    /// The leaf among its text's lexical alternatives.
    pub leaf: u32,
    /// Its word's bit ([`WordBits`]); 0 for a word its text writes once,
    /// which the text cannot both introduce and use.
    pub bit: u64,
    /// A line before the text introduces the word (`CardScope::introduces`).
    pub held: bool,
}

/// Whether `reading` is of a kind a text introduces: an Introduced Name, an
/// Anchor Word or a Vote Option.
pub(crate) fn introducible(reading: &LexicalReading) -> bool {
    match reading {
        LexicalReading::IntroducedName { .. }
        | LexicalReading::AnchorWord { .. }
        | LexicalReading::VoteOption { .. } => true,
        LexicalReading::Word(_)
        | LexicalReading::OwnName { .. }
        | LexicalReading::FlavorWord { .. }
        | LexicalReading::SetName { .. }
        | LexicalReading::KindName { .. }
        | LexicalReading::Numeral { .. } => false,
    }
}

/// The words of one text that a summary tracks, each with a bit: a word of
/// a kind a text introduces, written at more than one place, which may be
/// introduced at one and used at another.
#[derive(Debug, Default)]
pub(crate) struct WordBits {
    bits: BTreeMap<LexicalReading, u64>,
}

impl WordBits {
    /// A bit for each of `words`, or `None` for more than 64 of them.
    pub(crate) fn tracking<'a>(
        words: impl IntoIterator<Item = &'a LexicalReading>,
    ) -> Option<Self> {
        let words: std::collections::BTreeSet<&LexicalReading> = words.into_iter().collect();
        (words.len() <= 64).then(|| Self {
            bits: words
                .into_iter()
                .enumerate()
                .map(|(index, word)| (word.clone(), 1 << index))
                .collect(),
        })
    }

    /// The tracked words of an analyzed text: each written at more than one
    /// place.
    ///
    /// # Panics
    /// On more than 64 tracked words in one text.
    pub(crate) fn of(analyzed: &AnalyzedText) -> Self {
        let mut counts: BTreeMap<&LexicalReading, usize> = BTreeMap::new();
        for found in analyzed.matches.iter().chain(&analyzed.open_words) {
            if introducible(&found.reading) {
                *counts.entry(&found.reading).or_default() += 1;
            }
        }
        let tracked: Vec<&LexicalReading> = counts
            .into_iter()
            .filter(|&(_, count)| count > 1)
            .map(|(word, _)| word)
            .collect();
        Self::tracking(tracked.iter().copied()).unwrap_or_else(|| {
            panic!(
                "a text writes more than 64 words twice: {}",
                analyzed.text()
            )
        })
    }

    /// A word's bit; 0 for a word that is not tracked.
    pub(crate) fn bit(&self, word: &LexicalReading) -> u64 {
        self.bits.get(word).copied().unwrap_or(0)
    }

    /// The tracked words a line before the text introduces, as a mask.
    pub(crate) fn held(&self, scope: &CardScope) -> u64 {
        self.bits
            .iter()
            .filter(|(word, _)| scope.introduces(word))
            .fold(0, |mask, (_, bit)| mask | bit)
    }
}

/// The choices the Readings of one text offer, each the words of one
/// Constituent of a Production that `offers`, in the order found: a summary
/// holds a choice as its bit.
#[derive(Debug, Default)]
pub(crate) struct Offers {
    offers: Vec<Vec<LexicalReading>>,
}

impl Offers {
    /// The bit of the choice between `words`, or `None` for more than 32
    /// choices.
    pub(crate) fn bit(&mut self, mut words: Vec<LexicalReading>) -> Option<u32> {
        words.sort();
        words.dedup();
        let index = self
            .offers
            .iter()
            .position(|offer| *offer == words)
            .unwrap_or_else(|| {
                self.offers.push(words);
                self.offers.len() - 1
            });
        (index < 32).then(|| 1 << index)
    }

    /// Whether each choice of `introduced` is discharged: the Reading uses
    /// a word of it, or a line after the text does
    /// (`CardScope::discharges`).
    pub(crate) fn discharged(
        &self,
        introduced: Introductions,
        bits: &WordBits,
        scope: &CardScope,
    ) -> bool {
        self.offers
            .iter()
            .enumerate()
            .filter(|(index, _)| introduced.offers & 1 << index != 0)
            .all(|(_, offer)| {
                offer
                    .iter()
                    .any(|word| scope.discharges(word) || bits.bit(word) & introduced.used != 0)
            })
    }
}

/// What a Production does with the leaf of an introducible word at a
/// child position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Role {
    Introduces,
    /// It offers a name that only a description after it introduces
    /// (`ProductionDecl::undescribed`).
    Undescribed,
    Uses,
}

impl Role {
    pub(crate) fn of(production: &Production, position: usize) -> Self {
        let rules = &production.name_rules;
        match &production.rhs[position] {
            Symbol::Leaf(Test::RepeatedName) => Self::Uses,
            Symbol::Leaf(Test::IntroducedName) if rules.undescribed => Self::Undescribed,
            Symbol::Leaf(Test::IntroducedName) => Self::Introduces,
            Symbol::Leaf(_) if rules.offers => Self::Introduces,
            Symbol::Leaf(_) => Self::Uses,
            symbol @ (Symbol::Cat { .. } | Symbol::Punct(..)) => {
                unreachable!("{symbol:?} admitted a leaf")
            }
        }
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

/// What a Production says about the words a text introduces
/// (`ProductionDecl`).
#[derive(Debug, Clone, Default)]
pub(crate) struct NameRules {
    /// `ProductionDecl::offers`.
    pub offers: bool,
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

/// The introduced words of a Constituent, or of a partial Production's
/// children so far. A text that writes no such word twice and holds no
/// Production that says anything about them leaves every field empty.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) struct Introductions {
    /// The tracked words it introduces, as a mask ([`WordBits`]).
    pub introduced: u64,
    /// The tracked words it uses.
    pub used: u64,
    /// The choices it offers, as a mask ([`Offers`]).
    pub offers: u32,
    /// The `undescribed` name it ends in, through its last child: its leaf
    /// and bit. Only the description of a Production that `describes` it
    /// follows it.
    pub undescribed: Option<(u32, u64)>,
    /// The selection a phrase in it still needs.
    pub selection: Option<Selection>,
    /// The words of [`CompiledGrammar::name_heads`] its lexical head is, as
    /// a mask.
    pub headed_by: u16,
    /// The words of [`CompiledGrammar::name_heads`] the Production's own
    /// head is, as a mask. A Constituent's summary holds none.
    selector: u16,
}

impl Introductions {
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
                if let Some(word) = leaf.introduced {
                    match Role::of(production, position) {
                        Role::Introduces => next.introduced |= word.bit,
                        Role::Undescribed => next.undescribed = Some((word.leaf, word.bit)),
                        // A word its text writes once is used only where a
                        // line before the text introduces it.
                        Role::Uses if word.bit == 0 && !word.held => return None,
                        Role::Uses => next.used |= word.bit,
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
                next.used |= names.used;
                next.offers |= names.offers;
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

    /// Whether a root may have these words: nothing in it is left
    /// undescribed or unselected, and each word it uses is introduced, by
    /// it or by a line before the text (`held`, [`WordBits::held`]).
    /// Whether its choices are discharged is [`Offers::discharged`]'s to
    /// say.
    pub(crate) fn complete(self, held: u64) -> bool {
        self.undescribed.is_none()
            && self.selection.is_none_or(|selection| selection.refused)
            && self.used & !(self.introduced | held) == 0
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

/// Each leaf of a Reading that is a word of a kind a text introduces, in
/// order.
pub(crate) fn introducible_leaves<'r>(reading: &'r Reading, words: &mut Vec<&'r LexicalReading>) {
    match reading {
        Reading::Leaf(value) => {
            if introducible(value) {
                words.push(value);
            }
        }
        Reading::Node(node) => {
            for child in &node.children {
                introducible_leaves(child, words);
            }
        }
    }
}

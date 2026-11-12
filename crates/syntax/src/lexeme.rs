//! Dense ids for the declared Lexemes of a bound lexicon.

use std::num::NonZeroU32;

use sylex_lexicon::LexemeId;

/// A declared Lexeme of the lexicon a grammar is bound to, as a dense index
/// assigned in [`LexemeId`] order, so two ids order as their `LexemeId`s do.
/// The niche keeps `Option<Lexeme>` at four bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) struct Lexeme(NonZeroU32);

impl Lexeme {
    /// Its index into the tables a [`Lexemes`] is parallel to.
    pub(crate) fn index(self) -> usize {
        usize::try_from(self.0.get() - 1).expect("a u32 index fits a usize")
    }

    fn at(index: usize) -> Self {
        let id = u32::try_from(index + 1).expect("fewer than u32::MAX lexemes");
        Self(NonZeroU32::new(id).expect("one past an index is nonzero"))
    }
}

/// The declared [`LexemeId`]s of a lexicon, each interned as its dense
/// [`Lexeme`].
#[derive(Debug)]
pub(crate) struct Lexemes {
    /// Strictly ascending, so a `Lexeme` is its position plus one.
    ids: Vec<LexemeId>,
}

impl Lexemes {
    /// Interns `ids`, which must be strictly ascending (a lexicon's
    /// `BTreeMap` keys).
    pub(crate) fn new(ids: impl IntoIterator<Item = LexemeId>) -> Self {
        let ids: Vec<LexemeId> = ids.into_iter().collect();
        assert!(
            ids.is_sorted_by(|a, b| a < b),
            "lexeme ids are interned in strictly ascending order"
        );
        Self { ids }
    }

    /// The dense id of a declared Lexeme, or `None` if the lexicon does not
    /// declare it.
    pub(crate) fn get(&self, id: &LexemeId) -> Option<Lexeme> {
        self.ids.binary_search(id).ok().map(Lexeme::at)
    }

    /// The `LexemeId` a dense id stands for.
    pub(crate) fn id(&self, lexeme: Lexeme) -> LexemeId {
        self.ids[lexeme.index()]
    }

    /// How many Lexemes are declared.
    pub(crate) fn len(&self) -> usize {
        self.ids.len()
    }

    /// Every dense id, ascending.
    pub(crate) fn all(&self) -> impl Iterator<Item = Lexeme> + use<> {
        (0..self.ids.len()).map(Lexeme::at)
    }
}

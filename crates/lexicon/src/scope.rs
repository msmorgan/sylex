use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::LexemeId;
use crate::NameForm;

/// The prefix of a card name's Lexeme id.
const CARD_NAME_ID_PREFIX: &str = "name:";

/// The Lexeme id of the card name `name`.
#[must_use]
pub fn card_name_id(name: &str) -> LexemeId {
    LexemeId::new(&format!("{CARD_NAME_ID_PREFIX}{name}"))
}

/// The card names a text is in the scope of, each with the forms of it that
/// text admits. A face's text holds its own name in its Full form
/// [CR#201.5] and, where the card has a Shortened Name, in its Shortened
/// form [CR#201.5c].
///
/// A name the scope holds is the card's own name there
/// ([`crate::LexicalReading::OwnName`]), whether or not the lexicon declares
/// a card of that name; it is never another card's.
///
/// It also holds the face's Anchor Words [CR#614.12c]: the words its text
/// offers as its permanent enters and heads a bulleted ability with. An
/// Anchor Word ([`crate::LexicalReading::AnchorWord`]) stands only where the
/// scope holds it, and a Flavor Word of that spelling does not stand there.
///
/// And it holds the face's Vote Options [CR#701.38b]: the words a vote of
/// its text offers and its text then mentions. A Vote Option
/// ([`crate::LexicalReading::VoteOption`]) stands only where the scope
/// holds it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CardScope {
    /// By full name.
    names: BTreeMap<String, BTreeSet<NameForm>>,
    anchor_words: BTreeSet<String>,
    vote_options: BTreeSet<String>,
}

impl CardScope {
    /// The scope of a text that holds each of `names` in its Full form
    /// alone.
    #[must_use]
    pub fn full_names<'a>(names: impl IntoIterator<Item = &'a str>) -> Self {
        let mut scope = Self::default();
        for name in names {
            scope.insert(name, NameForm::Full);
        }
        scope
    }

    /// The scope of a text that holds each of `names` in every form: the
    /// text of a card that has a Shortened Name.
    #[must_use]
    pub fn every_form<'a>(names: impl IntoIterator<Item = &'a str>) -> Self {
        let mut scope = Self::default();
        for name in names {
            for form in NameForm::ALL {
                scope.insert(name, *form);
            }
        }
        scope
    }

    /// Puts one form of the card name `name` in scope.
    pub fn insert(&mut self, name: &str, form: NameForm) {
        self.names.entry(name.to_owned()).or_default().insert(form);
    }

    /// Whether the scope holds this form of the card name `name`.
    #[must_use]
    pub fn holds(&self, name: &str, form: NameForm) -> bool {
        self.names
            .get(name)
            .is_some_and(|forms| forms.contains(&form))
    }

    /// Whether the scope holds the card name `name` in any form.
    #[must_use]
    pub fn holds_name(&self, name: &str) -> bool {
        self.names.contains_key(name)
    }

    /// Puts the Anchor Word `word` in scope.
    ///
    /// # Panics
    /// On a word that has not the shape of one
    /// ([`crate::valid_anchor_word`]).
    pub fn insert_anchor_word(&mut self, word: &str) {
        assert!(
            crate::valid_anchor_word(word),
            "{word:?} has not the shape of an Anchor Word"
        );
        self.anchor_words.insert(word.to_owned());
    }

    /// Whether the scope holds the Anchor Word `word`.
    #[must_use]
    pub fn holds_anchor_word(&self, word: &str) -> bool {
        self.anchor_words.contains(word)
    }

    /// The Anchor Words the scope holds.
    pub fn anchor_words(&self) -> impl Iterator<Item = &str> {
        self.anchor_words.iter().map(String::as_str)
    }

    /// Puts the Vote Option `word` in scope.
    ///
    /// # Panics
    /// On a word that has not the shape of one
    /// ([`crate::valid_vote_option`]).
    pub fn insert_vote_option(&mut self, word: &str) {
        assert!(
            crate::valid_vote_option(word),
            "{word:?} has not the shape of a Vote Option"
        );
        self.vote_options.insert(word.to_owned());
    }

    /// Whether the scope holds the Vote Option `word`.
    #[must_use]
    pub fn holds_vote_option(&self, word: &str) -> bool {
        self.vote_options.contains(word)
    }

    /// The Vote Options the scope holds.
    pub fn vote_options(&self) -> impl Iterator<Item = &str> {
        self.vote_options.iter().map(String::as_str)
    }

    /// Every form of a card name the scope holds, with the name.
    pub fn forms(&self) -> impl Iterator<Item = (&str, NameForm)> {
        self.names
            .iter()
            .flat_map(|(name, forms)| forms.iter().map(|form| (name.as_str(), *form)))
    }
}

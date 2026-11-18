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

/// The card name whose Lexeme id is `card`, or `None` for the id of no card
/// name.
#[must_use]
pub fn card_name_of(card: LexemeId) -> Option<&'static str> {
    card.as_str().strip_prefix(CARD_NAME_ID_PREFIX)
}

/// The card names a text is in the scope of, each with the forms of it that
/// text admits. A face's text holds its own name in its Full form
/// [CR#201.5] and, where the card has a Shortened Name, in its Shortened
/// form [CR#201.5c].
///
/// A name the scope holds is the card's own name there
/// ([`crate::LexicalReading::OwnName`]), whether or not the lexicon declares
/// a card of that name; it is never another card's. A card-scoped Lexeme
/// whose card it holds stands there ([`crate::Lexicon::stands_in`]).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CardScope {
    /// By full name.
    names: BTreeMap<String, BTreeSet<NameForm>>,
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

    /// The scope of a text that holds, in its Full form, each card name
    /// whose Lexeme id is in `cards`: one a Lexeme scoped to those cards
    /// stands in.
    ///
    /// # Panics
    /// On the id of no card name.
    #[must_use]
    pub fn of_cards(cards: impl IntoIterator<Item = LexemeId>) -> Self {
        Self::full_names(cards.into_iter().map(|card| {
            card_name_of(card).unwrap_or_else(|| panic!("`{card}` is the id of no card name"))
        }))
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

    /// Whether the scope holds, in any form, the card name whose Lexeme id
    /// is `card` ([`card_name_id`]).
    #[must_use]
    pub fn holds_card(&self, card: LexemeId) -> bool {
        card_name_of(card).is_some_and(|name| self.holds_name(name))
    }

    /// Every form of a card name the scope holds, with the name.
    pub fn forms(&self) -> impl Iterator<Item = (&str, NameForm)> {
        self.names
            .iter()
            .flat_map(|(name, forms)| forms.iter().map(|form| (name.as_str(), *form)))
    }
}

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::LexemeId;
use crate::NameForm;

/// The card names a text is in the scope of, each with the forms of it that
/// text admits. A face's text holds its own names in every form
/// [CR#201.5,201.5c]; a scope may hold a name in its Full form alone.
///
/// A card name it holds in some form is that card's own name there, not
/// another card's, and a card-scoped Lexeme whose card it holds stands
/// there ([`crate::Lexicon::stands_in`]).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CardScope {
    names: BTreeMap<LexemeId, BTreeSet<NameForm>>,
}

impl CardScope {
    /// The scope of a face's text: its own card names, each in every form
    /// [CR#201.5c].
    #[must_use]
    pub fn own_names(names: impl IntoIterator<Item = LexemeId>) -> Self {
        let mut scope = Self::default();
        for name in names {
            for form in NameForm::ALL {
                scope.insert(name, *form);
            }
        }
        scope
    }

    /// Puts one form of a card name in scope.
    pub fn insert(&mut self, name: LexemeId, form: NameForm) {
        self.names.entry(name).or_default().insert(form);
    }

    /// Whether the scope holds this form of the card name `name`.
    #[must_use]
    pub fn holds(&self, name: LexemeId, form: NameForm) -> bool {
        self.names
            .get(&name)
            .is_some_and(|forms| forms.contains(&form))
    }

    /// Whether the scope holds the card named `name` in any form.
    #[must_use]
    pub fn holds_card(&self, name: LexemeId) -> bool {
        self.names.contains_key(&name)
    }
}

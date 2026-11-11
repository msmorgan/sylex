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

    /// Keeps only the forms of card names that `other` holds too.
    pub fn retain_common(&mut self, other: &Self) {
        self.names.retain(|name, forms| {
            other.names.get(name).is_some_and(|theirs| {
                forms.retain(|form| theirs.contains(form));
                !forms.is_empty()
            })
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retain_common_keeps_the_forms_both_hold_and_drops_a_name_left_with_none() {
        let both = LexemeId::new("name:Mishra, Claimed by Gix");
        let result = LexemeId::new("name:Mishra, Lost to Phyrexia");
        let shortened_only = LexemeId::new("name:Titania, Gaea Incarnate");
        let mut ours = CardScope::own_names([both, result]);
        ours.insert(shortened_only, NameForm::Shortened);
        let mut theirs = CardScope::own_names([both]);
        theirs.insert(result, NameForm::Full);
        theirs.insert(shortened_only, NameForm::Full);
        ours.retain_common(&theirs);
        let mut expected = CardScope::own_names([both]);
        expected.insert(result, NameForm::Full);
        assert_eq!(ours, expected);
        assert!(!ours.holds(result, NameForm::Shortened));
        assert!(!ours.holds_card(shortened_only));
    }
}

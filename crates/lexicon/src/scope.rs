use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::LexemeId;
use crate::LexicalReading;
use crate::NameForm;

/// The prefix of a card name's Lexeme id.
const CARD_NAME_ID_PREFIX: &str = "name:";

/// The Lexeme id of the card name `name`.
#[must_use]
pub fn card_name_id(name: &str) -> LexemeId {
    LexemeId::new(&format!("{CARD_NAME_ID_PREFIX}{name}"))
}

/// The parse context of a text: the card names it is in the scope of, each
/// with the forms of it that text admits, and the words its face's other
/// lines introduce and use. A face's text holds its own name in its Full
/// form [CR#201.5] and, where the card has a Shortened Name, in its
/// Shortened form [CR#201.5c].
///
/// A name the scope holds is the card's own name there
/// ([`crate::LexicalReading::OwnName`]), whether or not the lexicon declares
/// a card of that name; it is never another card's.
///
/// A face's text introduces words of three kinds, which no inventory lists:
/// an Introduced Name [CR#111.4], an Anchor Word [CR#614.12c] and a Vote
/// Option [CR#701.38b]. Its lines are read in order, and a line's scope
/// holds the words the lines before it introduce, which the line may use
/// (a name named again, an Anchor Word heading its bulleted ability, a Vote
/// Option where its votes are counted), and the words the lines after it
/// use, which discharge the choice the line offers them in (*choose Khans
/// or Dragons*; *votes for sickness or psychosis*). A Flavor Word spelled like
/// an Anchor Word the scope introduces does not stand there.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CardScope {
    /// By full name.
    names: BTreeMap<String, BTreeSet<NameForm>>,
    /// The words the lines before the text introduce.
    introduced: BTreeSet<LexicalReading>,
    /// The words the lines after the text use.
    discharged: BTreeSet<LexicalReading>,
}

/// Whether `word` is of a kind a text introduces and uses again.
fn introducible(word: &LexicalReading) -> bool {
    match word {
        LexicalReading::IntroducedName { .. }
        | LexicalReading::AnchorWord { .. }
        | LexicalReading::VoteOption { .. } => true,
        LexicalReading::Word(_)
        | LexicalReading::OwnName { .. }
        | LexicalReading::FlavorWord { .. }
        | LexicalReading::SetName { .. }
        | LexicalReading::Numeral { .. } => false,
    }
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

    /// Records that a line before the text introduces `word`, an Introduced
    /// Name, an Anchor Word or a Vote Option.
    ///
    /// # Panics
    /// On a reading of any other kind.
    pub fn introduce(&mut self, word: LexicalReading) {
        assert!(introducible(&word), "{word:?} is no word a text introduces");
        self.introduced.insert(word);
    }

    /// Whether a line before the text introduces `word`.
    #[must_use]
    pub fn introduces(&self, word: &LexicalReading) -> bool {
        self.introduced.contains(word)
    }

    /// Records that a line after the text uses `word`, an Introduced Name,
    /// an Anchor Word or a Vote Option. The use discharges a choice the
    /// text offers that word in; a name is offered in none.
    ///
    /// # Panics
    /// On a reading of any other kind.
    pub fn discharge(&mut self, word: LexicalReading) {
        assert!(introducible(&word), "{word:?} is no word a text introduces");
        self.discharged.insert(word);
    }

    /// Whether a line after the text uses `word`.
    #[must_use]
    pub fn discharges(&self, word: &LexicalReading) -> bool {
        self.discharged.contains(word)
    }

    /// States the Anchor Word `word`: the rest of the face introduces it
    /// and uses it.
    ///
    /// # Panics
    /// On a word that has not the shape of one
    /// ([`crate::valid_anchor_word`]).
    pub fn insert_anchor_word(&mut self, word: &str) {
        assert!(
            crate::valid_anchor_word(word),
            "{word:?} has not the shape of an Anchor Word"
        );
        let word = LexicalReading::AnchorWord {
            word: word.to_owned(),
        };
        self.introduced.insert(word.clone());
        self.discharged.insert(word);
    }

    /// Whether the scope introduces the Anchor Word `word`.
    #[must_use]
    pub fn holds_anchor_word(&self, word: &str) -> bool {
        self.anchor_words().any(|held| held == word)
    }

    /// The Anchor Words the scope introduces.
    pub fn anchor_words(&self) -> impl Iterator<Item = &str> {
        self.introduced.iter().filter_map(|word| {
            if let LexicalReading::AnchorWord { word } = word {
                Some(word.as_str())
            } else {
                None
            }
        })
    }

    /// States the Vote Option `word`: the rest of the face introduces it
    /// and uses it.
    ///
    /// # Panics
    /// On a word that has not the shape of one
    /// ([`crate::valid_vote_option`]).
    pub fn insert_vote_option(&mut self, word: &str) {
        assert!(
            crate::valid_vote_option(word),
            "{word:?} has not the shape of a Vote Option"
        );
        let word = LexicalReading::VoteOption {
            word: word.to_owned(),
        };
        self.introduced.insert(word.clone());
        self.discharged.insert(word);
    }

    /// The Vote Options the scope introduces.
    pub fn vote_options(&self) -> impl Iterator<Item = &str> {
        self.introduced.iter().filter_map(|word| {
            if let LexicalReading::VoteOption { word } = word {
                Some(word.as_str())
            } else {
                None
            }
        })
    }

    /// Every form of a card name the scope holds, with the name.
    pub fn forms(&self) -> impl Iterator<Item = (&str, NameForm)> {
        self.names
            .iter()
            .flat_map(|(name, forms)| forms.iter().map(|form| (name.as_str(), *form)))
    }
}

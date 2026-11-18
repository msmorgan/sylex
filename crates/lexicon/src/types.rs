//! Data-only vocabulary shared by lexical analysis and English grammar.

use serde::Deserialize;
use serde::Serialize;

use crate::FeatureBundle;

/// Stable, interned identity of a declared lexeme.
/// IDs are copied through lexical values and serialize as their original text.
/// Intern only declaration identities; arbitrary input and spellings stay
/// owned.
pub type LexemeId = dpsi::Ident;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Category {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Determinative,
    Pronoun,
    Preposition,
    Coordinator,
    Subordinator,
    Affix,
    Clitic,
    Numeral,
    /// An explicitly named inventory whose grammatical distribution is not yet
    /// mapped. It never implicitly acquires an ordinary part of speech.
    Catalog,
    Keyword,
    /// Open ability-head label, including its following em dash.
    FlavorWord,
    /// Open name of an expansion, cited before the noun *expansion*.
    SetName,
    /// Open Anchor Word [CR#614.12c], which its face's text introduces.
    AnchorWord,
    /// Open word a vote offers, a Vote Option [CR#701.38b], which its
    /// face's text introduces. It is named for no keyword action, so a grammar
    /// may name it (`docs/design/english.md` §2).
    OptionWord,
    Symbol,
}

impl Category {
    /// Whether a word of the Category hosts a prefix: a noun (*nonland*,
    /// *non-Vampire*), an adjective (*nonlegendary*) or a verb's participle
    /// (*nonattacking*).
    #[must_use]
    pub fn hosts_prefix(self) -> bool {
        matches!(self, Self::Noun | Self::Adjective | Self::Verb)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Tense {
    Present,
    Past,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Finiteness {
    Finite,
    Nonfinite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Case {
    Nominative,
    Accusative,
    Genitive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum WordForm {
    Invariant,
    /// The independent Genitive cell of a personal Pronoun paradigm.
    IndependentGenitive,
    Singular,
    Plural,
    Plain,
    Present,
    Preterite,
    Irrealis,
    GerundParticiple,
    PastParticiple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Countability {
    Count,
    Mass,
}

/// The case a word's first letter is written in, where its Lexeme takes an
/// initial capital that its declared spelling lacks
/// ([`crate::Capitalization::Initial`]). Which one a position calls for is
/// the grammar's to say, so a lexical match records it and no lexical value
/// does: a Reading derives it from its structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum LetterCase {
    /// The declared spelling.
    Lower,
    /// The declared spelling with its first letter capitalized.
    Capital,
}

/// An integer notation identity. Its parse and realization codecs belong to
/// `sylex_lexicon`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Numeral {
    Cardinal,
    Ordinal,
    Arabic(bool),
    Roman,
}

/// A compact declared-word value carried by grammar leaves and realization.
/// Source positions and declaration provenance are intentionally absent.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub struct LexicalValue {
    pub lexeme: LexemeId,
    pub form: WordForm,
    pub features: FeatureBundle,
}

/// A compact lexical alternative, independent of its source occurrence.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum LexicalReading {
    Word(LexicalValue),
    /// A card's own name in the text of its own face [CR#201.5]: the card's
    /// full name and the form of it written, Full or Shortened [CR#201.5c].
    /// It is spelled from these alone ([`crate::shortened_spelling`]) and
    /// needs no Lexeme, so a text may be read as if on a card of any name.
    /// It stands where the Card Scope holds that form of that name
    /// ([`crate::Lexicon::stands_in`]).
    OwnName {
        name: String,
        form: crate::NameForm,
    },
    /// An Introduced Name: a name the text itself gives an object, as a token
    /// it creates [CR#111.4]. No inventory lists it: the name is the title
    /// run written at its position ([`crate::Lexicon::analyze_source_scoped`]),
    /// and it is spelled as that run.
    IntroducedName {
        name: String,
    },
    FlavorWord {
        label: String,
    },
    /// A Set Name, read from the text by its title shape
    /// ([`crate::Lexicon::analyze`]).
    SetName {
        name: String,
    },
    /// An Anchor Word [CR#614.12c]: one of the two words a face's text
    /// offers as its permanent enters, each heading the bulleted ability
    /// linked to that choice [CR#607.2m]. No inventory lists it: the face's
    /// text introduces it where it offers the two, and uses it where it
    /// heads the ability ([`crate::CardScope`]). It is spelled as the word.
    AnchorWord {
        word: String,
    },
    /// A Vote Option [CR#701.38a]: one of the words a vote offers, "words
    /// with no rules meaning that are each connected to a different effect"
    /// [CR#701.38b] (*votes for sickness or psychosis*; *If psychosis gets
    /// more votes*; *each numbers vote*). No inventory lists it: the face's
    /// text introduces it where a vote offers it, and uses it where it
    /// mentions it ([`crate::CardScope`]). It is spelled as the word, which
    /// may be several (*Mines of Moria*).
    VoteOption {
        word: String,
    },
    Numeral {
        value: i32,
        notation: Numeral,
    },
}

//! Data-only vocabulary shared by lexical analysis and English grammar.

use serde::Deserialize;
use serde::Serialize;

use crate::FeatureBundle;
use crate::Onset;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
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

/// The selected capitalization of a grammatical lexical value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum SurfaceCase {
    Declared,
    Initial,
}

/// Positional distribution of one realization, independent of its host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct SurfaceFeatures {
    pub onset: Option<Onset>,
    pub initial: bool,
    pub interior: bool,
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
    pub capitalization: SurfaceCase,
}

/// A compact lexical alternative, independent of its source occurrence.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum LexicalReading {
    Word(LexicalValue),
    FlavorWord {
        label: String,
    },
    Numeral {
        value: i32,
        notation: Numeral,
        capitalization: SurfaceCase,
    },
}

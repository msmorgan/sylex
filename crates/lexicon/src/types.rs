//! Data-only vocabulary shared by lexical analysis and English grammar.

use serde::Deserialize;
use serde::Serialize;

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

/// How a prefix joins its host: solid (*nonland*) or with a hyphen
/// (*non-Vampire*; `docs/oracle-style-guide.md` §7, Negation with non-). The
/// host's lexical property selects it: an unmarked host is `Solid`.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize,
)]
pub enum Hyphenation {
    #[default]
    Solid,
    Hyphenated,
}

/// The two alternants of the genitive suffix (CGEL, Ch. 18 §4.2 [35i],
/// pp. 1595–1596): the bare genitive, an apostrophe alone, which every Plural
/// noun form ending in *s* takes, regular or irregular, a base plural included
/// (*players'*, *elves'*, *Thalakos'*), and the default *'s* genitive after any
/// other form (*player's*, *Spawn's*). A singular ending in *s* takes *'s*
/// (*Rats's*): CGEL's [35ii] makes the bare genitive only a variant there, and
/// Oracle writes *'s*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum GenitiveAlternant {
    ApostropheS,
    Bare,
}

/// The two forms of a card name: the Full name and its Shortened Name
/// [CR#201.5c].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum NameForm {
    Full,
    Shortened,
}

impl NameForm {
    pub const ALL: &[Self] = &[Self::Full, Self::Shortened];
}

/// One correlated lexical alternative.
///
/// A missing dimension is inapplicable, not a wildcard. Which combinations
/// are applicable is enforced by the lexical engine that owns morphology.
///
/// The Adjacency Agreement dimensions select a word's allomorph by a property
/// of the word next to it, as Number selects a form by its subject: the
/// context fixes each one, and no allomorph is a free choice. The relation is
/// [`RightEdge::agrees`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureBundle {
    pub number: Option<Number>,
    pub person: Option<Person>,
    pub tense: Option<Tense>,
    pub finiteness: Option<Finiteness>,
    pub case: Option<Case>,
    /// An indefinite article's: agrees with the Onset of the word after it
    /// (*a creature*, *an artifact*, *an 8/8*).
    #[serde(default)]
    pub following_onset: Option<Onset>,
    /// A prefix's: agrees with the prefix hyphenation of the host after it
    /// ([`LexicalProperties::prefix_hyphenation`](crate::LexicalProperties)).
    #[serde(default)]
    pub hyphenation: Option<Hyphenation>,
    /// The genitive suffix's: agrees with the alternant the host before it
    /// takes ([`RightEdge::genitive_host`]).
    #[serde(default)]
    pub genitive: Option<GenitiveAlternant>,
    /// A card name's form [CR#201.5c].
    #[serde(default)]
    pub name: Option<NameForm>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Onset {
    Consonant,
    Vowel,
}

/// Positional distribution of one realization, independent of its host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct SurfaceFeatures {
    pub onset: Option<Onset>,
    pub initial: bool,
    pub interior: bool,
}

/// What Adjacency Agreement reads at a word's left edge: the properties the
/// word before it agrees with, and its own dimension that agrees with that
/// word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct LeftEdge {
    /// Its Onset, which an article before it agrees with; `None` when no
    /// pronunciation is known, which no article agrees with.
    pub onset: Option<Onset>,
    /// How a prefix before it joins it.
    pub prefix_hyphenation: Hyphenation,
    /// Its [`FeatureBundle::genitive`].
    pub genitive: Option<GenitiveAlternant>,
}

/// What Adjacency Agreement reads at a word's right edge: the properties the
/// word after it agrees with, and its own dimensions that agree with that
/// word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct RightEdge {
    /// The genitive alternant a genitive suffix after it takes: the bare
    /// genitive after a Plural noun form ending in *s* (CGEL, Ch. 18 §4.2
    /// [35i], pp. 1595–1596), else the *'s* genitive.
    pub genitive_host: GenitiveAlternant,
    /// Its [`FeatureBundle::following_onset`].
    pub following_onset: Option<Onset>,
    /// Its [`FeatureBundle::hyphenation`].
    pub hyphenation: Option<Hyphenation>,
}

/// The edges of one realized word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Edges {
    pub left: LeftEdge,
    pub right: RightEdge,
}

impl Edges {
    /// The edges of a word with no Adjacency Agreement dimension, no
    /// hyphenated prefix and no Plural noun form, such as a numeral or a
    /// Flavor Word, with its Onset.
    #[must_use]
    pub fn plain(onset: Option<Onset>) -> Self {
        Self {
            left: LeftEdge {
                onset,
                prefix_hyphenation: Hyphenation::Solid,
                genitive: None,
            },
            right: RightEdge {
                genitive_host: GenitiveAlternant::ApostropheS,
                following_onset: None,
                hyphenation: None,
            },
        }
    }
}

impl RightEdge {
    /// Adjacency Agreement: whether a word with this right edge may stand
    /// immediately before one with the `next` left edge. Each agreement
    /// dimension of either word must equal the property of the other it
    /// agrees with.
    #[must_use]
    pub fn agrees(self, next: LeftEdge) -> bool {
        self.following_onset
            .is_none_or(|onset| next.onset == Some(onset))
            && self
                .hyphenation
                .is_none_or(|hyphenation| hyphenation == next.prefix_hyphenation)
            && next
                .genitive
                .is_none_or(|genitive| genitive == self.genitive_host)
    }
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

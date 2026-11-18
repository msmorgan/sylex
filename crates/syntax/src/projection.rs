//! The lexicon feature-bundle mapping table: the one declared
//! lexicon-to-syntax interface, and the only place the engine names a grammar
//! feature or value (`docs/design/english.md` §1).
//!
//! Binding resolves it against the grammar's feature domains once
//! ([`Projection`]): every licensed lexical value's leaf features are
//! resolved then, and a value the grammar's declared feature has no value for
//! is a bind error. A leaf's projected features are then a lookup.

use dpsi::Ident;
use serde::de::DeserializeOwned;
use sylex_lexicon::Case;
use sylex_lexicon::Countability;
use sylex_lexicon::Number;
use sylex_lexicon::Person;
use sylex_lexicon::Tense;
use sylex_lexicon::WordForm;

use crate::LoadError;
use crate::grammar::variant;
use crate::step::MAX_FEATURES;

/// Lexical features the engine projects onto leaves, by grammar feature name.
/// A verb form's tense is its `Time`: a present is `NonPast`, a preterite
/// `Past`, and a form with no tense leaves it unvalued.
pub(crate) const PROJECTED: [&str; 8] = [
    "Number",
    "Person",
    "Time",
    "VerbForm",
    "Case",
    "Inflection",
    "Countability",
    "Sign",
];

/// The index in [`PROJECTED`] of a numeral's or noun's Number.
pub(crate) const NUMBER: usize = 0;
const PERSON: usize = 1;
const TIME: usize = 2;
const VERB_FORM: usize = 3;
const CASE: usize = 4;
/// The index in [`PROJECTED`] of a word's inflectional Word Form.
pub(crate) const INFLECTION: usize = 5;
/// The index in [`PROJECTED`] of a noun's Countability.
const COUNTABILITY: usize = 6;
/// The index in [`PROJECTED`] of a numeral's Sign.
const SIGN: usize = 7;

/// Every lexical value the engine projects onto a grammar feature of this
/// name, whatever the lexicon licenses: a grammar that declares the feature
/// declares each of them.
pub(crate) fn projected_values(feature: &str) -> &'static [&'static str] {
    match feature {
        "Number" => &["Singular", "Plural"],
        "Sign" => &["Negative", "Zero", "Positive"],
        "Person" => &["Second", "Third"],
        "Time" => &["Past", "NonPast"],
        "VerbForm" => &["Plain", "Finite", "GerundParticiple", "PastParticiple"],
        "Case" => &["Nominative", "Accusative", "Genitive"],
        "Countability" => &["Count", "Mass"],
        "Inflection" => &[
            "Invariant",
            "IndependentGenitive",
            "Singular",
            "Plural",
            "Plain",
            "Present",
            "Preterite",
            "Irrealis",
            "GerundParticiple",
            "PastParticiple",
        ],
        _ => &[],
    }
}

/// The `VerbForm` a Word Form projects, if it is a verb's.
fn verb_form(form: WordForm) -> Option<&'static str> {
    match form {
        WordForm::Plain => Some("Plain"),
        // Irrealis *were* is a finite form, like the tensed ones.
        WordForm::Present | WordForm::Preterite | WordForm::Irrealis => Some("Finite"),
        WordForm::GerundParticiple => Some("GerundParticiple"),
        WordForm::PastParticiple => Some("PastParticiple"),
        WordForm::Invariant
        | WordForm::IndependentGenitive
        | WordForm::Singular
        | WordForm::Plural => None,
    }
}

/// The values of one projected feature by the lexical variant each names.
#[derive(Debug, Clone)]
struct Table<T> {
    /// The feature's id, where the grammar declares it.
    feature: Option<usize>,
    values: Vec<(T, u16)>,
}

impl<T: DeserializeOwned + PartialEq + Copy + std::fmt::Debug> Table<T> {
    /// The variants the grammar's values name, for a declared feature.
    fn read(feature: Option<usize>, values: &[Vec<Ident>]) -> Self {
        let values = feature.map_or_else(Vec::new, |id| {
            values[id]
                .iter()
                .enumerate()
                .filter_map(|(at, name)| Some((variant(name)?, value_index(at))))
                .collect()
        });
        Self { feature, values }
    }

    /// The (feature, value) a variant projects, or `None` if the grammar
    /// does not declare the feature.
    ///
    /// # Errors
    /// The feature is declared without a value naming the variant.
    fn project(&self, variant: T) -> Result<Option<(usize, u16)>, String> {
        let Some(feature) = self.feature else {
            return Ok(None);
        };
        self.values
            .iter()
            .find(|(v, _)| *v == variant)
            .map(|&(_, value)| Some((feature, value)))
            .ok_or_else(|| format!("{variant:?}"))
    }
}

fn value_index(at: usize) -> u16 {
    u16::try_from(at).expect("feature domains are small")
}

/// The grammar's values of the projected features, resolved at bind time.
#[derive(Debug, Clone)]
pub(crate) struct Projection {
    /// Each projected feature's id, by its index in [`PROJECTED`], where the
    /// grammar declares it.
    pub ids: [Option<usize>; 8],
    number: Table<Number>,
    person: Table<Person>,
    /// The `Time` of a preterite and of a present: `Past` and `NonPast`.
    time: Option<(usize, [u16; 2])>,
    case: Table<Case>,
    /// `VerbForm` by the Word Form that projects it.
    verb_form: Table<WordForm>,
    inflection: Table<WordForm>,
    countability: Table<Countability>,
    /// A numeral's Number: Singular and Plural.
    numeral_number: Option<(usize, [u16; 2])>,
    /// A numeral's Sign: Negative, Zero and Positive.
    sign: Option<(usize, [u16; 3])>,
}

impl Projection {
    /// Resolves the table against the grammar's features and their values.
    ///
    /// # Errors
    /// A declared projected feature without one of [`projected_values`].
    pub(crate) fn new(features: &[Ident], values: &[Vec<Ident>]) -> Result<Self, LoadError> {
        let ids = PROJECTED.map(|name| features.iter().position(|f| f == name));
        let named = |slot: usize, name: &str| -> Result<u16, LoadError> {
            let id = ids[slot].expect("only a declared feature is resolved");
            values[id]
                .iter()
                .position(|v| v == name)
                .map(value_index)
                .ok_or_else(|| {
                    LoadError::Invalid(format!(
                        "feature `{}` must include lexical value `{name}`",
                        PROJECTED[slot]
                    ))
                })
        };
        for (slot, id) in ids.iter().enumerate() {
            if id.is_some() {
                for value in projected_values(PROJECTED[slot]) {
                    named(slot, value)?;
                }
            }
        }
        let verb_form = Table {
            feature: ids[VERB_FORM],
            values: match ids[VERB_FORM] {
                Some(_) => [
                    WordForm::Plain,
                    WordForm::Present,
                    WordForm::Preterite,
                    WordForm::Irrealis,
                    WordForm::GerundParticiple,
                    WordForm::PastParticiple,
                ]
                .into_iter()
                .map(|form| {
                    let name = verb_form(form).expect("a verb's Word Form");
                    Ok((form, named(VERB_FORM, name)?))
                })
                .collect::<Result<_, LoadError>>()?,
                None => Vec::new(),
            },
        };
        let numeral_number = match ids[NUMBER] {
            Some(id) => Some((id, [named(NUMBER, "Singular")?, named(NUMBER, "Plural")?])),
            None => None,
        };
        let time = match ids[TIME] {
            Some(id) => Some((id, [named(TIME, "Past")?, named(TIME, "NonPast")?])),
            None => None,
        };
        let sign = match ids[SIGN] {
            Some(id) => Some((
                id,
                [
                    named(SIGN, "Negative")?,
                    named(SIGN, "Zero")?,
                    named(SIGN, "Positive")?,
                ],
            )),
            None => None,
        };
        Ok(Self {
            number: Table::read(ids[NUMBER], values),
            person: Table::read(ids[PERSON], values),
            time,
            case: Table::read(ids[CASE], values),
            verb_form,
            inflection: Table::read(ids[INFLECTION], values),
            countability: Table::read(ids[COUNTABILITY], values),
            numeral_number,
            sign,
            ids,
        })
    }

    /// The `Inflection` feature and its value naming a Word Form.
    ///
    /// # Errors
    /// The grammar declares no `Inflection`, or no value naming the form.
    pub(crate) fn inflection(&self, form: WordForm) -> Result<(usize, u16), LoadError> {
        self.inflection
            .project(form)
            .map_err(|form| format!("feature `Inflection` has no value `{form}`"))
            .and_then(|projected| {
                projected.ok_or_else(|| "the grammar declares no `Inflection`".to_owned())
            })
            .map_err(LoadError::Invalid)
    }

    /// The features a leaf of a licensed lexical value carries: its lexeme's
    /// own (`lexeme_features`), which outrank a projected one, then each
    /// projected feature the value carries. `countability` is the lexeme's
    /// declared Countability: a single one is projected, and a noun both
    /// count and mass is count in its plural form.
    ///
    /// # Errors
    /// A declared projected feature with no value naming the value's.
    pub(crate) fn leaf_features(
        &self,
        lexeme_features: &[(usize, u16)],
        countability: &[Countability],
        form: WordForm,
        bundle: &sylex_lexicon::FeatureBundle,
        owner: &str,
    ) -> Result<Vec<(usize, u16)>, LoadError> {
        let countability = match countability {
            [single] => Some(*single),
            _ if countability.contains(&Countability::Count)
                && bundle.number == Some(Number::Plural) =>
            {
                Some(Countability::Count)
            }
            _ => None,
        };
        // A form with no tense, a modal's preterite among them, has no time.
        let time = bundle.tense.map(|tense| {
            Ok(self.time.map(|(feature, [past, non_past])| {
                let value = match tense {
                    Tense::Past => past,
                    Tense::Present => non_past,
                };
                (feature, value)
            }))
        });
        let projected = [
            (NUMBER, bundle.number.map(|n| self.number.project(n))),
            (PERSON, bundle.person.map(|p| self.person.project(p))),
            (TIME, time),
            (
                VERB_FORM,
                verb_form(form).map(|_| self.verb_form.project(form)),
            ),
            (CASE, bundle.case.map(|c| self.case.project(c))),
            (INFLECTION, Some(self.inflection.project(form))),
            (
                COUNTABILITY,
                countability.map(|c| self.countability.project(c)),
            ),
        ];
        let mut features = lexeme_features.to_vec();
        for (slot, resolved) in projected {
            let Some(resolved) = resolved else { continue };
            let resolved = resolved.map_err(|variant| {
                LoadError::Invalid(format!(
                    "the lexicon licenses a {form:?} value of `{owner}` whose {} is {variant}, \
                     which feature `{}` has no value for",
                    PROJECTED[slot], PROJECTED[slot]
                ))
            })?;
            if let Some((feature, v)) = resolved
                && !features.iter().any(|&(f, _)| f == feature)
            {
                features.push((feature, v));
            }
        }
        Ok(features)
    }

    /// The features a numeral leaf of this value and notation carries: a
    /// cardinal's or arabic numeral's Number, and every numeral's Sign.
    pub(crate) fn numeral_features(
        &self,
        value: i32,
        notation: sylex_lexicon::Numeral,
    ) -> [Option<u16>; MAX_FEATURES] {
        let mut features = [None; MAX_FEATURES];
        if let Some((feature, [singular, plural])) = self.numeral_number
            && matches!(
                notation,
                sylex_lexicon::Numeral::Cardinal | sylex_lexicon::Numeral::Arabic(_)
            )
        {
            features[feature] = Some(if value == 1 { singular } else { plural });
        }
        if let Some((feature, [negative, zero, positive])) = self.sign {
            features[feature] = Some(match value.cmp(&0) {
                std::cmp::Ordering::Less => negative,
                std::cmp::Ordering::Equal => zero,
                std::cmp::Ordering::Greater => positive,
            });
        }
        features
    }
}

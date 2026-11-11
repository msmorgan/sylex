use std::collections::BTreeMap;
use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FormDeclaration {
    pub form: crate::WordForm,
    pub features: crate::FeatureBundle,
    /// None requests the default spelling; Some replaces it. A slot has
    /// exactly one spelling, and an unavailable slot is omitted from the
    /// lexeme's forms.
    pub surface: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LexicalProperties {
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub classes: BTreeSet<dpsi::Ident>,
    pub countability: Vec<crate::Countability>,
    /// Declared distribution features retained by name for the grammar adapter.
    pub features: BTreeMap<dpsi::Ident, dpsi::Ident>,
    /// How a prefix before the Lexeme joins it
    /// ([`crate::FeatureBundle::hyphenation`]), declared only where it is
    /// `Hyphenated`: a host is `Solid` by default, and declaring that is
    /// refused. Only a Lexeme of a Category that hosts a prefix declares it
    /// ([`crate::Category::hosts_prefix`]).
    #[serde(default)]
    pub prefix_hyphenation: Option<crate::Hyphenation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum SourceKind {
    Core,
    Plugin,
    Catalog,
    Keyword,
    Symbol,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub kind: SourceKind,
    pub path: String,
    /// Original inventory/member or declaration identity, independent of
    /// spelling.
    pub owner: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Capitalization {
    /// Only the declared spelling.
    Exact,
    /// The declared spelling, plus a variant with its first scalar uppercased
    /// when that differs. The reading records which was written
    /// ([`crate::SurfaceCase`]); no other case folding is licensed.
    Initial,
}

/// Bound entries relax only the declared word edge, never whitespace matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Binding {
    /// Both edges must be word edges.
    Free,
    /// The right edge may abut a following host.
    Prefix,
    /// The left edge may abut a preceding host.
    Suffix,
    /// Either edge may abut a host.
    Bound,
}

/// The declared internal boundaries of a lexical spelling.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum SurfaceStructure {
    /// One word: no whitespace, separator or quotation boundary. A hyphen or
    /// apostrophe must sit between word characters, except at the edges a
    /// bound form relaxes ([`Binding`]); a slash must sit between alphabetic
    /// characters.
    #[default]
    Word,
    /// An explicitly declared sequence of words separated by single spaces.
    Multiword,
    /// A declared Arabic measure pair followed by a single noun head word.
    MeasuredCompound,
    /// Catalog and notation entries retain their declared atomicity. Only the
    /// Catalog, Keyword, Symbol and Numeral categories may declare it, so an
    /// ordinary word cannot escape word validation.
    Opaque,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Lexeme {
    pub id: crate::LexemeId,
    pub lemma: String,
    pub category: crate::Category,
    pub properties: LexicalProperties,
    pub forms: Vec<FormDeclaration>,
    pub source: Source,
    pub capitalization: Capitalization,
    pub binding: Binding,
    #[serde(default)]
    pub surface_structure: SurfaceStructure,
    /// The cards whose own text alone admits the Lexeme: the cards that
    /// introduce an Introduced Name or an Anchor Word. Empty means the
    /// Lexeme is admitted everywhere, but for a card name's Shortened form,
    /// which only its own card admits [CR#201.5c].
    #[serde(default)]
    pub card_scope: BTreeSet<crate::LexemeId>,
    /// The Lexeme is a card name [CR#201.1]: a face's lexical scope holds it
    /// when the face is that card or one of its faces, and it is another
    /// card's name everywhere else. Its forms are told apart by
    /// [`crate::FeatureBundle::name`].
    #[serde(default)]
    pub card_name: bool,
    /// The Onset of every spelling of the Lexeme, declared where it is not
    /// the one its first letter gives (*hour*, *unity*; the default rule is
    /// `pronunciation::default_onset`).
    #[serde(default)]
    pub onset: Option<crate::Onset>,
}

impl Lexeme {
    #[must_use]
    pub fn invariant(
        id: impl AsRef<str>,
        lemma: impl Into<String>,
        category: crate::Category,
        source: Source,
    ) -> Self {
        Self {
            id: crate::LexemeId::new(id.as_ref()),
            lemma: lemma.into(),
            category,
            properties: LexicalProperties::default(),
            forms: vec![FormDeclaration {
                form: crate::WordForm::Invariant,
                features: crate::FeatureBundle::default(),
                surface: None,
            }],
            source,
            capitalization: Capitalization::Initial,
            binding: Binding::Free,
            card_scope: BTreeSet::new(),
            card_name: false,
            onset: None,
            surface_structure: if matches!(
                category,
                crate::Category::Catalog | crate::Category::Symbol
            ) {
                SurfaceStructure::Opaque
            } else {
                SurfaceStructure::Word
            },
        }
    }

    /// A card name [CR#201.1]: its Full form, spelled `name`, and its
    /// Shortened form [CR#201.5c] when it has one.
    #[must_use]
    pub fn card_name(
        id: impl AsRef<str>,
        name: &str,
        shortened: Option<String>,
        source: Source,
    ) -> Self {
        let mut lexeme = Self::invariant(id, name, crate::Category::Catalog, source);
        lexeme.capitalization = Capitalization::Exact;
        lexeme.card_name = true;
        lexeme.forms = [(crate::NameForm::Full, None)]
            .into_iter()
            .chain(shortened.map(|short| (crate::NameForm::Shortened, Some(short))))
            .map(|(form, surface)| FormDeclaration {
                form: crate::WordForm::Invariant,
                features: crate::FeatureBundle {
                    name: Some(form),
                    ..crate::FeatureBundle::default()
                },
                surface,
            })
            .collect();
        lexeme
    }

    #[must_use]
    pub fn noun(
        id: impl AsRef<str>,
        lemma: impl Into<String>,
        countability: Vec<crate::Countability>,
        source: Source,
    ) -> Self {
        let mut lexeme = Self::invariant(id, lemma, crate::Category::Noun, source);
        lexeme.properties.countability = countability;
        lexeme.forms = [crate::WordForm::Singular, crate::WordForm::Plural]
            .into_iter()
            .filter(|form| {
                *form != crate::WordForm::Plural
                    || lexeme
                        .properties
                        .countability
                        .contains(&crate::Countability::Count)
            })
            .map(|form| FormDeclaration {
                form,
                features: crate::FeatureBundle {
                    number: Some(if form == crate::WordForm::Singular {
                        crate::Number::Singular
                    } else {
                        crate::Number::Plural
                    }),
                    ..crate::FeatureBundle::default()
                },
                surface: None,
            })
            .collect();
        lexeme
    }

    /// Declares a regular verb's primary bundles and secondary forms.
    /// Callers replace irregular slots or remove unavailable slots before
    /// indexing.
    #[must_use]
    pub fn verb(id: impl AsRef<str>, lemma: impl Into<String>, source: Source) -> Self {
        let mut lexeme = Self::invariant(id, lemma, crate::Category::Verb, source);
        lexeme.forms = [
            crate::WordForm::Plain,
            crate::WordForm::GerundParticiple,
            crate::WordForm::PastParticiple,
        ]
        .into_iter()
        .map(|form| FormDeclaration {
            form,
            features: crate::FeatureBundle {
                finiteness: (form != crate::WordForm::Plain)
                    .then_some(crate::Finiteness::Nonfinite),
                ..crate::FeatureBundle::default()
            },
            surface: None,
        })
        .collect();
        for (form, tense) in [
            (crate::WordForm::Present, crate::Tense::Present),
            (crate::WordForm::Preterite, crate::Tense::Past),
        ] {
            for number in [crate::Number::Singular, crate::Number::Plural] {
                for person in [
                    crate::Person::First,
                    crate::Person::Second,
                    crate::Person::Third,
                ] {
                    lexeme.forms.push(FormDeclaration {
                        form,
                        features: crate::FeatureBundle {
                            number: Some(number),
                            person: Some(person),
                            tense: Some(tense),
                            finiteness: Some(crate::Finiteness::Finite),
                            ..crate::FeatureBundle::default()
                        },
                        surface: None,
                    });
                }
            }
        }
        lexeme
    }

    /// Merges each finite Singular slot with the Plural slot of the same form,
    /// person and spelling into one slot unmarked for number: that verb form
    /// does not agree in number (*entered*, *can*). Call it after spelling.
    pub fn merge_number_neutral_forms(&mut self) {
        let spelled: Vec<String> = self
            .forms
            .iter()
            .map(|slot| {
                slot.surface
                    .clone()
                    .unwrap_or_else(|| crate::default_surface(self, slot.form, &slot.features))
            })
            .collect();
        let mut merged = vec![false; self.forms.len()];
        for singular in 0..self.forms.len() {
            if self.forms[singular].features.finiteness != Some(crate::Finiteness::Finite)
                || self.forms[singular].features.number != Some(crate::Number::Singular)
            {
                continue;
            }
            let plural = crate::FeatureBundle {
                number: Some(crate::Number::Plural),
                ..self.forms[singular].features.clone()
            };
            let Some(partner) = (0..self.forms.len()).find(|&i| {
                !merged[i]
                    && self.forms[i].form == self.forms[singular].form
                    && self.forms[i].features == plural
                    && spelled[i] == spelled[singular]
            }) else {
                continue;
            };
            merged[partner] = true;
            let slot = &mut self.forms[singular];
            slot.features.number = None;
            slot.surface = Some(spelled[singular].clone());
        }
        let mut merged = merged.into_iter();
        self.forms.retain(|_| !merged.next().unwrap_or(false));
    }
}

//! The core English lexicon and the Catalogs, as one Lexical Environment for
//! `sylex_syntax`.
//!
//! Core vocabulary (`lexicon/english/`) is declared with Core Provenance and
//! is the only vocabulary a grammar may name. Catalog items (`catalogs/`) and
//! card names carry Catalog Provenance and reach a grammar only through their
//! Category and Subcategorization Frames.

mod build;
#[macro_use]
mod data_text;
mod names;
pub mod schema;
mod shortening;

use std::path::PathBuf;
use std::sync::LazyLock;

use dpsi::Ident;
use serde::de::DeserializeOwned;
use sylex_lexicon::Binding;
use sylex_lexicon::Category;
use sylex_lexicon::Lexeme;
use sylex_lexicon::LexicalError;
use sylex_lexicon::Lexicon;
use sylex_lexicon::SourceKind;
use sylex_lexicon::SurfaceStructure;
use sylex_syntax::FrameCatalog;

use crate::build::Builder;
use crate::build::allomorphic;
use crate::build::bound;
use crate::build::exact;
use crate::build::source;
pub use crate::data_text::read_data_file;
pub use crate::names::card_name_id;
pub use crate::names::face_scope;
pub use crate::names::shortened_name;
pub use crate::names::stated_scope;
use crate::schema::AbilityWord;
use crate::schema::AdjectiveDecl;
use crate::schema::AdverbEntry;
use crate::schema::ClosedClass;
use crate::schema::Designation;
use crate::schema::KeywordAbility;
use crate::schema::KeywordAction;
use crate::schema::NounDecl;
use crate::schema::PrepositionEntry;
use crate::schema::StemClass;
use crate::schema::SubtypeFamily;
use crate::schema::Subtypes;
use crate::schema::Token;
use crate::schema::TurnPart;
use crate::schema::Types;
use crate::schema::VerbDecl;
use crate::schema::VerbEntry;
pub use crate::shortening::legendary_permanent;
pub use crate::shortening::parse_type_line;
pub use crate::shortening::short_name;

/// The license feature of a keyword a keyword counter can be and of the
/// noun *counter* (`Sources::counter_kinds`).
const KIND_LICENSE: &str = "KindLicense";

/// Marks each word of a closed class that stands before a noun or heads its
/// own phrase there ([`sylex_lexicon::LexicalProperties::closed_class`]): a
/// determinative but one that is a fused head alone (*everything*, *anywhere*:
/// `Independent`), a pronoun, a preposition, a coordinator, a subordinator, and
/// an auxiliary verb, modals among them.
fn mark_closed_class(lexemes: &mut [Lexeme]) {
    for lexeme in lexemes {
        let feature = |name: &str| lexeme.properties.features.get(name).map(Ident::as_str);
        lexeme.properties.closed_class = match lexeme.category {
            Category::Determinative => feature("DeterminativeDistribution") != Some("Independent"),
            Category::Pronoun
            | Category::Preposition
            | Category::Coordinator
            | Category::Subordinator => true,
            Category::Verb => matches!(feature("VerbKind"), Some("Auxiliary" | "Modal")),
            Category::Noun
            | Category::Adjective
            | Category::Adverb
            | Category::Affix
            | Category::Clitic
            | Category::Numeral
            | Category::Catalog
            | Category::Keyword
            | Category::FlavorWord
            | Category::SetName
            | Category::KindName
            | Category::AnchorWord
            | Category::OptionWord
            | Category::Symbol => false,
        };
    }
}

/// Why the lexicon or a catalog cannot be loaded.
#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    #[error("{path}: {message}")]
    Format { path: String, message: String },
    #[error("{0}")]
    Invalid(String),
    #[error(transparent)]
    Lexical(#[from] LexicalError),
    #[error("{path}: {message}")]
    Io { path: String, message: String },
}

/// Every data file of this crate, by path: embedded at build time in release
/// and wasm builds, read at run time in dev builds ([`data_text!`]).
static FILES: [(&str, LazyLock<String>); 16] = [
    (CLOSED_CLASS, data_text!("lexicon/english/closed_class.ron")),
    (PREPOSITIONS, data_text!("lexicon/english/prepositions.ron")),
    (VERBS, data_text!("lexicon/english/verbs.ron")),
    (NOUNS, data_text!("lexicon/english/nouns.ron")),
    (ADJECTIVES, data_text!("lexicon/english/adjectives.ron")),
    (ADVERBS, data_text!("lexicon/english/adverbs.ron")),
    (TYPES, data_text!("catalogs/types.ron")),
    (SUBTYPES, data_text!("catalogs/subtypes.ron")),
    (CATALOG_VERBS, data_text!("catalogs/verbs.ron")),
    (KEYWORD_ACTIONS, data_text!("catalogs/keyword_actions.ron")),
    (
        KEYWORD_ABILITIES,
        data_text!("catalogs/keyword_abilities.ron"),
    ),
    (ABILITY_WORDS, data_text!("catalogs/ability_words.ron")),
    (DESIGNATIONS, data_text!("catalogs/designations.ron")),
    (STATUSES, data_text!("catalogs/statuses.ron")),
    (TURN_PARTS, data_text!("catalogs/turn_parts.ron")),
    (TOKENS, data_text!("catalogs/tokens.ron")),
];

pub(crate) const CLOSED_CLASS: &str = "lexicon/english/closed_class.ron";
const PREPOSITIONS: &str = "lexicon/english/prepositions.ron";
const VERBS: &str = "lexicon/english/verbs.ron";
const NOUNS: &str = "lexicon/english/nouns.ron";
const ADJECTIVES: &str = "lexicon/english/adjectives.ron";
const ADVERBS: &str = "lexicon/english/adverbs.ron";
const TYPES: &str = "catalogs/types.ron";
const SUBTYPES: &str = "catalogs/subtypes.ron";
const CATALOG_VERBS: &str = "catalogs/verbs.ron";
const KEYWORD_ACTIONS: &str = "catalogs/keyword_actions.ron";
/// The class of a Keyword Lexeme whose face has its bracketed text removable:
/// [`schema::KeywordAbility::bracketed_text_removable`].
pub const BRACKETED_TEXT_REMOVABLE: &str = "BracketedTextRemovable";
const KEYWORD_ABILITIES: &str = "catalogs/keyword_abilities.ron";
const ABILITY_WORDS: &str = "catalogs/ability_words.ron";
const DESIGNATIONS: &str = "catalogs/designations.ron";
const STATUSES: &str = "catalogs/statuses.ron";
const TURN_PARTS: &str = "catalogs/turn_parts.ron";
const TOKENS: &str = "catalogs/tokens.ron";
/// The card-name catalog, relative to the repository root.
pub const CARD_NAMES: &str = "data/gen/catalogs/card-names.txt";
/// The dungeon-name catalog, relative to the repository root.
pub const DUNGEON_NAMES: &str = "data/gen/catalogs/dungeon-names.txt";
fn parse<T: DeserializeOwned>(path: &str) -> Result<T, SourceError> {
    let text = FILES
        .iter()
        .find(|(p, _)| *p == path)
        .map(|(_, text)| text.as_str())
        .ok_or_else(|| SourceError::Invalid(format!("no data file {path}")))?;
    // Optional fields are written bare: `plural: "Elves"`, not `Some("Elves")`.
    ron::Options::default()
        .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
        .from_str(text)
        .map_err(|error| SourceError::Format {
            path: path.to_owned(),
            message: error.to_string(),
        })
}

/// Every authored lexicon and catalog file, parsed.
#[derive(Debug, Clone)]
pub struct Sources {
    pub closed_class: ClosedClass,
    pub prepositions: Vec<PrepositionEntry>,
    pub verbs: Vec<VerbEntry>,
    pub nouns: Vec<NounDecl>,
    pub adjectives: Vec<AdjectiveDecl>,
    pub adverbs: Vec<AdverbEntry>,
    pub types: Types,
    pub subtypes: Subtypes,
    pub catalog_verbs: Vec<VerbEntry>,
    pub keyword_actions: Vec<KeywordAction>,
    pub keyword_abilities: Vec<KeywordAbility>,
    pub ability_words: Vec<AbilityWord>,
    pub designations: Vec<Designation>,
    pub turn_parts: Vec<TurnPart>,
    pub statuses: Vec<schema::Status>,
    pub tokens: Vec<Token>,
    /// The names of the dungeon cards, read from [`DUNGEON_NAMES`].
    pub dungeon_names: Vec<String>,
}

/// A Lexicon and the Subcategorization Frames of its Lexemes.
#[derive(Debug)]
pub struct LexicalEnvironment {
    pub lexicon: Lexicon,
    pub frames: FrameCatalog,
}

impl LexicalEnvironment {
    /// Loads every data file, the dungeon names at [`dungeon_names_path`] and
    /// the card names at [`card_names_path`]. It reads no card's text or type
    /// line: a face's Card Scope is derived when its text is parsed
    /// ([`face_scope`]).
    ///
    /// # Errors
    /// An unreadable card-name file, or any [`Sources::environment`] error.
    pub fn load() -> Result<Self, SourceError> {
        let names = read(&card_names_path())?;
        Sources::load()?.environment(&names)
    }
}

fn read(path: &std::path::Path) -> Result<String, SourceError> {
    std::fs::read_to_string(path).map_err(|error| SourceError::Io {
        path: path.display().to_string(),
        message: error.to_string(),
    })
}

/// The card-name catalog in this repository.
#[must_use]
pub fn card_names_path() -> PathBuf {
    repository_path(CARD_NAMES)
}

/// The dungeon-name catalog in this repository.
#[must_use]
pub fn dungeon_names_path() -> PathBuf {
    repository_path(DUNGEON_NAMES)
}

fn repository_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

/// The lines of a generated name catalog, each a name.
fn generated_names(path: &std::path::Path) -> Result<Vec<String>, SourceError> {
    read(path)?
        .lines()
        .enumerate()
        .map(|(index, line)| {
            if line.trim().is_empty() || line.trim() != line {
                return Err(SourceError::Format {
                    path: path.display().to_string(),
                    message: format!("line {} is no name: {line:?}", index + 1),
                });
            }
            Ok(line.to_owned())
        })
        .collect()
}

impl Sources {
    /// # Errors
    /// Malformed RON in any data file, or an unreadable or malformed
    /// dungeon-name catalog.
    pub fn load() -> Result<Self, SourceError> {
        Ok(Self {
            closed_class: parse(CLOSED_CLASS)?,
            prepositions: parse(PREPOSITIONS)?,
            verbs: parse(VERBS)?,
            nouns: parse(NOUNS)?,
            adjectives: parse(ADJECTIVES)?,
            adverbs: parse(ADVERBS)?,
            types: parse(TYPES)?,
            subtypes: parse(SUBTYPES)?,
            catalog_verbs: parse(CATALOG_VERBS)?,
            keyword_actions: parse(KEYWORD_ACTIONS)?,
            keyword_abilities: parse(KEYWORD_ABILITIES)?,
            ability_words: parse(ABILITY_WORDS)?,
            designations: parse(DESIGNATIONS)?,
            turn_parts: parse(TURN_PARTS)?,
            statuses: parse(STATUSES)?,
            tokens: parse(TOKENS)?,
            dungeon_names: generated_names(&dungeon_names_path())?,
        })
    }

    /// The core lexicon and every catalog except card names.
    ///
    /// # Errors
    /// An invalid declaration: a duplicate identity, a frame naming an
    /// undeclared preposition, a verb without frames, a word declared both in
    /// the core lexicon and a catalog, or anything [`Lexicon::new`] rejects.
    pub fn vocabulary(&self) -> Result<(Vec<Lexeme>, FrameCatalog), SourceError> {
        let mut builder = Builder::default();
        self.core(&mut builder)?;
        self.catalogs(&mut builder)?;
        let (mut lexemes, mut frames) = builder.finish()?;
        self.parameter_patterns(&mut frames);
        check_core_and_catalogs_disjoint(&lexemes)?;
        mark_lexical_adjectival_passives(&mut lexemes)?;
        mark_closed_class(&mut lexemes);
        Ok((lexemes, frames))
    }

    /// The whole Lexical Environment, with card names read from `card_names`
    /// (one per line).
    ///
    /// # Errors
    /// Any [`Sources::vocabulary`] error.
    pub fn environment(&self, card_names: &str) -> Result<LexicalEnvironment, SourceError> {
        let (mut lexemes, frames) = self.vocabulary()?;
        let card_names = names::card_names(card_names);
        lexemes.extend(card_names);
        Ok(LexicalEnvironment {
            lexicon: Lexicon::new(lexemes)?,
            frames,
        })
    }

    fn core(&self, builder: &mut Builder) -> Result<(), SourceError> {
        let core = |path: &str, owner: &str| source(SourceKind::Core, path, owner);
        let closed = &self.closed_class;
        for entry in &closed.determinatives {
            let id = match &entry.sense {
                Some(sense) => format!("det:{}.{sense}", entry.lemma),
                None => format!("det:{}", entry.lemma),
            };
            builder.determinative(&id, entry, core(CLOSED_CLASS, &id));
        }
        for entry in &closed.adverbs {
            builder.adverb(entry, core(CLOSED_CLASS, &format!("adv:{}", entry.lemma)));
        }
        for entry in &closed.pronouns {
            let id = format!("pronoun:{}", entry.lemma);
            let mut lexeme = Lexeme::invariant(
                &id,
                &entry.lemma,
                Category::Pronoun,
                core(CLOSED_CLASS, &id),
            );
            lexeme.properties.features.insert(
                "PronounKind".into(),
                Ident::new(&format!("{:?}", entry.kind)),
            );
            match (entry.kind, entry.relative_gender) {
                (_, None) => {}
                (
                    schema::PronounKind::Relative,
                    Some(
                        gender @ (schema::RelativeGender::Personal
                        | schema::RelativeGender::NonPersonal),
                    ),
                ) => {
                    lexeme
                        .properties
                        .features
                        .insert("RelativeGender".into(), Ident::new(&format!("{gender:?}")));
                }
                (kind, Some(gender)) => {
                    return Err(SourceError::Invalid(format!(
                        "pronoun `{id}`: a {kind:?} pronoun with relative gender {gender:?}; only a \
                         relative pronoun has one, personal or non-personal"
                    )));
                }
            }
            lexeme.forms = entry
                .forms
                .iter()
                .map(|form| sylex_lexicon::FormDeclaration {
                    form: form.form,
                    features: sylex_lexicon::FeatureBundle {
                        person: Some(form.person),
                        number: Some(form.number),
                        case: form.case,
                        ..Default::default()
                    },
                    surface: Some(form.surface.clone()),
                })
                .collect();
            builder.push(lexeme);
        }
        for (prefix, category, words) in [
            ("coord", Category::Coordinator, &closed.coordinators),
            ("sub", Category::Subordinator, &closed.subordinators),
        ] {
            for word in words {
                let id = format!("{prefix}:{word}");
                builder.push(Lexeme::invariant(
                    &id,
                    word,
                    category,
                    core(CLOSED_CLASS, &id),
                ));
            }
        }
        for entry in &closed.auxiliaries {
            let id = format!("verb:{}", entry.lemma);
            builder.verb(&id, entry, core(CLOSED_CLASS, &id))?;
        }
        for entry in &closed.affixes {
            let id = format!("affix:{}", entry.name);
            let mut lexeme = allomorphic(
                &id,
                Category::Affix,
                entry.allomorphs.iter().map(|(hyphenation, surface)| {
                    let features = sylex_lexicon::FeatureBundle {
                        hyphenation: Some(*hyphenation),
                        ..Default::default()
                    };
                    (features, surface.clone())
                }),
                entry.binding,
                core(CLOSED_CLASS, &id),
            );
            // Initial capitalization belongs to the affix, so sentence-initial
            // forms need no duplicate capitalized catalog spellings.
            lexeme.capitalization = sylex_lexicon::Capitalization::Initial;
            builder.push(lexeme);
        }
        for entry in &closed.clitics {
            let id = format!("clitic:{}", entry.name);
            builder.push(allomorphic(
                &id,
                Category::Clitic,
                entry.allomorphs.iter().map(|(genitive, surface)| {
                    let features = sylex_lexicon::FeatureBundle {
                        genitive: Some(*genitive),
                        ..Default::default()
                    };
                    (features, surface.clone())
                }),
                entry.binding,
                core(CLOSED_CLASS, &id),
            ));
        }
        for symbol in &closed.symbols {
            let id = format!("symbol:{}", symbol.surface);
            let mut lexeme = exact(
                &id,
                &symbol.surface,
                Category::Symbol,
                source(SourceKind::Symbol, CLOSED_CLASS, &id),
            );
            lexeme
                .properties
                .classes
                .extend(symbol.classes.iter().map(|class| Ident::new(class)));
            lexeme.onset = symbol.onset;
            builder.push(lexeme);
        }
        for entry in &self.prepositions {
            let id = format!("prep:{}", entry.lemma);
            builder.preposition(entry, core(PREPOSITIONS, &id));
        }
        for entry in &self.verbs {
            let id = format!("verb:{}", entry.lemma);
            builder.verb(&id, entry, core(VERBS, &id))?;
        }
        for decl in &self.nouns {
            let entry = decl.entry();
            let id = format!("noun:{}", entry.lemma);
            builder.noun(&id, &entry, core(NOUNS, &id))?;
        }
        for decl in &self.adjectives {
            let entry = decl.entry();
            builder.adjective("adj", &entry, &core(ADJECTIVES, &entry.lemma));
        }
        for entry in &self.adverbs {
            builder.adverb(entry, core(ADVERBS, &format!("adv:{}", entry.lemma)));
        }
        Ok(())
    }

    fn statuses_and_turn_parts(&self, builder: &mut Builder) -> Result<(), SourceError> {
        let catalog = |path: &str, owner: &str| source(SourceKind::Catalog, path, owner);
        for status in &self.statuses {
            let source = catalog(STATUSES, status.rule.number());
            for adjective in &status.adjectives {
                builder.adjective("status_adj", &adjective.entry(), &source);
            }
            for preposition in &status.prepositions {
                builder.preposition(preposition, source.clone());
            }
        }
        for part in &self.turn_parts {
            let mut entry = part.noun.entry();
            entry.kind = schema::NounKind::Temporal;
            // A phase or step is a part of a turn [CR#500.1], and its of-phrase
            // is a Complement naming the turn, the whole it is a part of (*the
            // first combat phase of the turn*; CGEL, Ch. 5, §14.1, p. 442,
            // [7vi]), or the player whose turn it is [CR#102.1], a term in
            // that relation (*the upkeep of enchanted creature's controller*).
            // Every phase and step has it, so it is declared here once. Its
            // genitive names the player whose turn it is beside the whole, two
            // complements (*your first main phase of the game*, Chancellor of
            // the Tangle), as *Van Dyck's portrait of Charles I* has (CGEL,
            // Ch. 5, §16.5.3, p. 475, [56i]).
            entry.distinct_genitive = true;
            entry
                .frames
                .push(schema::FrameSpec::Complement(schema::FillSpec::PPWith(
                    vec!["of".into()],
                    Box::new(schema::FillSpec::NP),
                )));
            let id = format!("turn_part:{}", entry.lemma);
            builder.noun(&id, &entry, catalog(TURN_PARTS, part.rule.number()))?;
        }
        Ok(())
    }

    fn catalogs(&self, builder: &mut Builder) -> Result<(), SourceError> {
        let catalog = |path: &str, owner: &str| source(SourceKind::Catalog, path, owner);
        for decl in &self.types.card_types {
            let entry = decl.entry();
            let id = format!("type:{}", entry.lemma);
            builder.noun(&id, &entry, catalog(TYPES, &id))?;
            builder
                .lexemes
                .last_mut()
                .unwrap()
                .properties
                .classes
                .insert("CardType".into());
        }
        for lemma in &self.types.supertypes {
            let id = format!("supertype:{lemma}");
            builder.push(Lexeme::invariant(
                &id,
                lemma,
                Category::Adjective,
                catalog(TYPES, &id),
            ));
        }
        for lexeme in &mut builder.lexemes {
            if lexeme.id.as_str().starts_with("supertype:") {
                lexeme.properties.classes.insert("Supertype".into());
                // A supertype word names a Quality as a fused head
                // ([CR#205.4a]; *protection from snow*).
                lexeme
                    .properties
                    .features
                    .insert("QualityHead".into(), Ident::new("Fused"));
            }
        }
        let mut subtypes =
            std::collections::BTreeMap::<String, (NounDecl, Vec<SubtypeFamily>)>::new();
        for (family, decls) in &self.subtypes {
            for decl in decls {
                let lemma = decl.entry().lemma;
                let slot = subtypes
                    .entry(lemma)
                    .or_insert_with(|| (decl.clone(), Vec::new()));
                if slot.0 != *decl {
                    return Err(SourceError::Invalid(format!(
                        "subtype `{}` is declared differently in two families",
                        slot.0.entry().lemma
                    )));
                }
                slot.1.push(*family);
            }
        }
        for (lemma, (decl, families)) in &subtypes {
            let id = format!("subtype:{lemma}");
            // A prefix joins a subtype with a hyphen (*non-Vampire*;
            // `docs/oracle-style-guide.md` §7, Negation with non-), so no
            // subtype declares it.
            let mut entry = decl.entry();
            if entry.prefix_hyphenation.is_some() {
                return Err(SourceError::Invalid(format!(
                    "subtype `{lemma}` declares a prefix hyphenation, which every subtype takes \
                     hyphenated"
                )));
            }
            entry.prefix_hyphenation = Some(sylex_lexicon::Hyphenation::Hyphenated);
            // A creature or planeswalker type names a being, which takes *who*
            // or *which* as an animal does (CGEL, Ch. 5 §17.3, p. 498, [47i]);
            // every other subtype names a thing, which takes *which* ([48]). So
            // its family decides, and no subtype declares a gender.
            if entry.relative_gender.is_some() {
                return Err(SourceError::Invalid(format!(
                    "subtype `{lemma}` declares a relative gender, which its family decides"
                )));
            }
            if families.iter().any(|family| {
                matches!(
                    family,
                    SubtypeFamily::Creature | SubtypeFamily::Planeswalker
                )
            }) {
                entry.relative_gender = Some(schema::RelativeGender::Dual);
            }
            builder.noun(&id, &entry, catalog(SUBTYPES, &id))?;
            let lexeme = builder.lexemes.last_mut().expect("just pushed");
            lexeme.properties.classes = families
                .iter()
                .map(|family| Ident::new(&format!("{family:?}Type")))
                .chain(std::iter::once(Ident::new("Subtype")))
                .collect();
        }
        self.keywords(builder)?;
        self.ability_labels(builder);
        for designation in &self.designations {
            for decl in &designation.nouns {
                let entry = decl.entry();
                let id = format!("designation:{}", entry.lemma);
                builder.noun(
                    &id,
                    &entry,
                    catalog(DESIGNATIONS, designation.rule.number()),
                )?;
            }
            for lemma in &designation.adjectives {
                let id = format!("designation:{lemma}");
                builder.push(Lexeme::invariant(
                    &id,
                    lemma,
                    Category::Adjective,
                    catalog(DESIGNATIONS, designation.rule.number()),
                ));
            }
        }
        self.statuses_and_turn_parts(builder)?;
        for token in &self.tokens {
            if !subtypes.contains_key(&token.subtype) {
                return Err(SourceError::Invalid(format!(
                    "token `{}` names `{}`, which is not a declared subtype",
                    token.name, token.subtype
                )));
            }
            for name in &token.names {
                let id = format!("token:{name}");
                builder.push(exact(
                    &id,
                    name,
                    Category::Catalog,
                    catalog(TOKENS, token.rule.number()),
                ));
            }
        }
        for name in &self.dungeon_names {
            builder.push(dungeon_name(name));
        }
        self.counter_kinds(builder)
    }

    /// The license of a counter's kind (`KindLicense`, `docs/design/english.md`
    /// §3). A counter's kind premodifies the noun *counter* [CR#122.1]: a
    /// slash measure for a +X/+Y counter [CR#122.1a], a keyword for a keyword
    /// counter [CR#122.1b], and any other kind its Kind Name, read from the
    /// text (*a charge counter*). The keywords a keyword counter can be hold
    /// the license, and *counter*, the one noun that takes a kind, carries
    /// its head and is where analysis reads a Kind Name.
    fn counter_kinds(&self, builder: &mut Builder) -> Result<(), SourceError> {
        for keyword in self
            .keyword_abilities
            .iter()
            .filter(|keyword| keyword.counter)
        {
            let id = format!("keyword:{}", keyword.lemma);
            builder
                .lexemes
                .iter_mut()
                .find(|lexeme| lexeme.id.as_str() == id)
                .ok_or_else(|| {
                    SourceError::Invalid(format!(
                        "keyword counter `{}` names no declared keyword ability",
                        keyword.lemma
                    ))
                })?
                .properties
                .features
                .insert(KIND_LICENSE.into(), Ident::new("Kind"));
        }
        let head = builder
            .lexemes
            .iter_mut()
            .find(|lexeme| lexeme.id.as_str() == "noun:counter")
            .ok_or_else(|| {
                SourceError::Invalid(
                    "a counter's kind premodifies the noun `counter`, which the core lexicon does not declare".to_owned(),
                )
            })?;
        head.kind_name_host = true;
        head.properties
            .features
            .insert(KIND_LICENSE.into(), Ident::new("Head"));
        Ok(())
    }

    fn ability_labels(&self, builder: &mut Builder) {
        let catalog = |path: &str, owner: &str| source(SourceKind::Catalog, path, owner);
        for word in &self.ability_words {
            let id = format!("ability_word:{}", word.lemma);
            let mut lexeme = Lexeme::invariant(
                &id,
                &word.lemma,
                Category::Keyword,
                catalog(ABILITY_WORDS, &id),
            );
            lexeme.surface_structure = SurfaceStructure::Opaque;
            lexeme
                .properties
                .features
                .insert("LabelKind".into(), "AbilityWord".into());
            lexeme.properties.classes.insert("AbilityWord".into());
            builder.push(lexeme);
        }
    }

    /// Shared catalog verbs, keyword actions and keyword abilities, with their
    /// variants, verbs, Bound Keyword Surfaces and stems.
    fn keywords(&self, builder: &mut Builder) -> Result<(), SourceError> {
        let catalog = |path: &str, owner: &str| source(SourceKind::Catalog, path, owner);
        let mut shared_verbs = SharedVerbs::build(&self.catalog_verbs, builder)?;
        for action in &self.keyword_actions {
            let owner = action.rule.number();
            let entry_label = format!("keyword action `{}` ({owner})", action.name);
            for decl in &action.verbs {
                shared_verbs.resolve(
                    builder,
                    decl,
                    &entry_label,
                    "action",
                    catalog(KEYWORD_ACTIONS, owner),
                )?;
            }
            for adjective in &action.adjectives {
                builder.adjective(
                    "action_adj",
                    &adjective.entry(),
                    &catalog(KEYWORD_ACTIONS, owner),
                );
            }
            for decl in &action.nouns {
                let entry = decl.entry();
                let id = format!("action:{}", entry.lemma);
                builder.noun(&id, &entry, catalog(KEYWORD_ACTIONS, owner))?;
            }
        }
        let mut stems = std::collections::BTreeSet::new();
        for ability in &self.keyword_abilities {
            let owner = ability.rule.number();
            let entry_label = format!("keyword ability `{}` ({owner})", ability.name);
            let lemmas = std::iter::once(&ability.lemma)
                .chain(ability.variants.iter().map(|variant| &variant.lemma));
            for lemma in lemmas {
                let id = format!("keyword:{lemma}");
                let mut lexeme = Lexeme::invariant(
                    &id,
                    lemma,
                    Category::Keyword,
                    catalog(KEYWORD_ABILITIES, owner),
                );
                lexeme.surface_structure = SurfaceStructure::Opaque;
                let parameters = if lemma == &ability.lemma {
                    &ability.parameters
                } else {
                    &ability
                        .variants
                        .iter()
                        .find(|v| &v.lemma == lemma)
                        .unwrap()
                        .parameters
                };
                if !parameters.is_empty() && parameters.iter().all(|p| !p.is_empty()) {
                    lexeme
                        .properties
                        .classes
                        .insert("ParameterizedKeyword".into());
                }
                if ability.bracketed_text_removable {
                    lexeme
                        .properties
                        .classes
                        .insert(BRACKETED_TEXT_REMOVABLE.into());
                }
                builder.push(lexeme);
            }
            for entry in &ability.determinatives {
                let id = format!("keyword_det:{}", entry.lemma);
                builder.determinative(&id, entry, catalog(KEYWORD_ABILITIES, owner));
            }
            for decl in &ability.verbs {
                shared_verbs.resolve(
                    builder,
                    decl,
                    &entry_label,
                    "keyword_verb",
                    catalog(KEYWORD_ABILITIES, owner),
                )?;
            }
            for decl in &ability.nouns {
                let entry = decl.entry();
                let id = format!("keyword_noun:{}", entry.lemma);
                builder.noun(&id, &entry, catalog(KEYWORD_ABILITIES, owner))?;
            }
            if let Some(surface) = &ability.bound {
                builder.push(bound(
                    &format!("keyword:{}:bound", ability.lemma),
                    &surface.surface,
                    Category::Keyword,
                    Binding::Suffix,
                    catalog(KEYWORD_ABILITIES, owner),
                ));
                for class in &surface.stems {
                    stems.extend(self.stems(*class));
                }
            }
        }
        shared_verbs.check_all_used()?;
        for stem in stems {
            let id = format!("stem:{stem}");
            builder.push(bound(
                &id,
                &stem,
                Category::Noun,
                Binding::Prefix,
                catalog(KEYWORD_ABILITIES, &id),
            ));
            builder.lexemes.last_mut().unwrap().capitalization =
                sylex_lexicon::Capitalization::Initial;
        }
        Ok(())
    }

    fn parameter_patterns(&self, frames: &mut FrameCatalog) {
        use schema::KeywordParameter;
        use sylex_syntax::frames::ParameterPart;
        use sylex_syntax::frames::SlotKind;
        let convert = |parameters: &[KeywordParameter]| {
            let mut parts = vec![ParameterPart::Head];
            for parameter in parameters {
                match parameter {
                    KeywordParameter::Amount => parts.push(ParameterPart::Slot(SlotKind::Amount)),
                    KeywordParameter::Cost => parts.push(ParameterPart::Slot(SlotKind::Cost)),
                    KeywordParameter::QualityBefore => {
                        parts.insert(0, ParameterPart::Slot(SlotKind::Quality));
                    }
                    KeywordParameter::Quality(prep) | KeywordParameter::Name(prep) => {
                        if let Some(prep) = prep {
                            parts.push(ParameterPart::Word(format!("prep:{prep}").as_str().into()));
                        }
                        parts.push(ParameterPart::Slot(
                            if matches!(parameter, KeywordParameter::Name(_)) {
                                SlotKind::Name
                            } else {
                                SlotKind::Quality
                            },
                        ));
                    }
                    KeywordParameter::DashCost
                    | KeywordParameter::DashCondition
                    | KeywordParameter::DashAbility
                    | KeywordParameter::DashPowerToughness
                    | KeywordParameter::DashKeyword => {
                        parts.push(ParameterPart::Mark(
                            '—',
                            if matches!(
                                parameter,
                                KeywordParameter::DashCost | KeywordParameter::DashKeyword
                            ) {
                                Binding::Bound
                            } else {
                                Binding::Free
                            },
                        ));
                        parts.push(ParameterPart::Slot(match parameter {
                            KeywordParameter::DashCost => SlotKind::Cost,
                            KeywordParameter::DashCondition => SlotKind::Condition,
                            KeywordParameter::DashAbility => SlotKind::Ability,
                            KeywordParameter::DashPowerToughness => SlotKind::Measure,
                            _ => SlotKind::KeywordPhrase,
                        }));
                    }
                }
            }
            // One final Cost has two spellings, selected by the cost's
            // Category (style guide §3, "Periods" and "Em dashes"). Lower
            // both to Productions so punctuation is derived, never retained
            // in a Reading, and FinalParameter follows the selected spelling.
            // A non-final Cost, as in prototype, remains a mana cost.
            if matches!(parameters.last(), Some(KeywordParameter::Cost)) {
                let mut verbal = parts.clone();
                // Replace the final Cost slot, not its preceding parameters.
                let cost = verbal.pop();
                debug_assert!(matches!(cost, Some(ParameterPart::Slot(SlotKind::Cost))));
                verbal.extend([
                    ParameterPart::Mark('—', Binding::Bound),
                    ParameterPart::Slot(SlotKind::NonManaCost),
                    ParameterPart::Mark('.', Binding::Suffix),
                    ParameterPart::Feature("FinalParameter".into(), "SetOffCost".into()),
                ]);
                parts.push(ParameterPart::Feature(
                    "FinalParameter".into(),
                    "Cost".into(),
                ));
                return vec![parts, verbal];
            }
            parts.extend(final_set_off(parameters.last()));
            vec![parts]
        };
        let patterns = |parameters: &[Vec<KeywordParameter>]| -> Vec<Vec<ParameterPart>> {
            if parameters.is_empty() {
                vec![vec![ParameterPart::Head]]
            } else {
                parameters.iter().flat_map(|p| convert(p)).collect()
            }
        };
        // A pattern with any part besides its head carries a Keyword
        // Parameter; a bound stem is its Keyword Quality.
        let with_form = |mut parts: Vec<ParameterPart>| {
            let parametric = parts.iter().any(|part| match part {
                ParameterPart::Head | ParameterPart::Feature(..) => false,
                ParameterPart::Slot(_)
                | ParameterPart::Word(_)
                | ParameterPart::Words(_)
                | ParameterPart::ModifiedWords { .. }
                | ParameterPart::Mark(..) => true,
            });
            let form = if parametric { "Parametric" } else { "Bare" };
            parts.push(ParameterPart::Feature("KeywordForm".into(), form.into()));
            parts
        };
        for ability in &self.keyword_abilities {
            for (lemma, parameters) in std::iter::once((&ability.lemma, &ability.parameters))
                .chain(ability.variants.iter().map(|v| (&v.lemma, &v.parameters)))
            {
                frames.parameters.insert(
                    format!("keyword:{lemma}").as_str().into(),
                    patterns(parameters).into_iter().map(with_form).collect(),
                );
            }
            if let Some(bound) = &ability.bound {
                let patterns = patterns(&ability.parameters);
                let free_lemmas: std::collections::BTreeSet<_> =
                    std::iter::once(ability.lemma.to_ascii_lowercase())
                        .chain(
                            ability
                                .variants
                                .iter()
                                .map(|v| v.lemma.to_ascii_lowercase()),
                        )
                        .collect();
                let mut bare = std::collections::BTreeSet::new();
                let mut modified_only = std::collections::BTreeSet::new();
                for stem in bound.stems.iter().flat_map(|class| self.stems(*class)) {
                    let id = sylex_lexicon::LexemeId::new(&format!("stem:{stem}"));
                    let combined = format!("{stem}{}", bound.surface).to_ascii_lowercase();
                    if free_lemmas.contains(&combined) {
                        modified_only.insert(id);
                    } else {
                        bare.insert(id);
                    }
                }
                let bound_patterns = patterns
                    .into_iter()
                    .map(|mut parts| {
                        parts.insert(
                            0,
                            ParameterPart::ModifiedWords {
                                bare: bare.iter().copied().collect(),
                                modified_only: modified_only.iter().copied().collect(),
                                modifier: SlotKind::TypeModifier,
                                target: SlotKind::Quality,
                            },
                        );
                        with_form(parts)
                    })
                    .collect();
                frames.parameters.insert(
                    format!("keyword:{}:bound", ability.lemma).as_str().into(),
                    bound_patterns,
                );
            }
        }
    }

    /// The lowercase noun stems of a [`StemClass`].
    #[must_use]
    pub fn stems(&self, class: StemClass) -> Vec<String> {
        let families: &[SubtypeFamily] = match class {
            StemClass::LandType => &[SubtypeFamily::Land],
            StemClass::CreatureType => &[SubtypeFamily::Creature],
            StemClass::Subtype => &[
                SubtypeFamily::Artifact,
                SubtypeFamily::Battle,
                SubtypeFamily::Creature,
                SubtypeFamily::Dungeon,
                SubtypeFamily::Enchantment,
                SubtypeFamily::Land,
                SubtypeFamily::Planeswalker,
                SubtypeFamily::Spell,
            ],
            StemClass::CardType => {
                return self
                    .types
                    .card_types
                    .iter()
                    .map(|decl| decl.entry().lemma.to_lowercase())
                    .collect();
            }
            StemClass::Supertype => {
                return self
                    .types
                    .supertypes
                    .iter()
                    .map(|s| s.to_lowercase())
                    .collect();
            }
        };
        families
            .iter()
            .flat_map(|family| self.subtypes.get(family).into_iter().flatten())
            .map(|decl| decl.entry().lemma.to_lowercase())
            .collect()
    }
}

/// Tracks the shared Catalog verbs declared in `catalogs/verbs.ron` and which
/// keyword entries name them.
struct SharedVerbs<'a> {
    declared: std::collections::BTreeMap<String, &'a str>,
    used: std::collections::BTreeSet<String>,
}

impl<'a> SharedVerbs<'a> {
    fn build(verbs: &'a [VerbEntry], builder: &mut Builder) -> Result<Self, SourceError> {
        let mut declared = std::collections::BTreeMap::new();
        for verb in verbs {
            let key = verb.lemma.to_lowercase();
            if let Some(&existing) = declared.get(&key) {
                return Err(SourceError::Invalid(format!(
                    "shared verb `{}` in {CATALOG_VERBS} duplicates `{existing}`",
                    verb.lemma
                )));
            }
            let id = format!("catalog_verb:{}", verb.lemma);
            builder.verb(&id, verb, source(SourceKind::Catalog, CATALOG_VERBS, &id))?;
            declared.insert(key, verb.lemma.as_str());
        }
        Ok(Self {
            declared,
            used: std::collections::BTreeSet::new(),
        })
    }

    fn resolve(
        &mut self,
        builder: &mut Builder,
        decl: &VerbDecl,
        entry_label: &str,
        prefix: &str,
        src: sylex_lexicon::Source,
    ) -> Result<(), SourceError> {
        match decl {
            VerbDecl::Shared(lemma) => {
                let key = lemma.to_lowercase();
                let Some(&declared) = self.declared.get(&key) else {
                    return Err(SourceError::Invalid(format!(
                        "{entry_label} names shared verb `{lemma}`, which is not declared in \
                         {CATALOG_VERBS}"
                    )));
                };
                if declared != lemma {
                    return Err(SourceError::Invalid(format!(
                        "{entry_label} names shared verb `{lemma}`, which is spelled `{declared}` \
                         in {CATALOG_VERBS}"
                    )));
                }
                self.used.insert(key);
            }
            VerbDecl::Entry(verb) => {
                if let Some(&declared) = self.declared.get(&verb.lemma.to_lowercase()) {
                    return Err(SourceError::Invalid(format!(
                        "{entry_label} declares verb `{}`, which is already a shared verb \
                         `{declared}` in {CATALOG_VERBS}",
                        verb.lemma
                    )));
                }
                let id = format!("{prefix}:{}", verb.lemma);
                builder.verb(&id, verb, src)?;
            }
        }
        Ok(())
    }

    fn check_all_used(&self) -> Result<(), SourceError> {
        for (key, lemma) in &self.declared {
            if !self.used.contains(key) {
                return Err(SourceError::Invalid(format!(
                    "shared verb `{lemma}` in {CATALOG_VERBS} is named by no keyword entry"
                )));
            }
        }
        Ok(())
    }
}

/// The parts that close a keyword's parameter pattern after a final em-dash
/// parameter that is no Cost. A final em-dash Condition is a sentence, ending
/// in its own period as a verbal Cost does (*Companion — … .*); a final
/// em-dash Ability runs to the end of the line. A final em-dash mana cost
/// ends in no period (*suspend 4—{1}{U}*).
fn final_set_off(
    last: Option<&schema::KeywordParameter>,
) -> Vec<sylex_syntax::frames::ParameterPart> {
    use schema::KeywordParameter;
    use sylex_syntax::frames::ParameterPart;
    let feature = |value: &str| ParameterPart::Feature("FinalParameter".into(), value.into());
    match last {
        Some(KeywordParameter::DashCondition) => {
            vec![
                ParameterPart::Mark('.', Binding::Suffix),
                feature("SetOffCost"),
            ]
        }
        Some(KeywordParameter::DashAbility) => vec![feature("SetOffAbility")],
        Some(
            KeywordParameter::Amount
            | KeywordParameter::Cost
            | KeywordParameter::DashCost
            | KeywordParameter::Quality(_)
            | KeywordParameter::QualityBefore
            | KeywordParameter::Name(_)
            | KeywordParameter::DashPowerToughness
            | KeywordParameter::DashKeyword,
        )
        | None => Vec::new(),
    }
}

/// Marks each verb whose past participle is spelled like a declared
/// Adjective (*tapped*, *transformed*), and that Adjective, which is the
/// verb's Adjectival Passive: the grammar then does not derive the Adjective
/// from the participle again, and the participle, not the Adjective,
/// postmodifies with a Complement (*Aura attached to a creature*).
fn mark_lexical_adjectival_passives(lexemes: &mut [Lexeme]) -> Result<(), SourceError> {
    let lexicon = Lexicon::new(lexemes.to_vec())?;
    let mut marked = std::collections::BTreeSet::new();
    // An attributive-only homograph (*the left half*) is no predicative
    // Adjectival Passive.
    let predicative = |l: &&Lexeme| {
        l.properties
            .features
            .get("AdjectivePosition")
            .map(Ident::as_str)
            != Some("Attributive")
    };
    for adjective in lexemes
        .iter()
        .filter(|l| l.category == Category::Adjective)
        .filter(predicative)
    {
        let analyzed = lexicon.analyze(&adjective.lemma);
        for found in &analyzed.matches {
            if let sylex_lexicon::LexicalReading::Word(word) = &found.reading
                && found.start == 0
                && found.end == analyzed.tokens.len()
                && word.form == sylex_lexicon::WordForm::PastParticiple
                && lexicon.lexemes()[&word.lexeme].category == Category::Verb
            {
                marked.insert(word.lexeme);
                marked.insert(adjective.id);
            }
        }
    }
    for lexeme in lexemes.iter_mut().filter(|l| marked.contains(&l.id)) {
        lexeme
            .properties
            .features
            .insert("AdjectivalPassive".into(), "Lexical".into());
    }
    Ok(())
}

/// A dungeon's name [CR#309.1]: a Proper Name in scope on every card, as the
/// dungeon is a game object any card's text can name (*if you haven't
/// completed Tomb of Annihilation*, Acererak the Archlich; *venture into
/// Undercity* [CR#727.2]).
fn dungeon_name(name: &str) -> Lexeme {
    exact(
        &format!("dungeon_name:{name}"),
        name,
        Category::Catalog,
        source(SourceKind::Catalog, DUNGEON_NAMES, name),
    )
}

/// No word is both core vocabulary and a catalog item of the same Category.
fn check_core_and_catalogs_disjoint(lexemes: &[Lexeme]) -> Result<(), SourceError> {
    let mut core = std::collections::BTreeMap::new();
    for lexeme in lexemes.iter().filter(|l| l.source.kind == SourceKind::Core) {
        core.insert((lexeme.category, lexeme.lemma.to_lowercase()), lexeme.id);
    }
    for lexeme in lexemes
        .iter()
        .filter(|l| l.source.kind == SourceKind::Catalog)
    {
        if let Some(id) = core.get(&(lexeme.category, lexeme.lemma.to_lowercase())) {
            return Err(SourceError::Invalid(format!(
                "`{}` is a catalog item and must not also be core vocabulary `{id}`",
                lexeme.id
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A dungeon name is a Catalog item, so the loader refuses it when a Core
    /// word of its Category spells it, as for every other Catalog item.
    #[test]
    fn a_dungeon_name_is_never_core_vocabulary() {
        let (mut lexemes, _) = Sources::load().unwrap().vocabulary().unwrap();
        let dungeon = lexemes
            .iter()
            .find(|lexeme| lexeme.id.as_str() == "dungeon_name:Undercity")
            .expect("Undercity is a generated dungeon name");
        assert!(!dungeon.card_name);
        lexemes.push(Lexeme::invariant(
            "core:undercity",
            "undercity",
            Category::Catalog,
            source(SourceKind::Core, NOUNS, "core:undercity"),
        ));
        let error = check_core_and_catalogs_disjoint(&lexemes)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("`dungeon_name:Undercity` is a catalog item"),
            "{error}"
        );
    }
}

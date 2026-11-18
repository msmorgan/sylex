//! Builds Lexemes and Subcategorization Frames from authored entries.

use std::collections::BTreeSet;

use dpsi::Ident;
use sylex_lexicon::Binding;
use sylex_lexicon::Capitalization;
use sylex_lexicon::Category;
use sylex_lexicon::FeatureBundle;
use sylex_lexicon::Finiteness;
use sylex_lexicon::FormDeclaration;
use sylex_lexicon::Lexeme;
use sylex_lexicon::LexemeId;
use sylex_lexicon::Number;
use sylex_lexicon::Person;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_lexicon::SurfaceStructure;
use sylex_lexicon::WordForm;
use sylex_syntax::FrameCatalog;
use sylex_syntax::frames::ClauseKind;
use sylex_syntax::frames::Frame;
use sylex_syntax::frames::FrameFamily;
use sylex_syntax::frames::FrameSlot;
use sylex_syntax::frames::FrameVoice;
use sylex_syntax::frames::ParameterPart;
use sylex_syntax::frames::PrepositionFrame;
use sylex_syntax::frames::SlotFill;
use sylex_syntax::frames::SlotKind;
use sylex_syntax::frames::SlotRelation;
use sylex_syntax::frames::SlotRelation::Complement;
use sylex_syntax::frames::SlotRelation::IndirectObject;
use sylex_syntax::frames::SlotRelation::Object;
use sylex_syntax::frames::SlotRelation::PredicativeComplement;

use crate::SourceError;
use crate::schema::AdjectiveEntry;
use crate::schema::AdjectiveGrade;
use crate::schema::AdverbEntry;
use crate::schema::Catenation;
use crate::schema::DeterminativeEntry;
use crate::schema::FillSpec;
use crate::schema::FrameSpec;
use crate::schema::FusedHeadDetermination;
use crate::schema::IdiomPart;
use crate::schema::NounEntry;
use crate::schema::PrepositionEntry;
use crate::schema::PrepositionFrameSpec;
use crate::schema::VerbEntry;

/// Lexemes and frames gathered from every data file, with frames still naming
/// prepositions by lemma until every preposition is known.
#[derive(Default)]
pub(crate) struct Builder {
    parameters: std::collections::BTreeMap<LexemeId, Vec<Vec<sylex_syntax::frames::ParameterPart>>>,
    pub(crate) lexemes: Vec<Lexeme>,
    frames: Vec<(LexemeId, Category, Vec<FrameSpec>, String)>,
    /// Nouns whose genitive names something other than their *of*
    /// Complement (`NounEntry::distinct_genitive`).
    distinct_genitive: std::collections::BTreeSet<LexemeId>,
    prepositions: Vec<(LexemeId, Vec<PrepositionFrameSpec>, String)>,
}

pub(crate) fn source(kind: SourceKind, path: &str, owner: &str) -> Source {
    Source {
        kind,
        path: path.to_owned(),
        owner: owner.to_owned(),
    }
}

/// Words separated by spaces are one multiword Lexeme.
pub(crate) fn structured(mut lexeme: Lexeme) -> Lexeme {
    if lexeme.surface_structure == SurfaceStructure::Word && lexeme.lemma.contains(' ') {
        lexeme.surface_structure = SurfaceStructure::Multiword;
    }
    lexeme
}

fn replace(
    lexeme: &mut Lexeme,
    form: WordForm,
    person: Option<Person>,
    number: Option<Number>,
    surface: &str,
) -> bool {
    let mut found = false;
    for slot in &mut lexeme.forms {
        if slot.form == form
            && person.is_none_or(|p| slot.features.person == Some(p))
            && number.is_none_or(|n| slot.features.number == Some(n))
        {
            slot.surface = Some(surface.to_owned());
            found = true;
        }
    }
    found
}

impl Builder {
    pub(crate) fn push(&mut self, lexeme: Lexeme) {
        self.lexemes.push(structured(lexeme));
    }

    pub(crate) fn frames(&mut self, id: LexemeId, category: Category, frames: &[FrameSpec]) {
        if !frames.is_empty() {
            let owner = id.to_string();
            self.frames.push((id, category, frames.to_vec(), owner));
        }
    }

    pub(crate) fn adverb(&mut self, entry: &AdverbEntry, source: Source) {
        let id = format!("adv:{}", entry.lemma);
        let mut lexeme = Lexeme::invariant(&id, &entry.lemma, Category::Adverb, source);
        lexeme.properties.classes = entry
            .positions
            .iter()
            .map(|position| Ident::new(&format!("{position:?}Adverb")))
            .collect();
        self.push(lexeme);
    }

    pub(crate) fn determinative(&mut self, id: &str, entry: &DeterminativeEntry, source: Source) {
        let mut lexeme = Lexeme::invariant(id, &entry.lemma, Category::Determinative, source);
        lexeme.properties.features.insert(
            "DeterminativeDistribution".into(),
            Ident::new(&format!("{:?}", entry.distribution)),
        );
        lexeme.forms[0].features.number = entry.number;
        if let Some(allomorphs) = &entry.allomorphs {
            let slot = lexeme.forms.remove(0);
            lexeme.forms = allomorphs
                .iter()
                .map(|(onset, surface)| FormDeclaration {
                    features: FeatureBundle {
                        following_onset: Some(*onset),
                        ..slot.features.clone()
                    },
                    surface: Some(surface.clone()),
                    ..slot.clone()
                })
                .collect();
        }
        if let Some(countability) = entry.countability {
            lexeme.properties.features.insert(
                "Countability".into(),
                Ident::new(&format!("{countability:?}")),
            );
        }
        if let Some(kind) = entry.noun_kind {
            lexeme
                .properties
                .features
                .insert("NounKind".into(), Ident::new(&format!("{kind:?}")));
        }
        self.push(lexeme);
        self.frames(id.into(), Category::Determinative, &entry.frames);
    }

    pub(crate) fn verb(
        &mut self,
        id: &str,
        entry: &VerbEntry,
        source: Source,
    ) -> Result<(), SourceError> {
        let invalid = |reason: &str| SourceError::Invalid(format!("verb `{id}`: {reason}"));
        let auxiliary = source.kind == SourceKind::Core && source.path == crate::CLOSED_CLASS;
        if entry.frames.is_empty() && !auxiliary {
            return Err(invalid(
                "a verb declares at least one Subcategorization Frame",
            ));
        }
        if entry.binding.is_some() && !auxiliary {
            return Err(invalid("only a contracted auxiliary is bound"));
        }
        let mut lexeme = Lexeme::verb(id, &entry.lemma, source);
        lexeme
            .properties
            .classes
            .extend(entry.classes.iter().map(|class| Ident::new(class)));
        // Oracle text has no first person: a paradigm has
        // second- and third-person cells only.
        lexeme
            .forms
            .retain(|slot| slot.features.person != Some(Person::First));
        if let Some(binding) = entry.binding {
            lexeme.binding = binding;
            lexeme.capitalization = sylex_lexicon::Capitalization::Exact;
        }
        if entry.finite_only {
            lexeme.forms.retain(|slot| {
                slot.form == WordForm::Present
                    || (slot.form == WordForm::Preterite
                        && (entry.preterite.is_some() || entry.past.is_some()))
            });
        }
        if entry.finite_only && (entry.past_participle.is_some() || entry.gerund.is_some()) {
            return Err(invalid("a finite-only verb has no participles"));
        }
        let past: &[WordForm] = if entry.finite_only {
            &[WordForm::Preterite]
        } else {
            &[WordForm::Preterite, WordForm::PastParticiple]
        };
        let third = (Some(Person::Third), Some(Number::Singular));
        let fields = [
            (&[WordForm::Present][..], &entry.third_singular, third),
            (past, &entry.past, (None, None)),
            (&[WordForm::Preterite][..], &entry.preterite, (None, None)),
            (
                &[WordForm::PastParticiple][..],
                &entry.past_participle,
                (None, None),
            ),
            (
                &[WordForm::GerundParticiple][..],
                &entry.gerund,
                (None, None),
            ),
        ];
        for (forms, spellings, (person, number)) in fields {
            let Some(spellings) = spellings else { continue };
            for form in forms {
                if !replace(&mut lexeme, *form, person, number, spellings) {
                    return Err(invalid(&format!("no {form:?} slot to spell")));
                }
            }
        }
        if let Some(irrealis) = &entry.irrealis {
            lexeme.forms.push(FormDeclaration {
                form: WordForm::Irrealis,
                features: FeatureBundle {
                    number: Some(Number::Singular),
                    person: Some(Person::Third),
                    finiteness: Some(Finiteness::Finite),
                    ..FeatureBundle::default()
                },
                surface: Some(irrealis.clone()),
            });
        }
        for slot in &entry.forms {
            if !replace(
                &mut lexeme,
                slot.form,
                slot.person,
                slot.number,
                &slot.surface,
            ) {
                return Err(invalid(&format!("no {:?} slot to spell", slot.form)));
            }
        }
        if entry.modal {
            if !auxiliary {
                return Err(invalid("only an auxiliary is a modal"));
            }
            lexeme.properties.modal = true;
            for slot in &mut lexeme.forms {
                if slot.form == WordForm::Preterite {
                    slot.features.tense = None;
                }
            }
        }
        lexeme.merge_number_neutral_forms();
        lexeme.properties.features.insert(
            "VerbKind".into(),
            if entry.modal {
                "Modal"
            } else if auxiliary {
                "Auxiliary"
            } else {
                "Lexical"
            }
            .into(),
        );
        if let Some(base) = &entry.form_of {
            if !auxiliary {
                return Err(invalid("only an auxiliary is a form of another verb"));
            }
            // Spell every form from this entry's own lemma before naming it.
            let spelled = lexeme.clone();
            for slot in &mut lexeme.forms {
                if slot.surface.is_none() {
                    slot.surface = Some(sylex_lexicon::default_surface(
                        &spelled,
                        slot.form,
                        &slot.features,
                    ));
                }
            }
            lexeme.lemma.clone_from(base);
        }
        if entry.negative && !auxiliary {
            return Err(invalid("only an auxiliary has a negative form"));
        }
        if auxiliary {
            lexeme.properties.features.insert(
                "Polarity".into(),
                if entry.negative { "Negative" } else { "Affirmative" }.into(),
            );
        }
        let id = lexeme.id;
        self.push(lexeme);
        self.frames(id, Category::Verb, &entry.frames);
        Ok(())
    }

    pub(crate) fn noun(
        &mut self,
        id: &str,
        entry: &NounEntry,
        source: Source,
    ) -> Result<(), SourceError> {
        if entry.bare_mass_modified
            && !entry
                .countability
                .contains(&sylex_lexicon::Countability::Mass)
        {
            return Err(SourceError::Invalid(format!(
                "noun `{id}`: `bare_mass_modified` requires `Mass` countability"
            )));
        }
        let mut lexeme = Lexeme::noun(id, &entry.lemma, entry.countability.clone(), source);
        lexeme
            .properties
            .classes
            .extend(entry.classes.iter().map(|class| Ident::new(class)));
        lexeme.properties.features.insert(
            "NounDistribution".into(),
            Ident::new(&format!("{:?}", entry.distribution)),
        );
        lexeme
            .properties
            .features
            .insert("NounKind".into(), Ident::new(&format!("{:?}", entry.kind)));
        // An undeclared gender is a thing's: most nouns name one.
        let gender = match entry.relative_gender {
            None | Some(crate::schema::RelativeGender::NonPersonal) => Some("NonPersonal"),
            Some(crate::schema::RelativeGender::Personal) => Some("Personal"),
            Some(crate::schema::RelativeGender::Dual) => None,
        };
        if let Some(gender) = gender {
            lexeme
                .properties
                .features
                .insert("RelativeGender".into(), Ident::new(gender));
        }
        if entry.bare_mass_modified {
            lexeme
                .properties
                .features
                .insert("BareMass".into(), Ident::new("Modified"));
        }
        if let Some(plural) = &entry.plural {
            replace(&mut lexeme, WordForm::Plural, None, None, plural);
        }
        lexeme.properties.prefix_hyphenation = entry.prefix_hyphenation;
        lexeme.onset = entry.onset;
        // A noun whose pattern takes a Set Name before it is where analysis
        // reads one, and takes no other premodifier: its license
        // (`SetNameLicense`, english.md §3).
        lexeme.set_name_host = licensed_host(id, entry, SlotKind::SetName, "a Set Name")?;
        if lexeme.set_name_host {
            lexeme
                .properties
                .features
                .insert("SetNameLicense".into(), Ident::new("Head"));
        }
        // A noun whose pattern takes a cited Vote Option before it (*each
        // numbers vote*) takes no noun premodifier there: its license
        // (`OptionLicense`, english.md §3).
        if licensed_host(id, entry, SlotKind::OptionMention, "a cited option")? {
            lexeme
                .properties
                .features
                .insert("OptionLicense".into(), Ident::new("Head"));
        }
        if entry.distinct_genitive && !entry.frames.iter().any(selects_of) {
            return Err(SourceError::Invalid(format!(
                "noun `{id}`: only a noun with an *of* Complement frame has a distinct genitive"
            )));
        }
        let id = lexeme.id;
        self.push(lexeme);
        self.frames(id, Category::Noun, &entry.frames);
        if entry.distinct_genitive {
            self.distinct_genitive.insert(id);
        }
        if !entry.parameters.is_empty() {
            self.parameters.insert(id, entry.parameters.clone());
        }
        Ok(())
    }

    pub(crate) fn adjective(&mut self, prefix: &str, entry: &AdjectiveEntry, source: &Source) {
        let own = match entry.grade {
            None => "Plain",
            Some(AdjectiveGrade::Comparative) => "Comparative",
            Some(AdjectiveGrade::Superlative) => "Superlative",
            Some(AdjectiveGrade::Equative) => "Equative",
            Some(AdjectiveGrade::NonScalarInequality) => "NonScalarInequality",
        };
        let grades = [
            (Some(&entry.lemma), Some(own)),
            (entry.comparative.as_ref(), Some("Comparative")),
            (entry.superlative.as_ref(), Some("Superlative")),
        ];
        for (spelling, grade) in grades {
            let Some(spelling) = spelling else { continue };
            // A grade is an inflection of its adjective: it shares the lemma.
            let mut lexeme = Lexeme::invariant(
                format!("{prefix}:{spelling}"),
                &entry.lemma,
                Category::Adjective,
                source.clone(),
            );
            if spelling != &entry.lemma {
                replace(&mut lexeme, WordForm::Invariant, None, None, spelling);
            }
            if let Some(position) = entry.position {
                lexeme.properties.features.insert(
                    "AdjectivePosition".into(),
                    Ident::new(&format!("{position:?}")),
                );
            }
            if entry.needs_determiner {
                lexeme
                    .properties
                    .features
                    .insert("DeterminerNeed".into(), Ident::new("Required"));
            }
            if entry.follows_sum {
                lexeme.properties.classes.insert("SumAdjective".into());
            }
            if entry.fused_quality {
                lexeme
                    .properties
                    .features
                    .insert("QualityHead".into(), Ident::new("Fused"));
            }
            if let Some(determination) = entry.fused_head {
                let class = match determination {
                    FusedHeadDetermination::Ordinary => "FusedHeadAdjective",
                    FusedHeadDetermination::DefiniteArticle => "DefiniteFusedHeadAdjective",
                };
                lexeme.properties.classes.insert(class.into());
            }
            if let Some(grade) = grade {
                lexeme
                    .properties
                    .features
                    .insert("Grade".into(), grade.into());
            }
            let id = lexeme.id;
            self.push(lexeme);
            match grade {
                Some(g) if g == own => self.frames(id, Category::Adjective, &entry.frames),
                // A comparative selects its *than* Complement (CGEL, Ch. 13
                // §2.1, p. 1104).
                Some("Comparative") => self.frames(
                    id,
                    Category::Adjective,
                    &[FrameSpec::Complement(FillSpec::PP(vec!["than".into()]))],
                ),
                _ => {}
            }
        }
    }

    pub(crate) fn preposition(&mut self, entry: &PrepositionEntry, source: Source) {
        let lemma = entry.lemma.as_str();
        let lexeme = Lexeme::invariant(
            format!("prep:{lemma}"),
            lemma,
            Category::Preposition,
            source,
        );
        let id = lexeme.id;
        self.push(lexeme);
        self.prepositions
            .push((id, entry.frames.clone(), id.to_string()));
    }

    /// Validates and indexes everything gathered, resolving preposition lemmas.
    pub(crate) fn finish(self) -> Result<(Vec<Lexeme>, FrameCatalog), SourceError> {
        let mut ids = BTreeSet::new();
        for lexeme in &self.lexemes {
            if !ids.insert(lexeme.id) {
                return Err(SourceError::Invalid(format!(
                    "`{}` is declared twice",
                    lexeme.id
                )));
            }
        }
        let resolver = Resolver { ids: &ids };
        let mut catalog = FrameCatalog {
            parameters: self.parameters,
            ..FrameCatalog::default()
        };
        for (id, category, specs, owner) in &self.frames {
            let mut frames = Vec::new();
            for spec in specs {
                let mut frame = resolver.frame(spec, *category, owner)?;
                frame.genitive_alternant = *category == Category::Noun
                    && !self.distinct_genitive.contains(id)
                    && selects_of(spec);
                if frames.contains(&frame) {
                    return Err(SourceError::Invalid(format!(
                        "`{owner}` declares {spec:?} twice"
                    )));
                }
                frames.push(frame);
            }
            catalog.frames.insert(*id, frames);
        }
        for (id, specs, owner) in &self.prepositions {
            let mut frames = Vec::new();
            for spec in specs {
                let frame = PrepositionFrame {
                    relation: spec.relation,
                    complement: resolver.fill(&spec.complement, owner)?,
                    hosts: spec.hosts,
                    constraints: spec.constraints.clone(),
                };
                if frames
                    .iter()
                    .any(|f: &PrepositionFrame| f.complement == frame.complement)
                {
                    return Err(SourceError::Invalid(format!(
                        "`{owner}` declares a {:?} complement twice",
                        spec.complement
                    )));
                }
                frames.push(frame);
            }
            catalog.prepositions.insert(*id, frames);
        }
        Ok((self.lexemes, catalog))
    }
}

/// Resolves authored frames against the declared Lexemes.
struct Resolver<'a> {
    ids: &'a BTreeSet<LexemeId>,
}

fn slot(relation: SlotRelation, fill: SlotFill) -> FrameSlot {
    FrameSlot { relation, fill }
}

fn catenation(catenation: Catenation) -> SlotFill {
    match catenation {
        Catenation::ToInf => SlotFill::ToInf,
        Catenation::BareInf => SlotFill::BareInf,
        Catenation::Gerund => SlotFill::Gerund,
        Catenation::PastPart => SlotFill::PastPart,
    }
}

impl Resolver<'_> {
    fn preposition(&self, lemma: &str, owner: &str) -> Result<LexemeId, SourceError> {
        let id = LexemeId::new(&format!("prep:{lemma}"));
        if self.ids.contains(&id) {
            Ok(id)
        } else {
            Err(SourceError::Invalid(format!(
                "`{owner}` names `{lemma}`, which is not a declared preposition"
            )))
        }
    }

    fn pp(&self, lemmas: &[String], owner: &str) -> Result<SlotFill, SourceError> {
        Ok(SlotFill::PP(self.prepositions(lemmas, owner)?, None))
    }

    fn prepositions(&self, lemmas: &[String], owner: &str) -> Result<Vec<LexemeId>, SourceError> {
        lemmas
            .iter()
            .map(|lemma| self.preposition(lemma, owner))
            .collect()
    }

    fn fill(&self, spec: &FillSpec, owner: &str) -> Result<SlotFill, SourceError> {
        Ok(match spec {
            FillSpec::NP => SlotFill::NP,
            FillSpec::PP(lemmas) => self.pp(lemmas, owner)?,
            FillSpec::PPWith(lemmas, complement) => SlotFill::PP(
                self.prepositions(lemmas, owner)?,
                Some(self.fill(complement, owner)?.kind()),
            ),
            FillSpec::Pred => SlotFill::Pred,
            FillSpec::Name => SlotFill::Name,
            FillSpec::AdjP => SlotFill::AdjP,
            FillSpec::AdvP => SlotFill::AdvP,
            FillSpec::LocativeNP => SlotFill::LocativeNP,
            FillSpec::Position => SlotFill::Position,
            FillSpec::MeasureNP => SlotFill::MeasureNP,
            FillSpec::FiniteClause => SlotFill::Clause(ClauseKind::Finite),
            FillSpec::TemporalClause => SlotFill::Clause(ClauseKind::Temporal),
            FillSpec::ComparativeClause => SlotFill::Clause(ClauseKind::Comparative),
            FillSpec::ExpandableClause => SlotFill::ExpandableClause,
            FillSpec::SetOffClause => SlotFill::SetOffClause,
            FillSpec::VerblessClause => SlotFill::Clause(ClauseKind::Verbless),
            FillSpec::AbsoluteClause => SlotFill::Clause(ClauseKind::Absolute),
            FillSpec::ToInf => SlotFill::ToInf,
            FillSpec::HollowToInf => SlotFill::HollowToInf,
            FillSpec::BareInf => SlotFill::BareInf,
            FillSpec::Gerund => SlotFill::Gerund,
            FillSpec::PastPart => SlotFill::PastPart,
            FillSpec::KeywordPhrase => SlotFill::KeywordPhrase,
            FillSpec::QuotedAbility => SlotFill::QuotedAbility,
            FillSpec::Mention => SlotFill::Mention,
            FillSpec::OptionChoice => SlotFill::OptionChoice,
            FillSpec::Nominal => SlotFill::Nominal,
            FillSpec::Amount => SlotFill::Amount,
            FillSpec::Cost => SlotFill::Cost,
            FillSpec::Intransitive => SlotFill::Intransitive,
        })
    }

    /// The engine Frame an authored frame of a `category` head declares,
    /// with the voices it restricts a verb's frame to.
    fn frame(
        &self,
        spec: &FrameSpec,
        category: Category,
        owner: &str,
    ) -> Result<Frame, SourceError> {
        let (shape, voice) = match spec {
            FrameSpec::ActiveOnly(shape) => (&**shape, FrameVoice::ActiveOnly),
            FrameSpec::PassiveOnly(shape) => (&**shape, FrameVoice::PassiveOnly),
            shape => (shape, FrameVoice::ActiveOrPassive),
        };
        if voice != FrameVoice::ActiveOrPassive && category != Category::Verb {
            return Err(SourceError::Invalid(format!(
                "`{owner}`: only a verb's frame restricts its voice, not {spec:?}"
            )));
        }
        let mut frame = self.shaped_frame(shape, category, owner)?;
        frame.voice = voice;
        if !frame.is_well_formed() {
            return Err(SourceError::Invalid(format!(
                "`{owner}`: {spec:?} is Passive-only but has no passive"
            )));
        }
        Ok(frame)
    }

    /// The engine Frame of an authored frame's shape, in both voices.
    fn shaped_frame(
        &self,
        spec: &FrameSpec,
        category: Category,
        owner: &str,
    ) -> Result<Frame, SourceError> {
        let (family, slots) = match spec {
            FrameSpec::ActiveOnly(_) | FrameSpec::PassiveOnly(_) => {
                return Err(SourceError::Invalid(format!(
                    "`{owner}`: {spec:?} restricts a frame's voice inside a restricted frame"
                )));
            }
            FrameSpec::Intransitive => (FrameFamily::Intransitive, vec![]),
            FrameSpec::Monotransitive => (
                FrameFamily::Monotransitive,
                vec![slot(Object, SlotFill::NP)],
            ),
            FrameSpec::Transitive(fill) => (
                FrameFamily::Monotransitive,
                vec![slot(Object, self.fill(fill, owner)?)],
            ),
            FrameSpec::Ditransitive => (
                FrameFamily::Ditransitive,
                vec![
                    slot(IndirectObject, SlotFill::NP),
                    slot(Object, SlotFill::NP),
                ],
            ),
            FrameSpec::TransitiveWithPP(lemmas) => (
                FrameFamily::TransitiveWithPP,
                vec![
                    slot(Object, SlotFill::NP),
                    slot(Complement, self.pp(lemmas, owner)?),
                ],
            ),
            FrameSpec::Prepositional(lemmas) => (
                FrameFamily::Prepositional,
                vec![slot(Complement, self.pp(lemmas, owner)?)],
            ),
            FrameSpec::PrepositionalComplement(fill) => {
                let fill = self.fill(fill, owner)?;
                if !matches!(fill, SlotFill::PP(..)) {
                    return Err(SourceError::Invalid(format!(
                        "`{owner}`: {spec:?} names no Preposition Phrase"
                    )));
                }
                (FrameFamily::Prepositional, vec![slot(Complement, fill)])
            }
            FrameSpec::Copular => (
                FrameFamily::Copular,
                vec![slot(PredicativeComplement, SlotFill::Pred)],
            ),
            FrameSpec::Catenative(kind) => (
                FrameFamily::Catenative,
                vec![slot(Complement, catenation(*kind))],
            ),
            FrameSpec::ComplexTransitive(fill) => (
                FrameFamily::ComplexTransitive,
                vec![
                    slot(Object, SlotFill::NP),
                    slot(PredicativeComplement, self.fill(fill, owner)?),
                ],
            ),
            FrameSpec::ComplexCatenative(kind) => (
                FrameFamily::ComplexCatenative,
                vec![
                    slot(Object, SlotFill::NP),
                    slot(Complement, catenation(*kind)),
                ],
            ),
            FrameSpec::ContentClause => (
                FrameFamily::ContentClause,
                vec![slot(
                    Complement,
                    SlotFill::Clause(ClauseKind::Interrogative),
                )],
            ),
            FrameSpec::Amount => (
                FrameFamily::Amount,
                vec![slot(Complement, SlotFill::Amount)],
            ),
            FrameSpec::TransitiveWithComplement(fill) => {
                let fill = self.fill(fill, owner)?;
                // A Preposition Phrase Complement after the Object is the
                // TransitiveWithPP family's shape, whether or not the slot
                // names its Complement's kind.
                let family = if matches!(fill, SlotFill::PP(..)) {
                    FrameFamily::TransitiveWithPP
                } else {
                    FrameFamily::TransitiveWithComplement
                };
                (
                    family,
                    vec![slot(Object, SlotFill::NP), slot(Complement, fill)],
                )
            }
            FrameSpec::TransitiveAmount => (
                FrameFamily::TransitiveAmount,
                vec![
                    slot(Object, SlotFill::NP),
                    slot(Complement, SlotFill::Amount),
                ],
            ),
            FrameSpec::Cost => (FrameFamily::Cost, vec![slot(Complement, SlotFill::Cost)]),
            FrameSpec::Idiom(parts) => (
                FrameFamily::Idiom,
                parts
                    .iter()
                    .map(|part| {
                        Ok(match part {
                            IdiomPart::Prep(lemma) => {
                                slot(Complement, self.pp(std::slice::from_ref(lemma), owner)?)
                            }
                            IdiomPart::Pred => slot(PredicativeComplement, SlotFill::Pred),
                        })
                    })
                    .collect::<Result<_, SourceError>>()?,
            ),
            FrameSpec::Complement(fill) => {
                let family = match category {
                    Category::Noun => FrameFamily::NounComplement,
                    Category::Adjective => FrameFamily::AdjectiveComplement,
                    Category::Determinative => FrameFamily::DeterminativeComplement,
                    _ => {
                        return Err(SourceError::Invalid(format!(
                            "`{owner}`: only a noun, adjective or determinative declares a bare Complement frame"
                        )));
                    }
                };
                (family, vec![slot(Complement, self.fill(fill, owner)?)])
            }
        };
        // Any head may declare Intransitive: for a non-verb, that its
        // Complement is optional even where a phrase makes it obligatory.
        let verb_family = !matches!(spec, FrameSpec::Complement(_) | FrameSpec::Intransitive);
        if verb_family && category != Category::Verb
            || matches!(spec, FrameSpec::Complement(_)) && category == Category::Verb
        {
            return Err(SourceError::Invalid(format!(
                "`{owner}`: {spec:?} is not a frame of a {category:?}"
            )));
        }
        let frame = Frame {
            family,
            slots,
            voice: FrameVoice::ActiveOrPassive,
            genitive_alternant: false,
        };
        if !frame.is_well_formed() {
            return Err(SourceError::Invalid(format!(
                "`{owner}`: {spec:?} does not have the shape of its family"
            )));
        }
        Ok(frame)
    }
}

/// Whether a Complement frame selects an *of* phrase, the oblique that
/// alternates with a genitive subject-determiner: other roles than the
/// agent's take the default preposition *of* (CGEL, Ch. 5, §16.5.3,
/// p. 476). A frame that names *for* beside *of* names one argument in
/// either form (*the mana cost for spells you cast*), and alternates whole.
fn selects_of(spec: &FrameSpec) -> bool {
    matches!(
        spec,
        FrameSpec::Complement(FillSpec::PP(list) | FillSpec::PPWith(list, _))
            if list.iter().any(|preposition| preposition == "of")
    )
}

/// A Lexeme of a closed-class or catalog category, spelled exactly.
pub(crate) fn exact(id: &str, lemma: &str, category: Category, source: Source) -> Lexeme {
    let mut lexeme = Lexeme::invariant(id, lemma, category, source);
    lexeme.capitalization = Capitalization::Exact;
    lexeme
}

/// Whether a noun's parameter patterns take a parameter of `kind` before
/// the noun: a license feature's head (english.md §3).
///
/// # Errors
/// A pattern that takes one beside another parameter, or anywhere but right
/// before its head.
fn licensed_host(
    id: &str,
    entry: &NounEntry,
    kind: SlotKind,
    what: &str,
) -> Result<bool, SourceError> {
    let parameter = ParameterPart::Slot(kind);
    if entry.parameters.iter().any(|parts| {
        parts.contains(&parameter) && parts.as_slice() != [parameter.clone(), ParameterPart::Head]
    }) {
        return Err(SourceError::Invalid(format!(
            "noun `{id}`: {what} is a whole pattern's one parameter, right before its head"
        )));
    }
    Ok(entry
        .parameters
        .iter()
        .any(|parts| parts.contains(&parameter)))
}

/// A bound Lexeme spelled exactly.
pub(crate) fn bound(
    id: &str,
    lemma: &str,
    category: Category,
    binding: Binding,
    source: Source,
) -> Lexeme {
    let mut lexeme = exact(id, lemma, category, source);
    lexeme.binding = binding;
    lexeme
}

/// A bound Lexeme with its allomorphs, each the form its features select,
/// the first spelling its lemma.
pub(crate) fn allomorphic(
    id: &str,
    category: Category,
    allomorphs: impl IntoIterator<Item = (FeatureBundle, String)>,
    binding: Binding,
    source: Source,
) -> Lexeme {
    let forms: Vec<FormDeclaration> = allomorphs
        .into_iter()
        .map(|(features, surface)| FormDeclaration {
            form: WordForm::Invariant,
            features,
            surface: Some(surface),
        })
        .collect();
    let lemma = forms[0].surface.clone().expect("an allomorph is spelled");
    let mut lexeme = bound(id, &lemma, category, binding, source);
    lexeme.forms = forms;
    lexeme
}

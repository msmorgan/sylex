//! How the forest database reads an Embedding for a census of verb phrases:
//! which steps are wrappers, and which context a verb phrase stands in.

use sylex_lexicon::Lexicon;
use sylex_lexicon::WordForm;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Embedding;
use sylex_syntax::EmbeddingStep;

/// The steps an Embedding keeps that are no wrapper. The first one is the
/// context of a verb phrase; the next two say what governs it (the *if* above
/// a *would*).
pub(super) const DEPTH: usize = 3;

/// The context a verb phrase stands in: the first step of its Embedding that
/// is no wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Context {
    /// The `clause` of a Sentence Production: an ability's effect, or a
    /// static ability's statement.
    EffectClause,
    /// The `complement` of a Preposition Phrase headed by *when* or
    /// *whenever*, whose Complement is a clause.
    TriggerClause,
    /// The `complement` of a `Catenative(BareInf)` verb phrase headed by
    /// *will* in its Preterite form: *would*.
    WouldClause,
    /// The `complement` of a `Catenative(BareInf)` verb phrase headed by
    /// *can* (*can't*, *could*), *may* or *must*.
    Deontic,
    /// Under a Production of the `RelativeClause` Category, or the fused
    /// relatives `FusedRelative` and `FusedSubjectRelative` (*whichever*).
    RelativeClause,
    /// The head of a `Postmodifier` Production (`ParticipialPostmodifier`,
    /// `GerundParticipialPostmodifier`), the `filler` of one
    /// (`PastParticipialConjunct`, `GerundParticipialConjunct`, whose
    /// `PostmodifierConjunct` Category is a `Postmodifier`), or a verb phrase
    /// that is a `modifier` (`SingleParticipialPostmodifier`,
    /// `SingleGerundParticipialPostmodifier`, `FusedParticipialPostmodifier`,
    /// `PostposedParticipialModifier`).
    ParticipialPostmodifier,
    /// Anything else, as the clause of *if*, *unless* or *as long as*, a cost,
    /// or a catenative such as *choose to*.
    Other,
}

impl Context {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::EffectClause => "effect clause",
            Self::TriggerClause => "trigger clause",
            Self::WouldClause => "would clause",
            Self::Deontic => "deontic",
            Self::RelativeClause => "relative clause",
            Self::ParticipialPostmodifier => "participial postmodifier",
            Self::Other => "other",
        }
    }
}

/// The grammar and lexicon an Embedding is read with.
pub(crate) struct Reader<'a> {
    grammar: &'a CompiledGrammar,
    lexicon: &'a Lexicon,
}

impl<'a> Reader<'a> {
    /// # Panics
    /// When the grammar has no Category or Production the reading names.
    pub(crate) fn new(grammar: &'a CompiledGrammar, lexicon: &'a Lexicon) -> Self {
        for category in [
            "Sentence",
            "PrepositionPhrase",
            "RelativeClause",
            "Postmodifier",
            "VerbPhrase",
        ] {
            assert!(
                grammar.category_id(category).is_some(),
                "the grammar has no Category `{category}`"
            );
        }
        for production in [
            "FusedRelative",
            "FusedSubjectRelative",
            "AdjectivalPredicative",
        ] {
            assert!(
                (0..grammar.production_count()).any(|p| grammar.production_id(p) == production),
                "the grammar has no Production `{production}`"
            );
        }
        Self { grammar, lexicon }
    }

    fn category(&self, production: usize) -> &str {
        self.grammar.category_names()[self.grammar.production_category(production)].as_str()
    }

    fn frame(&self, production: usize) -> Option<String> {
        self.grammar
            .production_frame(production)
            .map(|(_, label, _)| label)
    }

    fn governor_lemma(&self, step: &EmbeddingStep) -> Option<&str> {
        let (lexeme, _) = step.governor_head?;
        let lexeme = self
            .lexicon
            .lexemes()
            .get(&lexeme)
            .unwrap_or_else(|| panic!("lexeme `{lexeme}` is not in the lexicon"));
        Some(lexeme.lemma.as_str())
    }

    /// Whether a step is a wrapper, kept in an Embedding but not counted and
    /// never a context: the step carries the verb phrase into the
    /// construction that has its context, adding none of its own.
    ///
    /// - An Auxiliary Complement (`production_auxiliary`): the participle under
    ///   perfect *have* or passive or progressive *be*.
    /// - A part of a coordination (`production_is_coordination`): a conjunct,
    ///   which bears its coordination's function (CGEL, Ch. 15 §1, p. 1278).
    /// - The Complement of *do* in its `Catenative(BareInf)` frame: do-support
    ///   (*don't untap*).
    /// - An adjectival passive (`AdjectivalPredicative`) as the predicative
    ///   Complement of *be*: the adjectival counterpart of the passive
    ///   auxiliary's Complement (*is revealed*).
    pub(crate) fn wrapper(&self, step: &EmbeddingStep) -> bool {
        let grammar = self.grammar;
        let catenative = self.frame(step.governor).as_deref() == Some("Catenative(BareInf)");
        grammar.production_auxiliary(step.governor) == Some(step.child)
            || grammar.production_is_coordination(step.governor)
            || (catenative && self.governor_lemma(step) == Some("do"))
            || (grammar.production_id(step.projection) == "AdjectivalPredicative"
                && self.governor_lemma(step) == Some("be"))
    }

    /// The context of a verb phrase under an Embedding.
    pub(crate) fn context(&self, embedding: &Embedding) -> Context {
        let Some(step) = embedding.steps.iter().find(|step| !self.wrapper(step)) else {
            return Context::Other;
        };
        let lemma = self.governor_lemma(step);
        let catenative = self.frame(step.governor).as_deref() == Some("Catenative(BareInf)");
        let preterite = step
            .governor_head
            .is_some_and(|(_, form)| form == WordForm::Preterite);
        let function = step.function.as_str();
        if catenative && function == "complement" && lemma == Some("will") && preterite {
            Context::WouldClause
        } else if catenative
            && function == "complement"
            && matches!(lemma, Some("can" | "may" | "must"))
        {
            Context::Deontic
        } else if self.category(step.governor) == "PrepositionPhrase"
            && function == "complement"
            && matches!(lemma, Some("when" | "whenever"))
        {
            Context::TriggerClause
        } else if self.category(step.governor) == "RelativeClause"
            || matches!(
                self.grammar.production_id(step.governor).as_str(),
                "FusedRelative" | "FusedSubjectRelative"
            )
        {
            Context::RelativeClause
        } else if self.category(step.projection) == "Postmodifier"
            || self.category(step.governor) == "Postmodifier"
            || (function == "modifier" && self.category(step.projection) == "VerbPhrase")
        {
            Context::ParticipialPostmodifier
        } else if self.category(step.governor) == "Sentence" && function == "clause" {
            Context::EffectClause
        } else {
            Context::Other
        }
    }
}

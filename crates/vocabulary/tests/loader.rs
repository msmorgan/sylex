//! The loader rejects declarations that would break the catalog contract.

use sylex_vocabulary::LexicalEnvironment;
use sylex_vocabulary::Sources;
use sylex_vocabulary::schema::FrameSpec;
use sylex_vocabulary::schema::NounDecl;
use sylex_vocabulary::schema::VerbDecl;
use sylex_vocabulary::schema::VerbEntry;

fn verb(lemma: &str, frames: Vec<FrameSpec>) -> VerbEntry {
    VerbEntry {
        lemma: lemma.to_owned(),
        third_singular: None,
        past: None,
        preterite: None,
        past_participle: None,
        gerund: None,
        irrealis: None,
        forms: Vec::new(),
        finite_only: false,
        negative: false,
        modal: false,
        form_of: None,
        binding: None,
        frames,
        classes: Vec::new(),
    }
}

fn rejects(sources: &Sources, reason: &str) {
    let error = sources.vocabulary().expect_err(reason).to_string();
    assert!(error.contains(reason), "{error}");
}

#[test]
fn a_catalog_word_is_never_core_vocabulary() {
    let mut sources = Sources::load().unwrap();
    let VerbDecl::Entry(action) = &sources.keyword_actions[0].verbs[0] else {
        panic!("expected inline keyword action verb");
    };
    sources.verbs.push((**action).clone());
    rejects(&sources, "must not also be core vocabulary");

    let mut sources = Sources::load().unwrap();
    let shared = sources.catalog_verbs[0].clone();
    sources.verbs.push(shared);
    rejects(&sources, "must not also be core vocabulary");

    let mut sources = Sources::load().unwrap();
    let card_type = sources.types.card_types[0].clone();
    sources.nouns.push(card_type);
    rejects(&sources, "must not also be core vocabulary");
}

#[test]
fn shared_catalog_verbs_are_declared_once_and_named_by_keywords() {
    let mut sources = Sources::load().unwrap();
    sources.keyword_actions[0]
        .verbs
        .push(VerbDecl::Shared("frobnicate".to_owned()));
    rejects(&sources, "is not declared in catalogs/verbs.ron");

    let mut sources = Sources::load().unwrap();
    sources.keyword_abilities[0]
        .verbs
        .push(VerbDecl::Shared("frobnicate".to_owned()));
    rejects(&sources, "is not declared in catalogs/verbs.ron");

    let mut sources = Sources::load().unwrap();
    sources.keyword_actions[0]
        .verbs
        .push(VerbDecl::Entry(Box::new(verb(
            "face",
            vec![FrameSpec::Monotransitive],
        ))));
    rejects(&sources, "already a shared verb `face`");

    let mut sources = Sources::load().unwrap();
    sources.keyword_abilities[0]
        .verbs
        .push(VerbDecl::Entry(Box::new(verb(
            "beam",
            vec![FrameSpec::Monotransitive],
        ))));
    rejects(&sources, "already a shared verb `beam`");

    let mut sources = Sources::load().unwrap();
    sources
        .catalog_verbs
        .push(verb("Face", vec![FrameSpec::Intransitive]));
    rejects(&sources, "duplicates `face`");

    let mut sources = Sources::load().unwrap();
    sources
        .catalog_verbs
        .push(verb("frobnicate", vec![FrameSpec::Intransitive]));
    rejects(&sources, "is named by no keyword entry");
}

#[test]
fn a_verb_has_a_frame() {
    let mut sources = Sources::load().unwrap();
    sources.verbs.push(verb("frobnicate", Vec::new()));
    rejects(&sources, "at least one Subcategorization Frame");
}

#[test]
fn a_passive_only_frame_has_a_passive() {
    let mut sources = Sources::load().unwrap();
    sources.verbs.push(verb(
        "frobnicate",
        vec![FrameSpec::PassiveOnly(Box::new(FrameSpec::Intransitive))],
    ));
    rejects(&sources, "is Passive-only but has no passive");
}

#[test]
fn a_frame_restricts_its_voice_once() {
    let mut sources = Sources::load().unwrap();
    sources.verbs.push(verb(
        "frobnicate",
        vec![FrameSpec::ActiveOnly(Box::new(FrameSpec::PassiveOnly(
            Box::new(FrameSpec::Monotransitive),
        )))],
    ));
    rejects(&sources, "inside a restricted frame");
}

#[test]
fn a_frame_names_declared_prepositions() {
    let mut sources = Sources::load().unwrap();
    sources.verbs.push(verb(
        "frobnicate",
        vec![FrameSpec::Prepositional(vec!["amidst-ish".to_owned()])],
    ));
    rejects(&sources, "not a declared preposition");
}

#[test]
fn a_frame_belongs_to_its_head_category() {
    let mut sources = Sources::load().unwrap();
    sources
        .nouns
        .push(NounDecl::Entry(sylex_vocabulary::schema::NounEntry {
            distribution: sylex_vocabulary::schema::NounDistribution::Ordinary,
            kind: sylex_vocabulary::schema::NounKind::Ordinary,
            relative_gender: None,
            parameters: Vec::new(),
            lemma: "frobnication".to_owned(),
            countability: vec![sylex_lexicon::Countability::Count],
            bare_mass_modified: false,
            plural: None,
            frames: vec![FrameSpec::Monotransitive],
            distinct_genitive: false,
            prefix_hyphenation: None,
            onset: None,
            classes: Vec::new(),
        }));
    rejects(&sources, "is not a frame of a Noun");
}

#[test]
fn an_identity_is_declared_once() {
    let mut sources = Sources::load().unwrap();
    let noun = sources.nouns[0].clone();
    sources.nouns.push(noun);
    rejects(&sources, "is declared twice");
}

/// A name is number-neutral: the text it stands in fixes its Number
/// (*When The Notary Hobbits enter*, *Mizzium Mortars deals*), so the
/// lexicon declares none.
#[test]
fn a_name_declares_no_number() {
    let env = LexicalEnvironment::load().unwrap();
    for lexeme in env
        .lexicon
        .lexemes()
        .values()
        .filter(|lexeme| lexeme.category == sylex_lexicon::Category::Catalog)
    {
        assert!(
            !lexeme.properties.features.contains_key("Number")
                && lexeme
                    .forms
                    .iter()
                    .all(|slot| slot.features.number.is_none()),
            "{}",
            lexeme.id.as_str()
        );
    }
}

/// Every subtype takes a prefix hyphenated (*non-Vampire*), so none declares
/// it: a declaration would be overwritten or contradicted.
#[test]
fn a_subtype_declares_no_prefix_hyphenation() {
    let mut sources = Sources::load().unwrap();
    let decls = sources.subtypes.values_mut().next().unwrap();
    let mut entry = decls[0].entry();
    entry.prefix_hyphenation = Some(sylex_lexicon::Hyphenation::Hyphenated);
    decls[0] = sylex_vocabulary::schema::NounDecl::Entry(entry);
    rejects(&sources, "declares a prefix hyphenation");
}

/// A frame's Complement Constraints reach the frame catalog as declared:
/// each of *among*'s names `Plural` (*from among flying, first strike,
/// lifelink, or vigilance*, Elspeth Resplendent), and its cited-name frame
/// `KindSelected` too (*from among Disenchant, ..., and Black Lotus*).
#[test]
fn a_frames_constraints_reach_the_frame_catalog() {
    let sources = Sources::load().unwrap();
    let (_, frames) = sources.vocabulary().unwrap();
    let names = |frame: &sylex_syntax::frames::PrepositionFrame| {
        frame
            .constraints
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    };
    for (id, list) in &frames.prepositions {
        let among = id.as_str() == "prep:among";
        assert!(!list.is_empty(), "{}", id.as_str());
        for frame in list {
            assert_eq!(
                names(frame).contains(&"Plural".to_owned()),
                among,
                "{frame:?}"
            );
        }
    }
    let among = &frames
        .prepositions
        .iter()
        .find(|(id, _)| id.as_str() == "prep:among")
        .unwrap()
        .1;
    let name = among
        .iter()
        .find(|frame| frame.complement == sylex_syntax::frames::SlotFill::Name)
        .unwrap();
    assert_eq!(names(name), ["KindSelected", "Plural"]);
}

/// A declared Adjective spelled like a verb's past participle is that verb's
/// Adjectival Passive, and the loader marks both: *attached* and *attach*.
/// *Tied* has no verb, and an attributive-only Adjective (*the left half*)
/// is no Adjectival Passive of *leave*, so none of these is marked.
#[test]
fn an_adjectival_passive_marks_its_adjective_and_its_verb() {
    let env = LexicalEnvironment::load().unwrap();
    let marked = |id: &str| {
        let lexeme = env
            .lexicon
            .lexemes()
            .values()
            .find(|lexeme| lexeme.id.as_str() == id)
            .unwrap_or_else(|| panic!("no lexeme {id}"));
        lexeme
            .properties
            .features
            .get("AdjectivalPassive")
            .is_some_and(|value| value.as_str() == "Lexical")
    };
    assert!(marked("adj:attached"));
    assert!(marked("action:attach"));
    assert!(!marked("adj:tied"));
    assert!(!marked("adj:left"));
    assert!(!marked("verb:leave"));
}

#[test]
fn bare_mass_modified_requires_mass_countability() {
    let mut sources = Sources::load().unwrap();
    sources
        .nouns
        .push(NounDecl::Entry(sylex_vocabulary::schema::NounEntry {
            distribution: sylex_vocabulary::schema::NounDistribution::Ordinary,
            kind: sylex_vocabulary::schema::NounKind::Ordinary,
            relative_gender: None,
            parameters: Vec::new(),
            lemma: "frobnication".to_owned(),
            countability: vec![sylex_lexicon::Countability::Count],
            bare_mass_modified: true,
            plural: None,
            frames: Vec::new(),
            distinct_genitive: false,
            prefix_hyphenation: None,
            onset: None,
            classes: Vec::new(),
        }));
    rejects(&sources, "requires `Mass` countability");
}

/// A Set Name is a noun's one parameter, right before it (*the Antiquities
/// expansion*), where analysis reads it; a pattern that puts one elsewhere
/// is refused, since analysis would read a Set Name no Production takes.
#[test]
fn a_set_name_parameter_stands_right_before_its_noun() {
    use sylex_syntax::frames::ParameterPart;
    use sylex_syntax::frames::SlotKind;
    let sources = Sources::load().unwrap();
    for parts in [
        vec![ParameterPart::Head, ParameterPart::Slot(SlotKind::SetName)],
        vec![
            ParameterPart::Slot(SlotKind::SetName),
            ParameterPart::Head,
            ParameterPart::Slot(SlotKind::Amount),
        ],
    ] {
        let mut misplaced = sources.clone();
        let at = misplaced
            .nouns
            .iter()
            .position(|noun| noun.entry().lemma == "expansion")
            .unwrap();
        let mut expansion = misplaced.nouns[at].entry();
        expansion.parameters = vec![parts];
        misplaced.nouns[at] = NounDecl::Entry(expansion);
        rejects(&misplaced, "a Set Name is a whole pattern's one parameter");
    }
}

/// A cited Vote Option is a noun's one parameter, right before it (*each
/// numbers vote* [CR#701.38b]), as a Set Name is, and the noun that takes it
/// is the head of its license (`docs/design/english.md` §3).
#[test]
fn a_cited_option_parameter_stands_right_before_its_noun() {
    use sylex_syntax::frames::ParameterPart;
    use sylex_syntax::frames::SlotKind;
    let sources = Sources::load().unwrap();
    let (lexemes, _) = sources.vocabulary().unwrap();
    let heads: Vec<&str> = lexemes
        .iter()
        .filter(|lexeme| lexeme.properties.features.contains_key("OptionLicense"))
        .map(|lexeme| lexeme.lemma.as_str())
        .collect();
    assert_eq!(heads, ["vote"]);
    let mut misplaced = sources.clone();
    let at = misplaced
        .nouns
        .iter()
        .position(|noun| noun.entry().lemma == "vote")
        .unwrap();
    let mut vote = misplaced.nouns[at].entry();
    vote.parameters = vec![vec![
        ParameterPart::Head,
        ParameterPart::Slot(SlotKind::OptionMention),
    ]];
    misplaced.nouns[at] = NounDecl::Entry(vote);
    rejects(
        &misplaced,
        "a cited option is a whole pattern's one parameter",
    );
}

/// A verb's Prepositional Complement of a named kind is a Preposition
/// Phrase (*votes for sickness or psychosis*: `PPWith(["for"],
/// OptionChoice)`).
#[test]
fn a_prepositional_complement_frame_names_a_preposition_phrase() {
    use sylex_vocabulary::schema::FillSpec;
    let mut sources = Sources::load().unwrap();
    sources.verbs.push(verb(
        "frobnicate",
        vec![FrameSpec::PrepositionalComplement(FillSpec::OptionChoice)],
    ));
    rejects(&sources, "names no Preposition Phrase");
}

/// A noun's *of* Complement frame alternates with a genitive determiner
/// unless the noun's genitive names something else: *hand* (*your hand*, *the
/// hand of that player*) and *mana cost*'s *for* or *of* frame do; *choice*
/// (*your choice of a creature type*), a phase (*your first main phase of the
/// game*) and *kind* (*each kind of counter*) do not, and no frame without
/// *of* does (*devotion to black*).
#[test]
fn an_of_frame_alternates_with_a_genitive_unless_its_noun_has_a_distinct_one() {
    let sources = Sources::load().unwrap();
    let (_, frames) = sources.vocabulary().unwrap();
    let alternants = |lexeme: &str| {
        frames
            .frames
            .iter()
            .find(|(id, _)| id.as_str() == lexeme)
            .unwrap_or_else(|| panic!("no frames for `{lexeme}`"))
            .1
            .iter()
            .filter(|frame| frame.genitive_alternant)
            .count()
    };
    assert_eq!(alternants("noun:hand"), 1);
    assert_eq!(alternants("noun:mana cost"), 1);
    assert_eq!(alternants("noun:choice"), 0);
    assert_eq!(alternants("turn_part:main phase"), 0);
    assert_eq!(alternants("noun:kind"), 0);
    assert_eq!(alternants("noun:devotion"), 0);
}

#[test]
fn only_a_noun_with_an_of_frame_has_a_distinct_genitive() {
    let mut sources = Sources::load().unwrap();
    sources
        .nouns
        .push(NounDecl::Entry(sylex_vocabulary::schema::NounEntry {
            distribution: sylex_vocabulary::schema::NounDistribution::Ordinary,
            kind: sylex_vocabulary::schema::NounKind::Ordinary,
            relative_gender: None,
            parameters: Vec::new(),
            lemma: "frobnication".to_owned(),
            countability: vec![sylex_lexicon::Countability::Count],
            bare_mass_modified: false,
            plural: None,
            frames: vec![FrameSpec::Complement(
                sylex_vocabulary::schema::FillSpec::PP(vec!["to".to_owned()]),
            )],
            distinct_genitive: true,
            prefix_hyphenation: None,
            onset: None,
            classes: Vec::new(),
        }));
    rejects(
        &sources,
        "only a noun with an *of* Complement frame has a distinct genitive",
    );
}

/// A noun's relative gender is its lexeme's: a player is personal (*each
/// player who*), a card non-personal, and a creature, a creature type and a
/// planeswalker type carry none, since a being takes *who* or *which* (CGEL,
/// Ch. 5 §17.3, p. 498); another subtype names a thing. Relative *who* is
/// personal and *which* non-personal (Ch. 12 §3.5.1, p. 1048).
#[test]
fn a_relative_gender_is_declared_on_nouns_and_relative_pronouns() {
    let env = LexicalEnvironment::load().unwrap();
    let gender = |id: &str| {
        let lexeme = env
            .lexicon
            .lexemes()
            .values()
            .find(|lexeme| lexeme.id.as_str() == id)
            .unwrap_or_else(|| panic!("no lexeme `{id}`"));
        lexeme
            .properties
            .features
            .get("RelativeGender")
            .map(|value| value.as_str().to_owned())
    };
    let personal = Some("Personal".to_owned());
    let non_personal = Some("NonPersonal".to_owned());
    assert_eq!(gender("noun:player"), personal);
    assert_eq!(gender("noun:opponent"), personal);
    assert_eq!(gender("noun:card"), non_personal);
    assert_eq!(gender("noun:damage"), non_personal);
    assert_eq!(gender("type:creature"), None);
    assert_eq!(gender("type:artifact"), non_personal);
    assert_eq!(gender("subtype:Goblin"), None);
    assert_eq!(gender("subtype:Ajani"), None);
    assert_eq!(gender("subtype:Equipment"), non_personal);
    assert_eq!(gender("pronoun:who"), personal);
    assert_eq!(gender("pronoun:which"), non_personal);
    assert_eq!(gender("pronoun:it"), None);
}

#[test]
fn a_subtype_s_family_decides_its_relative_gender() {
    let mut sources = Sources::load().unwrap();
    let decls = sources
        .subtypes
        .get_mut(&sylex_vocabulary::schema::SubtypeFamily::Artifact)
        .unwrap();
    let mut entry = decls[0].entry();
    entry.relative_gender = Some(sylex_vocabulary::schema::RelativeGender::NonPersonal);
    decls[0] = NounDecl::Entry(entry);
    rejects(
        &sources,
        "declares a relative gender, which its family decides",
    );
}

#[test]
fn only_a_relative_pronoun_has_a_relative_gender_and_never_dual() {
    use sylex_vocabulary::schema::RelativeGender;
    let at = |sources: &Sources, lemma: &str| {
        sources
            .closed_class
            .pronouns
            .iter()
            .position(|entry| entry.lemma == lemma)
            .unwrap()
    };
    let mut personal = Sources::load().unwrap();
    let it = at(&personal, "it");
    personal.closed_class.pronouns[it].relative_gender = Some(RelativeGender::NonPersonal);
    rejects(&personal, "only a relative pronoun has one");
    let mut dual = Sources::load().unwrap();
    let who = at(&dual, "who");
    dual.closed_class.pronouns[who].relative_gender = Some(RelativeGender::Dual);
    rejects(&dual, "only a relative pronoun has one");
}

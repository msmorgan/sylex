//! The loader rejects declarations that would break the catalog contract.

use sylex_vocabulary::LexicalEnvironment;
use sylex_vocabulary::Sources;
use sylex_vocabulary::schema::AnchorWord;
use sylex_vocabulary::schema::FrameSpec;
use sylex_vocabulary::schema::NounDecl;
use sylex_vocabulary::schema::ScopedName;
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
        active_only: false,
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
fn only_a_verb_with_frames_restricts_them_to_active_voice() {
    let mut sources = Sources::load().unwrap();
    let mut auxiliary = verb("frobnicate", Vec::new());
    auxiliary.active_only = true;
    sources.closed_class.auxiliaries.push(auxiliary);
    rejects(&sources, "restricts them to active voice");
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
            parameters: Vec::new(),
            lemma: "frobnication".to_owned(),
            countability: vec![sylex_lexicon::Countability::Count],
            bare_mass_modified: false,
            plural: None,
            frames: vec![FrameSpec::Monotransitive],
            prefix_hyphenation: None,
            onset: None,
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

#[test]
fn a_card_scope_names_cards_and_a_scoped_name_may_be_spelled_like_one() {
    let card_names = "Dark Depths\nMarit Lage's Slumber\nCitadel Siege\n";
    let mut sources = Sources::load().unwrap();
    sources
        .introduced_names
        .retain(|name| name.name == "Marit Lage");
    sources.anchor_words.clear();
    sources.vote_options.clear();
    sources.environment(card_names).unwrap();

    let mut stray = sources.clone();
    stray.introduced_names[0]
        .cards
        .push("Not a Card".to_owned());
    let error = stray.environment(card_names).unwrap_err().to_string();
    assert!(error.contains("which is not a card name"), "{error}");

    let mut stray = sources.clone();
    stray.anchor_words.push(AnchorWord {
        word: "Khans".to_owned(),
        cards: vec!["Not a Card".to_owned()],
    });
    let error = stray.environment(card_names).unwrap_err().to_string();
    assert!(error.contains("which is not a card name"), "{error}");

    // A scoped name may be spelled like a card's name (Disa the Restless's
    // *create a Tarmogoyf token*): another card's name stands only after
    // *named*, *Partner with* or *from among*, so the two never compete
    // elsewhere.
    let env = sources
        .environment(&format!("{card_names}Marit Lage\n"))
        .unwrap();
    let lexeme = |id: &str| &env.lexicon.lexemes()[&sylex_lexicon::LexemeId::new(id)];
    assert!(lexeme("name:Marit Lage").card_name);
    assert!(!lexeme("introduced_name:Marit Lage").card_name);
    let mut spelled_alike = sources.clone();
    spelled_alike.vote_options.push(scoped("Dark Depths"));
    spelled_alike.environment(card_names).unwrap();
}

fn scoped(name: &str) -> ScopedName {
    ScopedName {
        name: name.to_owned(),
        cards: vec!["Citadel Siege".to_owned()],
    }
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
            parameters: Vec::new(),
            lemma: "frobnication".to_owned(),
            countability: vec![sylex_lexicon::Countability::Count],
            bare_mass_modified: true,
            plural: None,
            frames: Vec::new(),
            prefix_hyphenation: None,
            onset: None,
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

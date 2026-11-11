//! The loader rejects declarations that would break the catalog contract.

use sylex_vocabulary::AttestedShortenings;
use sylex_vocabulary::Sources;
use sylex_vocabulary::schema::AnchorWord;
use sylex_vocabulary::schema::FrameSpec;
use sylex_vocabulary::schema::NounDecl;
use sylex_vocabulary::schema::ScopedName;
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
    let action = sources.keyword_actions[0].verbs[0].clone();
    sources.verbs.push(action);
    rejects(&sources, "must not also be core vocabulary");

    let mut sources = Sources::load().unwrap();
    let card_type = sources.types.card_types[0].clone();
    sources.nouns.push(card_type);
    rejects(&sources, "must not also be core vocabulary");
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
            plural: None,
            frames: vec![FrameSpec::Monotransitive],
            variable: false,
            prefix_hyphenation: None,
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
    sources.cited_names.clear();
    sources.plural_names.clear();
    let none = AttestedShortenings::default();
    sources.environment(card_names, &none).unwrap();

    let mut stray = sources.clone();
    stray.introduced_names[0]
        .cards
        .push("Not a Card".to_owned());
    let error = stray
        .environment(card_names, &none)
        .unwrap_err()
        .to_string();
    assert!(error.contains("which is not a card name"), "{error}");

    let mut stray = sources.clone();
    stray.anchor_words.push(AnchorWord {
        word: "Khans".to_owned(),
        cards: vec!["Not a Card".to_owned()],
    });
    let error = stray
        .environment(card_names, &none)
        .unwrap_err()
        .to_string();
    assert!(error.contains("which is not a card name"), "{error}");

    // A scoped name may be spelled like a card's name (Disa the Restless's
    // *create a Tarmogoyf token*): another card's name stands only after
    // *named*, *Partner with* or *from among*, so the two never compete
    // elsewhere.
    let env = sources
        .environment(&format!("{card_names}Marit Lage\n"), &none)
        .unwrap();
    let lexeme = |id: &str| &env.lexicon.lexemes()[&sylex_lexicon::LexemeId::new(id)];
    assert!(lexeme("name:Marit Lage").card_name);
    assert!(!lexeme("introduced_name:Marit Lage").card_name);
    for push in [
        |sources: &mut Sources| sources.vote_options.push(scoped("Dark Depths")),
        |sources: &mut Sources| sources.cited_names.push(scoped("Dark Depths")),
    ] {
        let mut spelled_alike = sources.clone();
        push(&mut spelled_alike);
        spelled_alike.environment(card_names, &none).unwrap();
    }
}

fn scoped(name: &str) -> ScopedName {
    ScopedName {
        name: name.to_owned(),
        cards: vec!["Citadel Siege".to_owned()],
    }
}

#[test]
fn a_name_is_singular_unless_listed_plural_and_a_plural_name_names_a_card() {
    let card_names = "Dark Depths\nCitadel Siege\n";
    let mut sources = Sources::load().unwrap();
    sources.introduced_names.clear();
    sources.anchor_words.clear();
    sources.vote_options.clear();
    sources.cited_names.clear();
    sources.plural_names = vec!["Citadel Siege".to_owned()];
    let none = AttestedShortenings::default();
    let env = sources.environment(card_names, &none).unwrap();
    let number = |name: &str| {
        env.lexicon.lexemes()[&sylex_lexicon::LexemeId::new(&format!("name:{name}"))]
            .properties
            .features
            .get("Number")
            .map(ToString::to_string)
    };
    assert_eq!(number("Dark Depths").as_deref(), Some("Singular"));
    assert_eq!(number("Citadel Siege").as_deref(), Some("Plural"));

    sources.plural_names.push("Not a Card".to_owned());
    let error = sources
        .environment(card_names, &none)
        .unwrap_err()
        .to_string();
    assert!(error.contains("names no card"), "{error}");
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

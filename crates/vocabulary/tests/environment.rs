//! The whole Lexical Environment loads, and its vocabulary analyses as
//! declared.

use sylex_lexicon::LexicalReading;
use sylex_vocabulary::LexicalEnvironment;

#[test]
fn the_environment_loads() {
    let environment = LexicalEnvironment::load().unwrap();
    let analyzed = environment.lexicon.analyze("Destroy target creature.");
    let ids: Vec<String> = analyzed
        .matches
        .iter()
        .filter_map(|m| match &m.reading {
            LexicalReading::Word(value) => Some(value.lexeme.to_string()),
            _ => None,
        })
        .collect();
    assert!(ids.contains(&"action:destroy".to_owned()), "{ids:?}");
}

#[test]
fn capitalized_prefix_has_one_lexical_analysis() {
    let environment = LexicalEnvironment::load().unwrap();
    for (text, prefix_end) in [("Nonbasic", 3), ("Non-basic", 4)] {
        let analyzed = environment.lexicon.analyze(text);
        let prefixes: Vec<_> = analyzed.matches.iter().filter(|found| {
            found.start == 0 && found.end == prefix_end && matches!(&found.reading, LexicalReading::Word(value) if value.lexeme.as_str() == "affix:non")
        }).collect();
        assert_eq!(prefixes.len(), 1, "{text}: {prefixes:?}");
    }
}

#[test]
fn catalog_determinatives_are_explicit_and_retain_verbal_homographs() {
    use sylex_lexicon::Category;
    use sylex_lexicon::SourceKind;
    use sylex_lexicon::WordForm;
    let env = LexicalEnvironment::load().unwrap();
    let sources = sylex_vocabulary::Sources::load().unwrap();
    for (text, ability) in [("enchanted", "enchant"), ("equipped", "equip")] {
        let owner = sources
            .keyword_abilities
            .iter()
            .find(|entry| entry.lemma == ability)
            .unwrap()
            .rule
            .number();
        let analyzed = env.lexicon.analyze(text);
        let values: Vec<_> = analyzed
            .matches
            .iter()
            .filter_map(|m| {
                if m.start != 0 || m.end != text.chars().count() {
                    return None;
                }
                match &m.reading {
                    LexicalReading::Word(v) => Some(v),
                    _ => None,
                }
            })
            .collect();
        assert!(
            values.iter().any(|v| {
                let l = &env.lexicon.lexemes()[&v.lexeme];
                l.category == Category::Determinative
                    && l.source.kind == SourceKind::Catalog
                    && l.source.owner == owner
            }),
            "{text}"
        );
        assert!(
            values.iter().any(
                |v| env.lexicon.lexemes()[&v.lexeme].category == Category::Verb
                    && v.form == WordForm::PastParticiple
            ),
            "{text}"
        );
    }
    for text in ["tapped", "white", "enchanting", "equipping"] {
        let analyzed = env.lexicon.analyze(text);
        assert!(!analyzed.matches.iter().any(|m| matches!(&m.reading, LexicalReading::Word(v) if env.lexicon.lexemes()[&v.lexeme].category == Category::Determinative)), "{text}");
    }
}

/// The combat roles [CR#506.2] are determinatives of the core closed class (the
/// verbs *attack* and *defend* are core vocabulary, not CR catalog items), and
/// each spelling is also the gerund-participle of a core verb (*the attacking
/// player*, *each defending player*), as *enchanted* is beside *enchant*.
#[test]
fn combat_role_determinatives_retain_verbal_homographs() {
    use sylex_lexicon::Category;
    use sylex_lexicon::SourceKind;
    use sylex_lexicon::WordForm;
    let env = LexicalEnvironment::load().unwrap();
    for text in ["attacking", "defending"] {
        let analyzed = env.lexicon.analyze(text);
        let values: Vec<_> = analyzed
            .matches
            .iter()
            .filter(|m| m.start == 0 && m.end == text.chars().count())
            .filter_map(|m| match &m.reading {
                LexicalReading::Word(v) => Some(v),
                _ => None,
            })
            .collect();
        assert!(
            values.iter().any(|v| {
                let l = &env.lexicon.lexemes()[&v.lexeme];
                l.category == Category::Determinative
                    && l.source.kind == SourceKind::Core
                    && l.id.as_str() == format!("det:{text}")
            }),
            "{text}"
        );
        assert!(
            values.iter().any(
                |v| env.lexicon.lexemes()[&v.lexeme].category == Category::Verb
                    && v.form == WordForm::GerundParticiple
            ),
            "{text}"
        );
    }
    for text in ["attacked", "defended", "blocking"] {
        let analyzed = env.lexicon.analyze(text);
        assert!(!analyzed.matches.iter().any(|m| matches!(&m.reading, LexicalReading::Word(v) if env.lexicon.lexemes()[&v.lexeme].category == Category::Determinative)), "{text}");
    }
}

#[test]
fn a_modal_preterite_has_no_tense() {
    use sylex_lexicon::Tense;
    use sylex_lexicon::WordForm;
    // *If you would draw a card while your library has no cards in it*
    // (Laboratory Maniac). Another primary form keeps its tense.
    let environment = LexicalEnvironment::load().unwrap();
    let tenses = |text: &str, lexeme: &str| -> Vec<Option<Tense>> {
        environment
            .lexicon
            .analyze(text)
            .matches
            .iter()
            .filter_map(|m| match &m.reading {
                LexicalReading::Word(value)
                    if value.lexeme.as_str() == lexeme
                        && matches!(value.form, WordForm::Present | WordForm::Preterite) =>
                {
                    Some(value.features.tense)
                }
                _ => None,
            })
            .collect()
    };
    for (text, lexeme) in [
        ("would", "verb:will"),
        ("could", "verb:can"),
        ("couldn't", "verb:can't"),
        ("might", "verb:may"),
    ] {
        let found = tenses(text, lexeme);
        assert!(
            !found.is_empty() && found.iter().all(Option::is_none),
            "{text}: {found:?}"
        );
    }
    for (text, lexeme, tense) in [
        ("will", "verb:will", Tense::Present),
        ("must", "verb:must", Tense::Present),
        ("was", "verb:be", Tense::Past),
        ("didn't", "verb:don't", Tense::Past),
    ] {
        let found = tenses(text, lexeme);
        assert!(
            !found.is_empty() && found.iter().all(|t| *t == Some(tense)),
            "{text}: {found:?}"
        );
    }
}

#[test]
fn shared_catalog_verbs_belong_to_neither_keyword_entry() {
    use sylex_lexicon::Category;
    use sylex_lexicon::SourceKind;

    let env = LexicalEnvironment::load().unwrap();
    for (lemma, expected_id) in [("face", "catalog_verb:face"), ("beam", "catalog_verb:beam")] {
        let verbs: Vec<_> = env
            .lexicon
            .lexemes()
            .values()
            .filter(|l| l.category == Category::Verb && l.lemma == lemma)
            .collect();
        assert_eq!(verbs.len(), 1, "{lemma}: {verbs:?}");
        let verb = verbs[0];
        assert_eq!(verb.id.as_str(), expected_id);
        assert_eq!(verb.source.kind, SourceKind::Catalog);
        assert_eq!(verb.source.path, "catalogs/verbs.ron");
        assert_eq!(verb.source.owner, expected_id);
    }

    for (text, word, expected_id) in [
        (
            "Whenever you face a dilemma, draw a card.",
            "face",
            "catalog_verb:face",
        ),
        (
            "Target opponent faces a villainous choice — They discard a card, or you draw a card.",
            "faces",
            "catalog_verb:face",
        ),
        (
            "Creatures can't be beamed up.",
            "beamed",
            "catalog_verb:beam",
        ),
    ] {
        let analyzed = env.lexicon.analyze(text);
        let verbal: std::collections::BTreeSet<_> = analyzed
            .matches
            .iter()
            .filter(|m| {
                let surface: String = analyzed.tokens[m.start..m.end].iter().collect();
                surface == word
            })
            .filter_map(|m| match &m.reading {
                LexicalReading::Word(v)
                    if env.lexicon.lexemes()[&v.lexeme].category == Category::Verb =>
                {
                    Some(v.lexeme.as_str().to_owned())
                }
                _ => None,
            })
            .collect();
        assert_eq!(
            verbal,
            std::collections::BTreeSet::from([expected_id.to_owned()]),
            "{text}"
        );
    }
}

use std::collections::BTreeSet;

use sylex_lexicon::*;

fn source(owner: &str) -> Source {
    Source {
        kind: SourceKind::Core,
        path: "tests/declared-lexicon.ron".to_owned(),
        owner: owner.to_owned(),
    }
}

fn invariant(id: &str, surface: &str, category: Category) -> Lexeme {
    Lexeme::invariant(id, surface, category, source(id))
}

fn multiword(id: &str, surface: &str, category: Category) -> Lexeme {
    let mut lexeme = invariant(id, surface, category);
    lexeme.surface_structure = SurfaceStructure::Multiword;
    lexeme
}

fn noun(id: &str, lemma: &str) -> Lexeme {
    Lexeme::noun(id, lemma, vec![Countability::Count], source(id))
}

fn verb(id: &str, lemma: &str) -> Lexeme {
    Lexeme::verb(id, lemma, source(id))
}

fn nominal(number: Number) -> FeatureBundle {
    FeatureBundle {
        number: Some(number),
        ..FeatureBundle::default()
    }
}

fn finite(person: Person, number: Number, tense: Tense) -> FeatureBundle {
    FeatureBundle {
        number: Some(number),
        person: Some(person),
        tense: Some(tense),
        finiteness: Some(Finiteness::Finite),
        ..FeatureBundle::default()
    }
}

fn nonfinite() -> FeatureBundle {
    FeatureBundle {
        finiteness: Some(Finiteness::Nonfinite),
        ..FeatureBundle::default()
    }
}

fn word(id: &str, form: WordForm, features: FeatureBundle) -> LexicalReading {
    LexicalReading::Word(LexicalValue {
        lexeme: id.into(),
        form,
        features,
    })
}

fn numeral(value: i32, notation: Numeral) -> LexicalReading {
    LexicalReading::Numeral { value, notation }
}

/// The readings of a span, less the title run every capitalized word
/// starts ([`title_runs`]).
fn at(text: &AnalyzedText, start: usize, end: usize) -> BTreeSet<LexicalReading> {
    text.matches
        .iter()
        .filter(|m| m.start == start && m.end == end)
        .filter(|m| !matches!(m.reading, LexicalReading::IntroducedName { .. }))
        .map(|m| m.reading.clone())
        .collect()
}

/// The title runs of a text, as analysis offers them for Introduced Names.
fn title_runs(text: &AnalyzedText) -> Vec<&str> {
    text.matches
        .iter()
        .filter_map(|m| match &m.reading {
            LexicalReading::IntroducedName { name } => Some(name.as_str()),
            _ => None,
        })
        .collect()
}

fn complete(lexicon: &Lexicon, text: &str) -> BTreeSet<LexicalReading> {
    at(&lexicon.analyze(text), 0, text.chars().count())
}

/// Each match spanning all of `text`, with the case it is written in.
fn complete_cased(lexicon: &Lexicon, text: &str) -> BTreeSet<(LexicalReading, Option<LetterCase>)> {
    let end = text.chars().count();
    lexicon
        .analyze(text)
        .matches
        .into_iter()
        .filter(|m| m.start == 0 && m.end == end)
        .filter(|m| !matches!(m.reading, LexicalReading::IntroducedName { .. }))
        .map(|m| (m.reading, m.case))
        .collect()
}

fn override_form(lexeme: &mut Lexeme, form: WordForm, surface: &str) {
    for declaration in &mut lexeme.forms {
        if declaration.form == form {
            declaration.surface = Some(surface.to_owned());
        }
    }
}

#[test]
fn counters_preserves_noun_and_correlated_finite_verb() {
    let lexicon =
        Lexicon::new([noun("n:counter", "counter"), verb("v:counter", "counter")]).unwrap();
    assert_eq!(
        complete(&lexicon, "counters"),
        BTreeSet::from([
            word("n:counter", WordForm::Plural, nominal(Number::Plural)),
            word(
                "v:counter",
                WordForm::Present,
                finite(Person::Third, Number::Singular, Tense::Present)
            ),
        ])
    );
}

#[test]
fn cast_preserves_plain_present_preterite_and_participle_alternatives() {
    let mut cast = verb("v:cast", "cast");
    override_form(&mut cast, WordForm::Preterite, "cast");
    override_form(&mut cast, WordForm::PastParticiple, "cast");
    let lexicon = Lexicon::new([cast]).unwrap();
    let readings = complete(&lexicon, "cast");

    for expected in [
        word("v:cast", WordForm::Plain, FeatureBundle::default()),
        word(
            "v:cast",
            WordForm::Present,
            finite(Person::First, Number::Singular, Tense::Present),
        ),
        word(
            "v:cast",
            WordForm::Preterite,
            finite(Person::Third, Number::Plural, Tense::Past),
        ),
        word("v:cast", WordForm::PastParticiple, nonfinite()),
    ] {
        assert!(readings.contains(&expected), "missing {expected:?}");
    }
    assert!(!readings.contains(&word(
        "v:cast",
        WordForm::Present,
        finite(Person::Third, Number::Singular, Tense::Present),
    )));
    assert_eq!(readings.len(), 13);
}

#[test]
fn declarations_roundtrip_with_overridden_forms() {
    let mut cast = verb("v:cast", "cast");
    override_form(&mut cast, WordForm::Preterite, "cast");
    override_form(&mut cast, WordForm::PastParticiple, "cast");

    let serialized = ron::to_string(&cast).unwrap();
    let restored: Lexeme = ron::from_str(&serialized).unwrap();
    assert_eq!(restored, cast);

    let lexicon = Lexicon::new([restored]).unwrap();
    let readings = complete(&lexicon, "cast");
    assert!(readings.contains(&word(
        "v:cast",
        WordForm::Preterite,
        finite(Person::Third, Number::Plural, Tense::Past),
    )));
    assert!(readings.contains(&word("v:cast", WordForm::PastParticiple, nonfinite(),)));
}

#[test]
fn one_or_two_targets_retains_each_word_and_numeral_reading() {
    let lexicon = Lexicon::new([
        noun("n:one", "one"),
        invariant("d:one", "one", Category::Determinative),
        invariant("c:or", "or", Category::Coordinator),
        invariant("d:target", "target", Category::Determinative),
        noun("n:creature", "creature"),
    ])
    .unwrap();
    let plain_one = BTreeSet::from([
        word("n:one", WordForm::Singular, nominal(Number::Singular)),
        word("d:one", WordForm::Invariant, FeatureBundle::default()),
        numeral(1, Numeral::Cardinal),
    ]);
    assert_eq!(complete(&lexicon, "one"), plain_one);
    let text = lexicon.analyze("One or two target creatures");
    assert_eq!(at(&text, 0, 3), plain_one);
    assert!(
        text.matches
            .iter()
            .filter(|m| m.start == 0)
            .filter(|m| !matches!(m.reading, LexicalReading::IntroducedName { .. }))
            .all(|m| m.case == Some(LetterCase::Capital))
    );
    assert_eq!(
        at(&text, 7, 10),
        BTreeSet::from([numeral(2, Numeral::Cardinal)])
    );
    assert_eq!(
        at(&text, 11, 17),
        BTreeSet::from([word(
            "d:target",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    assert_eq!(
        at(&text, 18, 27),
        BTreeSet::from([word(
            "n:creature",
            WordForm::Plural,
            nominal(Number::Plural)
        ),])
    );
    assert_eq!(text.unknown_words(), [] as [std::ops::Range<usize>; 0]);
    assert!(!text.matches.iter().any(|m| m.start == 0 && m.end > 3));
}

#[test]
fn numeric_prefixes_survive_longer_numeric_phrases() {
    let lexicon = Lexicon::new([noun("n:one", "one")]).unwrap();
    let text = lexicon.analyze("one hundred");
    assert_eq!(
        at(&text, 0, 3),
        BTreeSet::from([
            word("n:one", WordForm::Singular, nominal(Number::Singular)),
            numeral(1, Numeral::Cardinal),
        ])
    );
    assert_eq!(
        at(&text, 0, 11),
        BTreeSet::from([numeral(100, Numeral::Cardinal)])
    );
}

#[test]
fn initial_variants_do_not_case_fold_exact_catalog_entries() {
    let mut catalog = invariant("name:ThreeDog", "Three Dog", Category::Catalog);
    catalog.capitalization = Capitalization::Exact;
    catalog.source = Source {
        kind: SourceKind::Catalog,
        path: "tests/card-names.txt".to_owned(),
        owner: "Three Dog".to_owned(),
    };
    let lexicon = Lexicon::new([noun("n:counter", "counter"), catalog]).unwrap();
    assert_eq!(
        complete_cased(&lexicon, "Counters"),
        BTreeSet::from([(
            word("n:counter", WordForm::Plural, nominal(Number::Plural)),
            Some(LetterCase::Capital)
        )])
    );
    assert_eq!(
        complete_cased(&lexicon, "counters"),
        BTreeSet::from([(
            word("n:counter", WordForm::Plural, nominal(Number::Plural)),
            Some(LetterCase::Lower)
        )])
    );
    assert!(complete(&lexicon, "COUNTERS").is_empty());
    assert_eq!(
        complete_cased(&lexicon, "Three Dog"),
        BTreeSet::from([(
            word(
                "name:ThreeDog",
                WordForm::Invariant,
                FeatureBundle::default()
            ),
            None
        )])
    );
    assert!(complete(&lexicon, "three Dog").is_empty());
    assert!(complete(&lexicon, "Three dog").is_empty());
}

#[test]
fn default_morphology_handles_y_sibilants_e_and_ie() {
    let lexicon = Lexicon::new([
        noun("n:ability", "ability"),
        noun("n:box", "box"),
        verb("v:copy", "copy"),
        verb("v:watch", "watch"),
        verb("v:exile", "exile"),
        verb("v:die", "die"),
    ])
    .unwrap();
    for (surface, reading) in [
        (
            "abilities",
            word("n:ability", WordForm::Plural, nominal(Number::Plural)),
        ),
        (
            "boxes",
            word("n:box", WordForm::Plural, nominal(Number::Plural)),
        ),
        (
            "copies",
            word(
                "v:copy",
                WordForm::Present,
                finite(Person::Third, Number::Singular, Tense::Present),
            ),
        ),
        (
            "watches",
            word(
                "v:watch",
                WordForm::Present,
                finite(Person::Third, Number::Singular, Tense::Present),
            ),
        ),
        (
            "copying",
            word("v:copy", WordForm::GerundParticiple, nonfinite()),
        ),
        (
            "exiling",
            word("v:exile", WordForm::GerundParticiple, nonfinite()),
        ),
        (
            "dying",
            word("v:die", WordForm::GerundParticiple, nonfinite()),
        ),
    ] {
        assert_eq!(complete(&lexicon, surface), BTreeSet::from([reading]));
    }
    for (surface, id) in [("copied", "v:copy"), ("exiled", "v:exile")] {
        assert_eq!(complete(&lexicon, surface), past_readings(id));
    }
    for malformed in [
        "abilitys", "boxs", "copys", "watchs", "copyed", "exileed", "exileing", "dieing",
    ] {
        assert!(complete(&lexicon, malformed).is_empty(), "{malformed}");
    }
}

fn past_readings(id: &str) -> BTreeSet<LexicalReading> {
    let mut readings: BTreeSet<_> = [
        (Person::First, Number::Singular),
        (Person::Second, Number::Singular),
        (Person::Third, Number::Singular),
        (Person::First, Number::Plural),
        (Person::Second, Number::Plural),
        (Person::Third, Number::Plural),
    ]
    .into_iter()
    .map(|(person, number)| word(id, WordForm::Preterite, finite(person, number, Tense::Past)))
    .collect();
    readings.insert(word(id, WordForm::PastParticiple, nonfinite()));
    readings
}

#[test]
fn overrides_replace_defaults() {
    let mut tap = verb("v:tap", "tap");
    override_form(&mut tap, WordForm::GerundParticiple, "tapping");
    let mut draw = verb("v:draw", "draw");
    override_form(&mut draw, WordForm::Preterite, "drew");
    override_form(&mut draw, WordForm::PastParticiple, "drawn");
    let lexicon = Lexicon::new([tap, draw]).unwrap();
    assert_eq!(
        complete(&lexicon, "tapping"),
        BTreeSet::from([word("v:tap", WordForm::GerundParticiple, nonfinite()),])
    );
    assert_eq!(
        complete(&lexicon, "drawn"),
        BTreeSet::from([word("v:draw", WordForm::PastParticiple, nonfinite()),])
    );
    let expected_drew: BTreeSet<_> = past_readings("v:draw").into_iter().filter(|reading| {
        matches!(reading, LexicalReading::Word(value) if value.form == WordForm::Preterite)
    }).collect();
    assert_eq!(complete(&lexicon, "drew"), expected_drew);
    for obsolete in ["taping", "drawed"] {
        assert!(complete(&lexicon, obsolete).is_empty(), "{obsolete}");
    }
}

#[test]
fn be_agreement_keeps_complete_bundles_instead_of_crossing_features() {
    let mut be = invariant("v:be", "be", Category::Verb);
    be.forms = [
        ("am", Person::First, Number::Singular),
        ("is", Person::Third, Number::Singular),
        ("are", Person::Second, Number::Singular),
        ("are", Person::First, Number::Plural),
        ("are", Person::Second, Number::Plural),
        ("are", Person::Third, Number::Plural),
    ]
    .into_iter()
    .map(|(surface, person, number)| FormDeclaration {
        form: WordForm::Present,
        features: finite(person, number, Tense::Present),
        surface: Some(surface.to_owned()),
    })
    .collect();
    let lexicon = Lexicon::new([be]).unwrap();
    assert_eq!(
        complete(&lexicon, "am"),
        BTreeSet::from([word(
            "v:be",
            WordForm::Present,
            finite(Person::First, Number::Singular, Tense::Present)
        ),])
    );
    assert_eq!(
        complete(&lexicon, "is"),
        BTreeSet::from([word(
            "v:be",
            WordForm::Present,
            finite(Person::Third, Number::Singular, Tense::Present)
        ),])
    );
    assert_eq!(
        complete(&lexicon, "are"),
        BTreeSet::from([
            word(
                "v:be",
                WordForm::Present,
                finite(Person::Second, Number::Singular, Tense::Present)
            ),
            word(
                "v:be",
                WordForm::Present,
                finite(Person::First, Number::Plural, Tense::Present)
            ),
            word(
                "v:be",
                WordForm::Present,
                finite(Person::Second, Number::Plural, Tense::Present)
            ),
            word(
                "v:be",
                WordForm::Present,
                finite(Person::Third, Number::Plural, Tense::Present)
            ),
        ])
    );
    assert!(complete(&lexicon, "bes").is_empty());
}

#[test]
fn multiword_and_bound_forms_keep_overlapping_analyses() {
    let mut swamp = invariant("type:Swamp", "Swamp", Category::Catalog);
    swamp.capitalization = Capitalization::Exact;
    swamp.binding = Binding::Prefix;
    let mut walk = invariant("bound:walk", "walk", Category::Keyword);
    walk.binding = Binding::Suffix;
    let lexicon = Lexicon::new([
        multiword("term:mana-value", "mana value", Category::Noun),
        invariant("n:mana", "mana", Category::Noun),
        noun("n:value", "value"),
        swamp,
        walk,
        invariant("keyword:Swampwalk", "Swampwalk", Category::Keyword),
    ])
    .unwrap();
    let phrase = lexicon.analyze("mana value");
    assert_eq!(
        at(&phrase, 0, 10),
        BTreeSet::from([word(
            "term:mana-value",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    assert_eq!(
        at(&phrase, 0, 4),
        BTreeSet::from([word(
            "n:mana",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    assert_eq!(
        at(&phrase, 5, 10),
        BTreeSet::from([word(
            "n:value",
            WordForm::Singular,
            nominal(Number::Singular)
        ),])
    );
    let bound = lexicon.analyze("Swampwalk");
    for (start, end, id) in [
        (0, 5, "type:Swamp"),
        (5, 9, "bound:walk"),
        (0, 9, "keyword:Swampwalk"),
    ] {
        assert_eq!(
            at(&bound, start, end),
            BTreeSet::from([word(id, WordForm::Invariant, FeatureBundle::default()),])
        );
    }
    assert_eq!(bound.unknown_words(), [] as [std::ops::Range<usize>; 0]);
    assert!(complete(&lexicon, "mana  value").is_empty());
    assert!(!lexicon.analyze("Swampwalker").matches.iter().any(|m| {
        m.reading == word("bound:walk", WordForm::Invariant, FeatureBundle::default())
    }));
}

#[test]
fn unicode_positions_and_normalization_preserve_distinct_provenance() {
    let lexeme = invariant("name:Ae", "Æther", Category::Catalog);
    let expected_source = lexeme.source.clone();
    let lexicon = Lexicon::new([lexeme, noun("n:counter", "counter")]).unwrap();
    let text = lexicon.analyze_source("Æther\u{a0}counter", Some("Æther counter"));
    assert_eq!(text.raw, "Æther\u{a0}counter");
    assert_eq!(text.normalized.as_deref(), Some("Æther counter"));
    assert_eq!(text.tokens.iter().collect::<String>(), "Æther counter");
    assert_eq!(text.byte_range(6, 13), Some(7..14));
    assert_eq!(&text.text()[7..14], "counter");
    assert_eq!(
        at(&text, 6, 13),
        BTreeSet::from([word(
            "n:counter",
            WordForm::Singular,
            nominal(Number::Singular)
        ),])
    );
    assert_eq!(
        at(&text, 0, 5),
        BTreeSet::from([word(
            "name:Ae",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    let name = text
        .matches
        .iter()
        .find(|m| m.start == 0 && m.end == 5)
        .unwrap();
    let LexicalReading::Word(value) = &name.reading else {
        panic!("expected declared name");
    };
    assert_eq!(lexicon.lexemes()[&value.lexeme].source, expected_source);
    assert_eq!(text.byte_range(14, 15), None);
    assert_eq!(text.byte_range(8, 2), None);
    assert_eq!(lexicon.analyze("Æther counter").normalized, None);
}

#[test]
fn undeclared_words_and_parts_of_speech_are_not_guessed() {
    let lexicon = Lexicon::new([noun("n:counter", "counter")]).unwrap();
    let text = lexicon.analyze("counters countering unknown");
    assert_eq!(
        at(&text, 0, 8),
        BTreeSet::from([word("n:counter", WordForm::Plural, nominal(Number::Plural)),])
    );
    assert_eq!(text.unknown_words(), vec![9..19, 20..27]);
    assert!(!text.matches.iter().any(|m| m.start >= 9));
}

#[test]
fn independently_constructed_readings_roundtrip_through_realization() {
    let lexicon = Lexicon::new([noun("n:counter", "counter")]).unwrap();
    let lower = LetterCase::Lower;
    let capital = LetterCase::Capital;
    for (reading, case, expected_surface, written) in [
        (
            word("n:counter", WordForm::Plural, nominal(Number::Plural)),
            lower,
            "counters",
            Some(lower),
        ),
        (
            word("n:counter", WordForm::Singular, nominal(Number::Singular)),
            capital,
            "Counter",
            Some(capital),
        ),
        (
            numeral(100, Numeral::Cardinal),
            lower,
            "one hundred",
            Some(lower),
        ),
        (numeral(2, Numeral::Cardinal), capital, "Two", Some(capital)),
        (
            numeral(1_001, Numeral::Cardinal),
            lower,
            "one thousand, one",
            Some(lower),
        ),
        (
            numeral(-2, Numeral::Ordinal),
            lower,
            "second to last",
            Some(lower),
        ),
        (
            numeral(-1_000, Numeral::Arabic(true)),
            lower,
            "-1,000",
            None,
        ),
        (numeral(4, Numeral::Roman), lower, "IV", None),
        // A numeral with one spelling keeps it where a capital is called for.
        (numeral(4, Numeral::Roman), capital, "IV", None),
    ] {
        assert_eq!(
            lexicon.realize_in(&reading, case).unwrap(),
            expected_surface
        );
        assert_eq!(lexicon.takes_capital(&reading).unwrap(), written.is_some());
        // A match records a case exactly where the reading has two spellings.
        let other = match written {
            Some(_) => None,
            None => Some(lower),
        };
        assert!(lexicon.realize_written(&reading, other).is_err());
        assert!(
            complete_cased(&lexicon, expected_surface).contains(&(reading.clone(), written)),
            "{reading:?} from {expected_surface}"
        );
    }
}

#[test]
fn every_returned_reading_realizes_its_exact_source_occurrence() {
    let lexicon = Lexicon::new([
        noun("n:counter", "counter"),
        verb("v:counter", "counter"),
        multiword("term:mana-value", "mana value", Category::Noun),
    ])
    .unwrap();
    let text = lexicon.analyze("Counters: one hundred mana value; countered.");
    assert!(
        text.matches
            .iter()
            .any(|m| m.reading == numeral(100, Numeral::Cardinal))
    );
    for found in &text.matches {
        let range = text.byte_range(found.start, found.end).unwrap();
        assert_eq!(
            lexicon.realize_written(&found.reading, found.case).unwrap(),
            &text.text()[range]
        );
    }
}

#[test]
fn invalid_constructed_values_are_rejected() {
    let lexicon = Lexicon::new([noun("n:counter", "counter")]).unwrap();
    for reading in [
        word("n:counter", WordForm::Plural, nominal(Number::Singular)),
        word("v:counter", WordForm::Plain, FeatureBundle::default()),
        numeral(4_000, Numeral::Roman),
        numeral(2, Numeral::Arabic(true)),
    ] {
        assert!(
            matches!(
                lexicon.realize(&reading),
                Err(LexicalError::UnlicensedValue)
            ),
            "{reading:?}"
        );
    }
    assert_eq!(
        lexicon.realize(&numeral(i32::MAX, Numeral::Roman)).unwrap(),
        "infinitum"
    );
}

#[test]
fn normalized_numerals_roundtrip_without_erasing_visible_notation() {
    let lexicon = Lexicon::new([]).unwrap();
    for value in [i32::MIN, -1000, -999, -1, 0, 1, 999, 1000, i32::MAX] {
        for requested in [Numeral::Arabic(false), Numeral::Arabic(true)] {
            let notation = requested.canonical_notation(value);
            let reading = numeral(value, notation);
            let surface = requested.try_format(value).unwrap();
            assert_eq!(lexicon.realize(&reading).unwrap(), surface);
            assert_eq!(complete(&lexicon, &surface), BTreeSet::from([reading]));
            assert_eq!(notation.canonical_notation(value), notation);
        }
        assert_eq!(
            Numeral::Arabic(true).canonical_notation(value) == Numeral::Arabic(false),
            value.unsigned_abs() < 1000,
        );
    }
    for (notation, surface) in [(Numeral::Cardinal, "one"), (Numeral::Roman, "I")] {
        let value = numeral(1, notation);
        assert_eq!(lexicon.realize(&value).unwrap(), surface);
        assert!(complete(&lexicon, surface).contains(&value));
    }
}

#[test]
fn invariant_categories_reject_inapplicable_nominal_features() {
    let features = FeatureBundle {
        number: Some(Number::Plural),
        person: Some(Person::First),
        case: Some(Case::Nominative),
        ..FeatureBundle::default()
    };
    let mut pronoun = invariant("pro:we", "we", Category::Pronoun);
    pronoun.forms[0].features = features.clone();
    let lexicon = Lexicon::new([pronoun]).unwrap();
    assert_eq!(
        complete(&lexicon, "we"),
        BTreeSet::from([word("pro:we", WordForm::Invariant, features.clone()),])
    );
    let mut preposition = invariant("p:with", "with", Category::Preposition);
    preposition.forms[0].features = features;
    assert!(matches!(Lexicon::new([preposition]),
        Err(LexicalError::Declaration { lexeme, .. }) if lexeme == "p:with"));
}

#[test]
fn combining_marks_do_not_create_false_free_word_boundaries() {
    let lexicon = Lexicon::new([invariant("n:e", "e", Category::Noun)]).unwrap();
    let text = lexicon.analyze("e\u{301}");
    assert_eq!(text.words(), vec![0..2]);
    assert_eq!(text.unknown_words(), vec![0..2]);
    assert_eq!(text.byte_range(0, 2), Some(0..3));
    assert_eq!(text.matches, [] as [sylex_lexicon::LexicalMatch; 0]);
    assert_eq!(
        complete(&lexicon, "e"),
        BTreeSet::from([word("n:e", WordForm::Invariant, FeatureBundle::default()),])
    );
}

#[test]
fn crossing_bound_matches_do_not_hide_unsegmentable_words() {
    let bound = |id: &str, surface: &str| {
        let mut lexeme = invariant(id, surface, Category::Noun);
        lexeme.binding = Binding::Bound;
        lexeme
    };
    let crossing = Lexicon::new([bound("piece:ab", "ab"), bound("piece:bc", "bc")]).unwrap();
    let text = crossing.analyze("abc");
    assert_eq!(
        at(&text, 0, 2),
        BTreeSet::from([word(
            "piece:ab",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    assert_eq!(
        at(&text, 1, 3),
        BTreeSet::from([word(
            "piece:bc",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    assert_eq!(text.unknown_words(), vec![0..3]);
    let adjacent = Lexicon::new([bound("piece:a", "a"), bound("piece:bc", "bc")]).unwrap();
    let text = adjacent.analyze("abc");
    assert_eq!(
        at(&text, 0, 1),
        BTreeSet::from([word(
            "piece:a",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    assert_eq!(
        at(&text, 1, 3),
        BTreeSet::from([word(
            "piece:bc",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    assert_eq!(text.unknown_words(), [] as [std::ops::Range<usize>; 0]);
}

/// The ability words are a listed inventory [CR#207.2c]; flavor words are
/// listed nowhere [CR#207.2d], so an italic head no declaration spells is one
/// and the run itself is the label.
#[test]
fn an_italic_run_no_declaration_spells_is_a_flavor_word() {
    let mut battalion = invariant(
        "lexeme:ability_word/battalion",
        "Battalion",
        Category::Keyword,
    );
    battalion
        .properties
        .features
        .insert("LabelKind".into(), "AbilityWord".into());
    let lexicon =
        Lexicon::new([battalion, verb("lexeme:keyword_action/attack", "attack")]).unwrap();

    assert_eq!(
        lexicon.analyze_italic_head("Battalion"),
        ItalicHead::AbilityWord(LexicalValue {
            lexeme: "lexeme:ability_word/battalion".into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
        }),
        "a declared ability word is not a flavor word",
    );

    for run in ["Kowabunga", "Blade Beam", "A Test of Your Reflexes"] {
        assert_eq!(
            lexicon.analyze_italic_head(run),
            ItalicHead::FlavorWord {
                label: run.to_owned()
            },
            "{run:?} is a flavor word whose label is the run verbatim",
        );
    }

    assert_eq!(
        lexicon.analyze_italic_head("Attack"),
        ItalicHead::FlavorWord {
            label: "Attack".to_owned()
        },
        "only a listed ability word displaces a flavor word; a declared verb does not",
    );
}

#[test]
fn declared_alphabetic_slash_compounds_preserve_identity_without_guessing_parts() {
    let lexicon = Lexicon::new([
        invariant("coordinator:inclusive", "and/or", Category::Coordinator),
        invariant("coordinator:translated", "oui/non", Category::Coordinator),
    ])
    .unwrap();
    for (owner, surface) in [
        ("coordinator:inclusive", "and/or"),
        ("coordinator:translated", "oui/non"),
    ] {
        let expected = word(owner, WordForm::Invariant, FeatureBundle::default());
        assert_eq!(lexicon.realize(&expected).unwrap(), surface);
        assert_eq!(complete(&lexicon, surface), BTreeSet::from([expected]));
    }
    for surface in [
        "and", "or", "and / or", "and/ or", "and /or", "and/nor", "and//or",
    ] {
        assert!(complete(&lexicon, surface).is_empty(), "{surface}");
    }
    for surface in ["/word", "word/", "word//word", "1/2", "word/2", "2/word"] {
        for binding in [Binding::Free, Binding::Bound] {
            let mut entry = invariant("invalid", surface, Category::Coordinator);
            entry.binding = binding;
            assert!(Lexicon::new([entry]).is_err(), "{surface} {binding:?}");
        }
        let mut entry = invariant("invalid-variant", "ordinary", Category::Coordinator);
        entry.forms[0].surface = Some(surface.into());
        assert!(Lexicon::new([entry]).is_err(), "override {surface}");
    }
    let notation = Lexicon::new([invariant("notation", "1/2", Category::Symbol)]).unwrap();
    let expected = word("notation", WordForm::Invariant, FeatureBundle::default());
    assert_eq!(notation.realize(&expected).unwrap(), "1/2");
    assert_eq!(complete(&notation, "1/2"), BTreeSet::from([expected]));
}

#[test]
fn ordinary_recipes_reject_separator_and_quote_boundaries() {
    for surface in [
        "two words",
        "two\nwords",
        "word\t",
        "a,b",
        "\"word\"",
        "“word”",
        "'word'",
        "word;next",
    ] {
        assert!(
            Lexicon::new([invariant("word", surface, Category::Noun)]).is_err(),
            "{surface:?}"
        );
        let mut lexeme = noun("noun", "card");
        lexeme.forms[0].surface = Some(surface.into());
        assert!(Lexicon::new([lexeme]).is_err(), "override {surface:?}");
    }
    for surface in [
        "two\nwords",
        "two\twords",
        "two  words",
        "two \"words\"",
        "two, words",
    ] {
        assert!(
            Lexicon::new([multiword("phrase", surface, Category::Noun)]).is_err(),
            "{surface:?}"
        );
    }
    let lexicon = Lexicon::new([
        multiword("phrase", "two words", Category::Noun),
        invariant("catalog", "A, \"Name\"", Category::Catalog),
        invariant("symbol", "{W/U}", Category::Symbol),
        invariant("contraction", "y'all", Category::Pronoun),
    ])
    .unwrap();
    for surface in ["two words", "A, \"Name\"", "{W/U}", "y'all"] {
        assert!(!complete(&lexicon, surface).is_empty());
    }
    let mut disguised = invariant("noun", "two words", Category::Noun);
    disguised.surface_structure = SurfaceStructure::Opaque;
    assert!(Lexicon::new([disguised]).is_err());
}

#[test]
fn bound_forms_expose_adjacent_hosts_without_guessing_unknown_hosts() {
    let mut prefix = invariant("prefix:non", "non", Category::Affix);
    prefix.binding = Binding::Prefix;
    let mut suffix = invariant("suffix:walk", "walk", Category::Keyword);
    suffix.binding = Binding::Suffix;
    let lexicon = Lexicon::new([prefix, suffix, noun("noun:land", "land")]).unwrap();
    for text in ["nonland", "landwalk", "nonlandwalk", "nonnonland"] {
        let analyzed = lexicon.analyze(text);
        assert!(analyzed.unknown_words().is_empty(), "{text}");
        let start = text.find("land").unwrap();
        assert_eq!(
            at(&analyzed, start, start + 4),
            BTreeSet::from([word(
                "noun:land",
                WordForm::Singular,
                nominal(Number::Singular)
            )])
        );
    }
    for text in ["unknownland", "landunknown", "nonunknown", "unknownwalk"] {
        assert!(!lexicon.analyze(text).unknown_words().is_empty(), "{text}");
    }
    assert_eq!(
        lexicon.analyze("outlandish").matches,
        [] as [sylex_lexicon::LexicalMatch; 0]
    );
}

#[test]
fn plain_forms_do_not_assign_finiteness_but_participles_do() {
    let lexicon = Lexicon::new([verb("v:draw", "draw")]).unwrap();
    let plain = word("v:draw", WordForm::Plain, FeatureBundle::default());
    assert!(complete(&lexicon, "draw").contains(&plain));
    assert_eq!(lexicon.realize(&plain).unwrap(), "draw");
    for finiteness in [Finiteness::Finite, Finiteness::Nonfinite] {
        let falsely_classified = word(
            "v:draw",
            WordForm::Plain,
            FeatureBundle {
                finiteness: Some(finiteness),
                ..FeatureBundle::default()
            },
        );
        assert!(matches!(
            lexicon.realize(&falsely_classified),
            Err(LexicalError::UnlicensedValue)
        ));
        let mut declaration = verb("v:draw", "draw");
        declaration
            .forms
            .iter_mut()
            .find(|slot| slot.form == WordForm::Plain)
            .unwrap()
            .features
            .finiteness = Some(finiteness);
        assert!(matches!(
            Lexicon::new([declaration]),
            Err(LexicalError::Declaration { .. })
        ));
    }
    assert!(complete(&lexicon, "drawing").contains(&word(
        "v:draw",
        WordForm::GerundParticiple,
        nonfinite()
    )));
    assert!(complete(&lexicon, "drawed").contains(&word(
        "v:draw",
        WordForm::PastParticiple,
        nonfinite()
    )));
}

#[test]
fn flavor_heads_are_lossless_open_labels_at_line_starts() {
    // Title casing leaves function words lowercase (*of the*).
    let lexicon = Lexicon::new([
        invariant("prep:of", "of", Category::Preposition),
        invariant("det:the", "the", Category::Determinative),
    ])
    .unwrap();
    for (label, body) in [
        // Astrologian's Planisphere and Blue Mage's Cane.
        ("Diana", "Equip {2}"),
        ("Spirit of the Whalaqee", "Equip {2}"),
        // Deliberately unseen label establishes the open lexical class.
        ("An Unlisted Label Ω!", "Equip {2}"),
    ] {
        let text = format!("{label} — {body}");
        let expected = LexicalReading::FlavorWord {
            label: label.into(),
        };
        let end = label.chars().count() + 2;
        assert_eq!(
            at(&lexicon.analyze(&text), 0, end),
            BTreeSet::from([expected.clone()])
        );
        assert_eq!(lexicon.realize(&expected).unwrap(), format!("{label} —"));
        assert!(!lexicon.takes_capital(&expected).unwrap());
        assert_eq!(lexicon.edges(&expected).unwrap().left.onset, None);
        let prefixed = format!("Trample\n{text}");
        assert_eq!(
            at(&lexicon.analyze(&prefixed), 8, 8 + end),
            BTreeSet::from([expected])
        );
    }
    for text in [
        " — Equip {2}",
        "Diana— Equip {2}",
        "Diana - Equip {2}",
        "Diana\t — Equip {2}",
        "Diana\n — Equip {2}",
    ] {
        assert!(
            !lexicon
                .analyze(text)
                .matches
                .iter()
                .any(|found| matches!(found.reading, LexicalReading::FlavorWord { .. })),
            "{text:?}"
        );
    }
    let analyzed = lexicon.analyze("prefix Diana — Equip {2}");
    assert!(!analyzed.matches.iter().any(
        |found| found.start == 7 && matches!(found.reading, LexicalReading::FlavorWord { .. })
    ));
    for label in ["", "Diana\nOther", "Diana\tOther", "Diana — Other"] {
        let reading = LexicalReading::FlavorWord {
            label: label.into(),
        };
        assert!(lexicon.realize(&reading).is_err());
        assert!(lexicon.edges(&reading).is_err());
    }
}

/// A Flavor Word has natural title casing (`docs/oracle-style-guide.md` §14,
/// Ability words and flavor words): a label with a lowercase word that is
/// not a function word is a clause before an em dash, never a Flavor Word.
#[test]
fn a_label_without_title_casing_is_no_flavor_word() {
    let lexicon = Lexicon::new([
        invariant("pro:it", "it", Category::Pronoun),
        invariant("det:a", "a", Category::Determinative),
    ])
    .unwrap();
    let flavor = |text: &str| {
        lexicon
            .analyze(text)
            .matches
            .iter()
            .any(|found| matches!(found.reading, LexicalReading::FlavorWord { .. }))
    };
    // Company Commander's flavor word.
    assert!(flavor("Bring it Down! — Draw a card."));
    // Ensnared by the Mara: the keyword action's options follow the dash.
    assert!(!flavor(
        "Each opponent faces a villainous choice — They discard a card, or you draw a card."
    ));
    for label in [
        "Each opponent faces a villainous choice",
        "Uncharted marvel",
    ] {
        let reading = LexicalReading::FlavorWord {
            label: label.into(),
        };
        assert!(lexicon.realize(&reading).is_err(), "{label}");
    }
}

/// A registered ability word [CR#207.2c] or keyword [CR#702.1] is never
/// also emitted as an open flavor-word label [CR#207.2d].
#[test]
fn a_registered_label_is_never_also_a_flavor_word() {
    let mut channel = invariant("lexeme:ability_word/channel", "Channel", Category::Keyword);
    channel
        .properties
        .features
        .insert("LabelKind".into(), "AbilityWord".into());
    let equip = invariant("lexeme:keyword_ability/equip", "Equip", Category::Keyword);
    let lexicon = Lexicon::new([channel, equip]).unwrap();
    // Mnemonic Sphere's ability head.
    for text in [
        "Channel — {U}, Discard this card: Draw a card.",
        "Equip — {U}, Discard this card: Draw a card.",
    ] {
        let analyzed = lexicon.analyze(text);
        assert!(
            !analyzed
                .matches
                .iter()
                .any(|found| matches!(found.reading, LexicalReading::FlavorWord { .. })),
            "{text:?}"
        );
        assert!(
            at(&analyzed, 0, if text.starts_with('C') { 7 } else { 5 })
                .iter()
                .any(|reading| matches!(reading, LexicalReading::Word(_)))
        );
    }
    assert!(matches!(
        lexicon.analyze_italic_head("Channel"),
        ItalicHead::AbilityWord(_)
    ));
    assert!(matches!(
        lexicon.analyze_italic_head("Equip"),
        ItalicHead::Keyword(_)
    ));

    assert_eq!(
        lexicon
            .analyze("Diana — Equip {2}")
            .matches
            .iter()
            .find_map(|found| match &found.reading {
                LexicalReading::FlavorWord { label } => Some(label.as_str()),
                _ => None,
            }),
        Some("Diana"),
        "an unregistered label is still a flavor word",
    );
    for label in ["Channel", "Equip"] {
        let reading = LexicalReading::FlavorWord {
            label: label.into(),
        };
        assert!(lexicon.realize(&reading).is_err(), "{label}");
        assert!(lexicon.edges(&reading).is_err(), "{label}");
    }
}

/// Citadel Siege: *• Khans — ...*. No Lexeme declares an Anchor Word
/// [CR#614.12c]: it stands where the Card Scope holds it, and there the
/// label is never a Flavor Word; elsewhere the label is an open Flavor Word
/// and no Anchor Word.
#[test]
fn an_anchor_word_stands_where_the_scope_holds_it_and_a_flavor_word_elsewhere() {
    let lexicon = Lexicon::new([noun("anchor/card", "card")]).unwrap();
    let text = "• Khans — At the beginning of combat on your turn, draw a card.";
    assert_eq!(sylex_lexicon::bulleted_label(text), Some("Khans"));
    assert_eq!(sylex_lexicon::bulleted_label("Khans — Draw a card."), None);
    let anchor = LexicalReading::AnchorWord {
        word: "Khans".into(),
    };
    let flavor = LexicalReading::FlavorWord {
        label: "Khans".into(),
    };
    let mut on = CardScope::full_names(["Citadel Siege"]);
    on.insert_anchor_word("Khans");
    let off = CardScope::full_names(["Citadel Siege"]);
    let readings = |scope: &CardScope| {
        let analyzed = lexicon.analyze_source_scoped(text, None, scope);
        (
            at(&analyzed, 2, 7).contains(&anchor),
            analyzed.matches.iter().any(|found| found.reading == flavor),
        )
    };
    assert_eq!(readings(&on), (true, false));
    assert_eq!(readings(&off), (false, true));
    assert!(lexicon.stands_in(&anchor, &on));
    assert!(!lexicon.stands_in(&flavor, &on));
    assert!(!lexicon.stands_in(&anchor, &off));
    assert!(lexicon.stands_in(&flavor, &off));
    // It is spelled as the word, and found where the text offers it too.
    assert_eq!(lexicon.realize(&anchor).unwrap(), "Khans");
    assert!(lexicon.edges(&anchor).is_ok());
    let offer = "As this enchantment enters, choose Khans or Dragons.";
    let analyzed = lexicon.analyze_source_scoped(offer, None, &on);
    assert!(at(&analyzed, 35, 40).contains(&anchor));
    assert!(analyzed.unknown_words().iter().all(|word| word.start != 35));
    // Only whole words spell it.
    let inside = lexicon.analyze_source_scoped("Khanste", None, &on);
    assert!(inside.matches.iter().all(|found| found.reading != anchor));
    for word in ["", " Khans", "Khans —", "{G}"] {
        let reading = LexicalReading::AnchorWord { word: word.into() };
        assert!(lexicon.realize(&reading).is_err(), "{word:?}");
    }
}

#[test]
fn lexical_ids_are_interned_across_declarations_values_and_wire_formats() {
    fn copied<T: Copy>(value: T) -> T {
        value
    }
    let declared = noun("lexical-id-wire/card", "card");
    let id = copied(declared.id);
    let json = serde_json::to_value(&declared).unwrap();
    assert_eq!(json["id"], "lexical-id-wire/card");
    let restored: Lexeme = serde_json::from_value(json.clone()).unwrap();
    assert_eq!(restored, declared);
    assert!(std::ptr::eq(restored.id.as_str(), id.as_str()));
    assert_eq!(serde_json::to_value(&restored).unwrap(), json);
    let ron = ron::to_string(&declared).unwrap();
    let restored: Lexeme = ron::from_str(&ron).unwrap();
    assert_eq!(restored, declared);
    assert!(std::ptr::eq(restored.id.as_str(), id.as_str()));

    let lexicon = Lexicon::new([declared.clone()]).unwrap();
    assert_eq!(lexicon.lexemes()["lexical-id-wire/card"], declared);
    let analyzed = lexicon.analyze("card");
    let LexicalReading::Word(value) = &analyzed.matches[0].reading else {
        panic!("declared word")
    };
    assert_eq!(value.lexeme, id);
    assert!(std::ptr::eq(value.lexeme.as_str(), id.as_str()));
    let json = serde_json::to_value(&analyzed.matches[0].reading).unwrap();
    assert_eq!(json["Word"]["lexeme"], "lexical-id-wire/card");
    let restored: LexicalReading = serde_json::from_value(json).unwrap();
    assert_eq!(restored, analyzed.matches[0].reading);
    assert_eq!(lexicon.realize(&restored).unwrap(), "card");
    let LexicalReading::Word(value) = restored else { panic!("declared word") };
    assert!(std::ptr::eq(value.lexeme.as_str(), id.as_str()));

    let absent = "lexical-id-lookup-never-interned";
    assert!(LexemeId::get(absent).is_none());
    assert!(!lexicon.lexemes().contains_key(absent));
    assert_eq!(
        lexicon.analyze(absent).matches,
        [] as [sylex_lexicon::LexicalMatch; 0]
    );
    assert!(LexemeId::get(absent).is_none());
    assert!(matches!(
        Lexicon::new([declared.clone(), declared]),
        Err(LexicalError::Declaration { .. })
    ));
}

#[test]
fn an_own_name_is_admitted_only_where_its_scope_holds_its_form_and_realizes_alone() {
    // Chandra, Pyromaster's own text shortens its name to *Chandra*
    // [CR#201.5c].
    let name = "Chandra, Pyromaster";
    let card = Lexeme::card_name(name, source("name:Chandra, Pyromaster"));
    assert_eq!(card.id, sylex_lexicon::card_name_id(name));
    let decoded: Lexeme = ron::from_str(&ron::to_string(&card).unwrap()).unwrap();
    assert_eq!(decoded, card);
    let lexicon = Lexicon::new([decoded, noun("subtype:Chandra", "Chandra")]).unwrap();
    let own = |form| LexicalReading::OwnName {
        name: name.to_owned(),
        form,
    };
    let shortened = own(NameForm::Shortened);
    let full = own(NameForm::Full);
    // Each form is spelled from the name alone, whatever the lexicon holds.
    assert_eq!(lexicon.realize(&shortened).unwrap(), "Chandra");
    assert_eq!(lexicon.realize(&full).unwrap(), name);
    let other = word(
        card.id.as_str(),
        WordForm::Invariant,
        FeatureBundle::default(),
    );
    let every_form = CardScope::every_form([name]);
    let full_only = CardScope::full_names([name]);
    let elsewhere = CardScope::full_names(["Dark Depths"]);
    let named = |text: &str, scope: &CardScope| -> Vec<LexicalReading> {
        let input = lexicon.analyze_source_scoped("raw metadata", Some(text), scope);
        assert_eq!(input.raw, "raw metadata");
        assert_eq!(input.text(), text);
        // The subtype spelled like the Shortened Name stands on every card.
        assert!(input.matches.iter().any(
            |m| matches!(&m.reading, LexicalReading::Word(v) if v.lexeme == "subtype:Chandra")
        ));
        input
            .matches
            .iter()
            .filter(|m| m.reading == other || matches!(m.reading, LexicalReading::OwnName { .. }))
            .map(|m| m.reading.clone())
            .collect()
    };
    // The Shortened form stands only where the scope holds that form.
    assert_eq!(
        named("Chandra", &every_form),
        std::slice::from_ref(&shortened)
    );
    for scope in [&full_only, &elsewhere, &CardScope::default()] {
        assert_eq!(named("Chandra", scope), []);
        assert!(!lexicon.stands_in(&shortened, scope));
    }
    // The full name is the card's own where the scope holds it, and
    // another card's, the Lexeme's, everywhere else.
    for scope in [&every_form, &full_only] {
        let found = named(name, scope);
        assert!(
            found.contains(&full) && !found.contains(&other),
            "{found:?}"
        );
        assert!(lexicon.stands_in(&full, scope) && !lexicon.stands_in(&other, scope));
    }
    for scope in [&elsewhere, &CardScope::default()] {
        assert_eq!(named(name, scope), std::slice::from_ref(&other));
        assert!(!lexicon.stands_in(&full, scope) && lexicon.stands_in(&other, scope));
    }
}

#[test]
fn a_scope_may_hold_a_name_no_lexeme_declares() {
    // No card is named *Abuelo the Brave*; a text may be read as if on one.
    let lexicon = Lexicon::new([noun("noun:card", "card")]).unwrap();
    let name = "Abuelo the Brave";
    let scope = CardScope::every_form([name]);
    let analyzed = lexicon.analyze_source_scoped("Abuelo the Brave, Abuelo", None, &scope);
    let own = |form| LexicalReading::OwnName {
        name: name.to_owned(),
        form,
    };
    assert_eq!(at(&analyzed, 0, 16), BTreeSet::from([own(NameForm::Full)]));
    assert_eq!(
        at(&analyzed, 18, 24),
        BTreeSet::from([own(NameForm::Shortened)])
    );
    assert_eq!(analyzed.unknown_words(), []);
    // The full name written out is the name: its first word is no
    // Shortened Name there.
    assert_eq!(at(&analyzed, 0, 6), BTreeSet::new());
    // Inside a longer word the spelling is no name.
    let analyzed = lexicon.analyze_source_scoped("Abuelos", None, &scope);
    assert_eq!(at(&analyzed, 0, 6), BTreeSet::new());
    assert_eq!(title_runs(&analyzed), ["Abuelos"]);
    // A name with no shortened spelling has no Shortened form to realize.
    let progenitus = LexicalReading::OwnName {
        name: "Progenitus".to_owned(),
        form: NameForm::Shortened,
    };
    assert!(lexicon.realize(&progenitus).is_err());
    assert!(lexicon.edges(&progenitus).is_err());
}

#[test]
fn a_card_name_is_scoped_to_no_card_and_has_one_form() {
    let mut card = Lexeme::card_name("Chandra, Pyromaster", source("name:Chandra, Pyromaster"));
    assert!(Lexicon::new([card.clone()]).is_ok());
    card.card_scope = BTreeSet::from([LexemeId::new("name:Chandra, Torch of Defiance")]);
    assert!(Lexicon::new([card.clone()]).is_err());
    card.card_scope.clear();
    card.forms[0].surface = Some("Chandra".into());
    assert!(Lexicon::new([card]).is_err());
}

#[test]
fn a_card_scope_admits_spellings_on_the_cards_it_names() {
    // Travel Through Caradhras's vote option, in scope on that card alone.
    let mut option = invariant(
        "vote_option:Redhorn Pass",
        "Redhorn Pass",
        Category::Catalog,
    );
    option.capitalization = Capitalization::Exact;
    option.card_scope = BTreeSet::from([LexemeId::new("name:Travel Through Caradhras")]);
    let lexicon = Lexicon::new([option.clone()]).unwrap();
    let found = |scope: &[&str]| {
        let scope = CardScope::full_names(scope.iter().copied());
        lexicon
            .analyze_source_scoped("Redhorn Pass", None, &scope)
            .matches
            .iter()
            .any(|m| matches!(&m.reading, LexicalReading::Word(v) if v.lexeme == option.id))
    };
    assert!(found(&["Travel Through Caradhras"]));
    assert!(!found(&[]));
    assert!(!found(&["Vampire Hexmage"]));
    // The Lexeme's own spelling names none of the cards that admit it.
    assert!(!found(&["Redhorn Pass"]));
}

/// The lexicon of the card names a title run is held against.
fn card_names(names: &[&str]) -> Lexicon {
    Lexicon::new(
        names
            .iter()
            .map(|name| Lexeme::card_name(name, source(&format!("name:{name}")))),
    )
    .unwrap()
}

#[test]
fn a_title_run_is_offered_at_each_capitalized_word() {
    // Rinoa Heartilly.
    let lexicon = card_names(&["Rinoa Heartilly"]);
    let scope = CardScope::full_names(["Rinoa Heartilly"]);
    let analyzed = lexicon.analyze_source_scoped(
        "When Rinoa Heartilly enters, create Angelo, a legendary 1/1 green and white Dog \
         creature token.",
        None,
        &scope,
    );
    // The card's own name written out is no title run.
    assert_eq!(
        title_runs(&analyzed),
        ["When Rinoa Heartilly", "Angelo", "Dog"]
    );
    // A title run alone covers no word: only a parse makes it a name.
    assert_ne!(analyzed.unknown_words(), [] as [std::ops::Range<usize>; 0]);
    let angelo = LexicalReading::IntroducedName {
        name: "Angelo".to_owned(),
    };
    assert_eq!(lexicon.realize(&angelo).unwrap(), "Angelo");
    assert!(lexicon.stands_in(&angelo, &scope));
    assert_eq!(
        lexicon.edges(&angelo).unwrap().left.onset,
        Some(Onset::Vowel)
    );
}

#[test]
fn a_title_run_is_never_a_card_name_or_a_part_of_one() {
    let found = |lexicon: &Lexicon, card: &str, text: &str| -> Vec<String> {
        let scope = CardScope::full_names([card]);
        let analyzed = lexicon.analyze_source_scoped(text, None, &scope);
        title_runs(&analyzed)
            .into_iter()
            .map(str::to_owned)
            .collect()
    };
    // Helm of Kaldra: each card name written out ends its run at its comma,
    // and no run starts at one or inside one.
    let kaldra = card_names(&["Helm of Kaldra", "Sword of Kaldra", "Shield of Kaldra"]);
    assert_eq!(
        found(
            &kaldra,
            "Helm of Kaldra",
            "Equipment named Helm of Kaldra, Sword of Kaldra, and Shield of Kaldra",
        ),
        ["Equipment"]
    );
    // Urza, Lord Protector: the run stops before *and*, short of the card
    // name written there.
    let urza = card_names(&["Urza, Lord Protector", "The Mightstone and Weakstone"]);
    assert_eq!(
        found(
            &urza,
            "Urza, Lord Protector",
            "an artifact named The Mightstone and Weakstone",
        ),
        [] as [&str; 0]
    );
    // Skoa, Embermage: the name ends at its second comma.
    let skoa = card_names(&["Skoa, Embermage"]);
    assert_eq!(
        found(
            &skoa,
            "Grist, the Hunger Tide",
            "Discard another card named Skoa, Embermage, Sacrifice two Mountains",
        ),
        ["Discard", "Sacrifice", "Mountains"]
    );
    // Vanille, Cheerful l'Cie: the run ends before the lowercase *l'Cie*,
    // short of the card name written there.
    let fang = card_names(&["Vanille, Cheerful l'Cie", "Fang, Fearless l'Cie"]);
    assert_eq!(
        found(
            &fang,
            "Vanille, Cheerful l'Cie",
            "a creature named Fang, Fearless l'Cie"
        ),
        [] as [&str; 0]
    );
    // An Introduced Name is no card's name, and not the scope's own.
    let named = |name: &str| LexicalReading::IntroducedName {
        name: name.to_owned(),
    };
    assert!(skoa.realize(&named("Skoa, Embermage")).is_err());
    assert!(skoa.realize(&named("Skoa")).is_ok());
    let scope = CardScope::full_names(["Abuelo the Brave"]);
    assert!(!skoa.stands_in(&named("Abuelo the Brave"), &scope));
    // A Shortened Name ends no run at its comma: Icingdeath, Frost Tyrant
    // creates Icingdeath, Frost Tongue.
    let icingdeath = card_names(&["Icingdeath, Frost Tyrant"]);
    let scope = CardScope::every_form(["Icingdeath, Frost Tyrant"]);
    let analyzed = icingdeath.analyze_source_scoped(
        "create Icingdeath, Frost Tongue, a legendary white Equipment artifact token",
        None,
        &scope,
    );
    assert!(title_runs(&analyzed).contains(&"Icingdeath, Frost Tongue"));
}

#[test]
fn mode_bullets_stay_outside_open_flavor_word_labels() {
    let lexicon = Lexicon::new([]).unwrap();
    for label in [
        "Cure Wounds",
        "Dispel Magic",
        "Gentle Repose",
        "An Unlisted Label Ω!",
    ] {
        let expected = LexicalReading::FlavorWord {
            label: label.into(),
        };
        for prefix in ["• ", "Choose one —\n• "] {
            let text = format!("{prefix}{label} — You gain 2 life.");
            let start = prefix.chars().count();
            let end = start + label.chars().count() + 2;
            assert_eq!(
                at(&lexicon.analyze(&text), start, end),
                BTreeSet::from([expected.clone()])
            );
            assert_eq!(lexicon.realize(&expected).unwrap(), format!("{label} —"));
            assert!(at(&lexicon.analyze(&text), start - 2, end).is_empty());
        }
    }
    let invalid = LexicalReading::FlavorWord {
        label: "• Cure Wounds".into(),
    };
    assert!(lexicon.realize(&invalid).is_err());
    assert!(lexicon.edges(&invalid).is_err());
}

#[test]
fn independent_genitive_is_a_declared_pronoun_paradigm_cell() {
    let features = FeatureBundle {
        number: Some(Number::Singular),
        person: Some(Person::Second),
        case: Some(Case::Genitive),
        ..FeatureBundle::default()
    };
    let mut pronoun = invariant("you", "you", Category::Pronoun);
    pronoun.forms = vec![FormDeclaration {
        form: WordForm::IndependentGenitive,
        features: features.clone(),
        surface: Some("yours".into()),
    }];
    let lexicon = Lexicon::new([pronoun]).unwrap();
    let expected = word("you", WordForm::IndependentGenitive, features);
    assert_eq!(
        at(&lexicon.analyze("yours"), 0, 5),
        BTreeSet::from([expected.clone()])
    );
    assert_eq!(lexicon.realize(&expected).unwrap(), "yours");
}

#[test]
fn independent_genitive_requires_pronoun_genitive_features_and_explicit_spelling() {
    let features = FeatureBundle {
        number: Some(Number::Singular),
        person: Some(Person::Second),
        case: Some(Case::Genitive),
        ..FeatureBundle::default()
    };
    let mut pronoun = invariant("you", "you", Category::Pronoun);
    pronoun.forms = vec![FormDeclaration {
        form: WordForm::IndependentGenitive,
        features,
        surface: Some("yours".into()),
    }];
    let mut wrong_category = pronoun.clone();
    wrong_category.category = Category::Noun;
    let mut wrong_case = pronoun.clone();
    wrong_case.forms[0].features.case = Some(Case::Accusative);
    let mut verbal = pronoun.clone();
    verbal.forms[0].features.finiteness = Some(Finiteness::Finite);
    let mut missing_person = pronoun.clone();
    missing_person.forms[0].features.person = None;
    let mut missing_number = pronoun.clone();
    missing_number.forms[0].features.number = None;
    let mut missing_spelling = pronoun;
    missing_spelling.forms[0].surface = None;
    for invalid in [
        wrong_category,
        wrong_case,
        verbal,
        missing_person,
        missing_number,
        missing_spelling,
    ] {
        assert!(Lexicon::new([invalid]).is_err());
    }
}

#[test]
fn only_a_modal_preterite_has_no_tense() {
    // *If you would draw a card while your library has no cards in it*
    // (Laboratory Maniac): the modal's preterite fixes no time.
    let modal = |modal: bool, strip: &[WordForm]| {
        let mut will = verb("v:will", "will");
        will.properties.modal = modal;
        will.forms
            .retain(|slot| matches!(slot.form, WordForm::Present | WordForm::Preterite));
        override_form(&mut will, WordForm::Preterite, "would");
        for slot in &mut will.forms {
            if strip.contains(&slot.form) {
                slot.features.tense = None;
            }
        }
        Lexicon::new([will])
    };
    let lexicon = modal(true, &[WordForm::Preterite]).unwrap();
    let would = complete(&lexicon, "would");
    assert!(!would.is_empty());
    assert!(would.iter().all(|reading| matches!(
        reading,
        LexicalReading::Word(value)
            if value.form == WordForm::Preterite && value.features.tense.is_none()
    )));
    // A non-modal's preterite without a tense, a modal's with one, and a
    // present without one are each refused.
    let refused = |result: Result<Lexicon, LexicalError>, defect: &str| {
        let error = result.unwrap_err().to_string();
        assert!(error.contains(defect), "{error}");
    };
    refused(
        modal(false, &[WordForm::Preterite]),
        "a preterite without a tense on a verb that is not a modal",
    );
    refused(modal(true, &[]), "a modal's preterite with a tense");
    refused(
        modal(true, &[WordForm::Preterite, WordForm::Present]),
        "inapplicable form or feature bundle",
    );
    let mut cat = invariant("n:cat", "cat", Category::Noun);
    cat.properties.modal = true;
    refused(Lexicon::new([cat]), "a modal that is not a verb");
}

/// The Set Names analysis reads in `text`, by their spans' text.
fn set_names(lexicon: &Lexicon, text: &str) -> BTreeSet<String> {
    lexicon
        .analyze(text)
        .matches
        .iter()
        .filter_map(|found| match &found.reading {
            LexicalReading::SetName { name } => Some(name.clone()),
            _ => None,
        })
        .collect()
}

fn set_name_host() -> Lexicon {
    let mut expansion = noun("noun:expansion", "expansion");
    expansion.set_name_host = true;
    Lexicon::new([
        expansion,
        invariant("det:the", "the", Category::Determinative),
        invariant("prep:in", "in", Category::Preposition),
    ])
    .unwrap()
}

/// A Set Name is read by its title shape alone, right before a noun that
/// takes one [CR#206.3a..206.3c]: no list of set names is consulted, and no
/// word of the run is refused for spelling an English word or a card's name.
/// Each name is a `SetList.json` shape: a colon subtitle, a word-final
/// apostrophe, interior prepositions, digits and a comma inside a numeral.
#[test]
fn a_set_name_is_read_from_its_title_shape_before_its_noun() {
    let lexicon = set_name_host();
    for name in [
        "Antiquities",
        "Arabian Nights",
        "Time Spiral",
        "Urza's Saga",
        "Phyrexia: All Will Be One",
        "The Brothers' War",
        "Shadows over Innistrad",
        "Warhammer 40,000 Commander",
        "30th Anniversary Edition",
        "Duel Decks: Elves vs. Goblins",
    ] {
        let text = format!("originally printed in the {name} expansion");
        assert_eq!(
            set_names(&lexicon, &text),
            BTreeSet::from([name.to_owned()]),
            "{text}"
        );
        let start = "originally printed in the ".chars().count();
        let reading = LexicalReading::SetName { name: name.into() };
        assert!(
            at(&lexicon.analyze(&text), start, start + name.chars().count()).contains(&reading)
        );
        assert_eq!(lexicon.realize(&reading).unwrap(), name);
        assert!(!lexicon.takes_capital(&reading).unwrap());
        assert!(lexicon.stands_in(&reading, &CardScope::default()));
    }
    assert_eq!(
        lexicon
            .edges(&LexicalReading::SetName {
                name: "Antiquities".into()
            })
            .unwrap()
            .left
            .onset,
        Some(Onset::Vowel)
    );
}

/// A Set Name is the whole title run before its noun, never a shorter run
/// inside it, so a capitalized word before it, joined by connectors, is part of
/// it.
#[test]
fn a_set_name_is_the_whole_title_run() {
    let lexicon = set_name_host();
    for (text, name) in [
        ("the Urza's Saga expansion", "Urza's Saga"),
        (
            "printed in the Adventures in the Forgotten Realms expansion",
            "Adventures in the Forgotten Realms",
        ),
        (
            "each Island in the Antiquities expansion",
            "Island in the Antiquities",
        ),
    ] {
        assert_eq!(
            set_names(&lexicon, text),
            BTreeSet::from([name.to_owned()]),
            "{text}"
        );
    }
}

/// No Set Name is read where no noun that takes one follows, nor from a run
/// that is no title run.
#[test]
fn a_set_name_is_read_only_before_its_noun() {
    let lexicon = set_name_host();
    for text in [
        "the Antiquities",
        "the antiquities expansion",
        "the Antiquities  expansion",
        "the Antiquities of expansion",
        "Antiquities\nexpansion",
    ] {
        assert!(set_names(&lexicon, text).is_empty(), "{text}");
    }
    // The plural is a form of the noun that takes a Set Name.
    assert_eq!(
        set_names(&lexicon, "the Antiquities expansions"),
        BTreeSet::from(["Antiquities".to_owned()])
    );
    let without_host = Lexicon::new([noun("noun:expansion", "expansion")]).unwrap();
    assert!(set_names(&without_host, "the Antiquities expansion").is_empty());
    for name in [
        "",
        "antiquities",
        "Antiquities ",
        "of Antiquities",
        "Antiquities of",
        "A\tB",
    ] {
        assert!(
            lexicon
                .realize(&LexicalReading::SetName { name: name.into() })
                .is_err(),
            "{name:?}"
        );
    }
}

#[test]
fn only_a_noun_takes_a_set_name() {
    let mut host = invariant("prep:in", "in", Category::Preposition);
    host.set_name_host = true;
    assert!(Lexicon::new([host]).is_err());
}

use sylex_lexicon::Category;
use sylex_lexicon::Lexeme;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Numeral;
use sylex_lexicon::Onset;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_lexicon::SurfaceCase;

fn row(id: &str, spelling: &str) -> Lexeme {
    Lexeme::invariant(
        id,
        spelling,
        Category::Adjective,
        Source {
            kind: SourceKind::Core,
            path: "pronunciation-fixture".into(),
            owner: id.into(),
        },
    )
}

/// A word's Onset is its first letter's unless its Lexeme declares one: the
/// silent *h* of *hour* (Midnight Clock's *an hour counter*) and the /j/ of
/// *unity* (Call for Unity's *a unity counter*).
#[test]
fn a_declared_onset_replaces_the_first_letter_s() {
    let declared = |id: &str, onset| {
        let mut lexeme = row(id, id);
        lexeme.onset = Some(onset);
        lexeme
    };
    let lexicon = Lexicon::new([
        declared("hour", Onset::Vowel),
        declared("unity", Onset::Consonant),
        row("island", "island"),
        row("forest", "forest"),
    ])
    .unwrap();
    for value in lexicon.values() {
        let reading = LexicalReading::Word(value.clone());
        let surface = lexicon.realize(&reading).unwrap();
        let properties = lexicon.surface_features(&reading).unwrap();
        let expected = if value.lexeme == "hour" || value.lexeme == "island" {
            Onset::Vowel
        } else {
            Onset::Consonant
        };
        assert_eq!(properties.onset, Some(expected), "{value:?}");
        assert_eq!(
            properties.initial,
            value.capitalization == SurfaceCase::Initial
        );
        assert_eq!(
            properties.interior,
            value.capitalization == SurfaceCase::Declared
        );
        assert!(
            lexicon
                .analyze(&surface)
                .matches
                .iter()
                .any(|m| m.reading == reading)
        );
    }
    // A numeral is spoken as its cardinal, whose *one* begins with /w/.
    for (value, expected) in [
        (1, Onset::Consonant),
        (8, Onset::Vowel),
        (11, Onset::Vowel),
        (18, Onset::Vowel),
        (80, Onset::Vowel),
        (180, Onset::Consonant),
        (800, Onset::Vowel),
        (-8, Onset::Consonant),
    ] {
        let reading = LexicalReading::Numeral {
            value,
            notation: Numeral::Arabic(false),
            capitalization: SurfaceCase::Declared,
        };
        let properties = lexicon.surface_features(&reading).unwrap();
        assert_eq!(properties.onset, Some(expected));
        assert!(properties.initial && properties.interior);
    }
}

#[test]
fn invalid_pronunciation_declarations_fail_loading() {
    // A declaration is an irregularity: *forest* begins with a consonant
    // anyway.
    let mut redundant = row("forest", "forest");
    redundant.onset = Some(Onset::Consonant);
    assert!(
        Lexicon::new([redundant])
            .unwrap_err()
            .to_string()
            .contains("by default")
    );
    let mut not_an_article = row("adjective", "uniform");
    not_an_article.forms[0].features.following_onset = Some(Onset::Vowel);
    assert!(
        Lexicon::new([not_an_article])
            .unwrap_err()
            .to_string()
            .contains("inapplicable")
    );
    // *a* alone, with no form for a vowel Onset.
    let mut incomplete = row("article", "a");
    incomplete.category = Category::Determinative;
    incomplete.forms[0].features.following_onset = Some(Onset::Consonant);
    assert!(
        Lexicon::new([incomplete])
            .unwrap_err()
            .to_string()
            .contains("lacks a form")
    );
}

#[test]
fn multiword_names_and_bound_forms_take_the_first_pronounced_onset() {
    for (spelling, expected) in [
        ("Eight-and-a-Half-Tails", Some(Onset::Vowel)),
        ("Urza's Saga", Some(Onset::Vowel)),
        ("island", Some(Onset::Vowel)),
        ("forest", Some(Onset::Consonant)),
        // A numeral is spoken as its cardinal.
        ("2ˣ", Some(Onset::Consonant)),
        // No letter or numeral begins these, and they declare no Onset.
        ("Éomer", None),
        ("{T}", None),
    ] {
        let mut entry = row("onset", spelling);
        entry.category = Category::Catalog;
        entry.surface_structure = sylex_lexicon::SurfaceStructure::Opaque;
        if matches!(spelling, "island" | "forest") {
            entry.binding = sylex_lexicon::Binding::Prefix;
        }
        let lexicon = Lexicon::new([entry]).unwrap();
        for value in lexicon.values() {
            let reading = LexicalReading::Word(value.clone());
            assert_eq!(
                lexicon.surface_features(&reading).unwrap().onset,
                expected,
                "{spelling}"
            );
        }
    }
}

/// A measure's sign is spoken, *plus* or *minus*: *a +1/+1 counter*, *a
/// -1/-1 counter*.
#[test]
fn a_measured_compound_takes_the_onset_of_its_measure() {
    for (spelling, expected) in [
        ("+1/+1 counter", Onset::Consonant),
        ("-1/-1 counter", Onset::Consonant),
        ("8/8 counter", Onset::Vowel),
    ] {
        let mut entry = Lexeme::noun(
            "counter",
            spelling,
            vec![sylex_lexicon::Countability::Count],
            Source {
                kind: SourceKind::Catalog,
                path: "pronunciation-fixture".into(),
                owner: "counter".into(),
            },
        );
        entry.surface_structure = sylex_lexicon::SurfaceStructure::MeasuredCompound;
        let lexicon = Lexicon::new([entry]).unwrap();
        for value in lexicon.values() {
            let reading = LexicalReading::Word(value.clone());
            assert_eq!(
                lexicon.surface_features(&reading).unwrap().onset,
                Some(expected),
                "{spelling}"
            );
        }
    }
}

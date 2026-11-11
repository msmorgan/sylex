//! Subcategorization Frames drive phrases uniformly for every head
//! category, not only verbs and prepositions.

use std::collections::BTreeSet;

use sylex_lexicon::Category;
use sylex_lexicon::Countability;
use sylex_lexicon::Lexeme;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;
use sylex_syntax::Reading;
use sylex_syntax::frames::FrameFamily;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/frames.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let mut lexemes = vec![
        Lexeme::verb("v:see", "see", core()),
        Lexeme::invariant("a:equal", "equal", Category::Adjective, core()),
    ];
    for word in ["the", "a", "each"] {
        lexemes.push(Lexeme::invariant(
            format!("d:{word}"),
            word,
            Category::Determinative,
            core(),
        ));
    }
    for word in ["of", "to"] {
        lexemes.push(Lexeme::invariant(
            format!("p:{word}"),
            word,
            Category::Preposition,
            core(),
        ));
    }
    for noun in ["card", "cost"] {
        lexemes.push(Lexeme::noun(
            format!("n:{noun}"),
            noun,
            vec![Countability::Count],
            core(),
        ));
    }
    Lexicon::new(lexemes).unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:see": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "n:cost": [(family: NounComplement, slots: [(relation: Complement, fill: PP(["p:of"]))])],
        "a:equal": [(family: AdjectiveComplement, slots: [(relation: Complement, fill: PP(["p:to"]))])],
        "d:each": [(family: DeterminativeComplement, slots: [(relation: Complement, fill: PP(["p:of"]))])],
    },
    prepositions: {
        "p:of": [(complement: NP, hosts: ComplementOnly)],
        "p:to": [(complement: NP, hosts: ComplementOnly)],
    },
)"#;

const GRAMMAR: &str = r"
    root Sentence
    feature Number = Singular | Plural
    feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
    category AdjectivePhrase
    category DeterminerPhrase
    category Nominal [Number]
    category NounPhrase
    category PrepositionPhrase
    category Sentence
    category VerbPhrase [VerbForm]
    slot NP = NounPhrase
    slot PP = PrepositionPhrase
    Imperative: Sentence -> clause:VerbPhrase '.' where clause.VerbForm = Plain
    Determined: NounPhrase -> determiner:DeterminerPhrase Nominal
    FusedHead: NounPhrase -> DeterminerPhrase
    Postpositive: Nominal -> Nominal modifier:AdjectivePhrase
    valence VerbPhrase { head: Verb }
    valence PrepositionPhrase { head: Preposition }
    valence Nominal { head: Noun }
    valence AdjectivePhrase { head: Adjective }
    valence DeterminerPhrase { head: Determinative }
";

/// Every Reading of `text`, each checked for Admission and round-trip law 1.
fn readings(text: &str) -> Vec<Reading> {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(GRAMMAR).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap();
    let readings = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
    for reading in &readings {
        assert_eq!(grammar.admit(reading, &lexicon, &BTreeSet::new()), Ok(()));
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    readings
}

/// The one Reading of `text`.
fn only_reading(text: &str) -> Reading {
    let mut readings = readings(text);
    assert_eq!(readings.len(), 1, "{text}: {readings:#?}");
    readings.remove(0)
}

/// The Frame Family selected by the first `category` Constituent of `words`
/// words that has a Selected Frame.
fn selected(reading: &Reading, category: &str, words: usize) -> Option<FrameFamily> {
    fn leaves(reading: &Reading) -> usize {
        match reading {
            Reading::Leaf(_) => 1,
            Reading::Node(node) => node.children.iter().map(leaves).sum(),
        }
    }
    let node = reading.node()?;
    if node.category == category && leaves(reading) == words && node.selected_frame.is_some() {
        return node.selected_frame.as_ref().map(|f| f.family);
    }
    node.children
        .iter()
        .find_map(|c| selected(c, category, words))
}

#[test]
fn noun_selects_its_complement() {
    let reading = only_reading("See the cost of the card.");
    assert_eq!(
        selected(&reading, "Nominal", 4),
        Some(FrameFamily::NounComplement)
    );
}

#[test]
fn adjective_selects_its_complement() {
    let reading = only_reading("See a card equal to the cost.");
    assert_eq!(
        selected(&reading, "AdjectivePhrase", 4),
        Some(FrameFamily::AdjectiveComplement)
    );
}

#[test]
fn determinative_selects_its_complement() {
    // This toy grammar also lets a bare `the` head a noun phrase, so
    // `[each of the] cards` is a second Reading; the claim is only that the
    // determinative's frame selects `of the cards`.
    assert!(readings("See each of the cards.").iter().any(|reading| {
        selected(reading, "DeterminerPhrase", 4) == Some(FrameFamily::DeterminativeComplement)
    }));
}

#[test]
fn a_complement_frame_is_optional_for_non_verbs() {
    let reading = only_reading("See the cost.");
    assert_eq!(selected(&reading, "Nominal", 1), None);
}

//! Unlike slots: where a head's frames select two or more of an unlike slot's
//! kinds in one position, the engine derives a Frame Slot there filled by the
//! slot's coordination, restricted to the kinds that head selects (CGEL,
//! Ch. 15 §3.1, p. 1323, rule [1]). Grounded in Tolaria's *loses banding and
//! all "bands with other" abilities*.

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
use sylex_syntax::frames::SlotFill;
use sylex_syntax::frames::SlotKind;

fn lexicon() -> Lexicon {
    let source = |kind| Source {
        kind,
        path: "tests/unlike_frames.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("c:and", "and", Category::Coordinator, source(SourceKind::Core)),
        Lexeme::verb("v:lose", "lose", source(SourceKind::Catalog)),
        Lexeme::verb("v:destroy", "destroy", source(SourceKind::Catalog)),
        Lexeme::invariant(
            "k:banding",
            "banding",
            Category::Keyword,
            source(SourceKind::Catalog),
        ),
        Lexeme::noun(
            "n:ability",
            "ability",
            vec![Countability::Count],
            source(SourceKind::Catalog),
        ),
    ])
    .unwrap()
}

// *Lose* selects a Noun Phrase or a keyword phrase as its Object, *destroy*
// only a Noun Phrase.
const FRAMES: &str = r#"(
    frames: {
        "v:lose": [
            (family: Monotransitive, slots: [(relation: Object, fill: NP)]),
            (family: Monotransitive, slots: [(relation: Object, fill: KeywordPhrase)]),
        ],
        "v:destroy": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
    },
)"#;

// The unlike slot's kinds include Amount, which neither verb selects.
const GRAMMAR: &str = r#"
    root Sentence
    feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
    feature Voice = Active | Passive
    feature ComplementKind = NP | KeywordPhrase | Amount
    category AmountPhrase
    category KeywordPhrase
    category Nominal
    category NounPhrase
    category ObjectConjunct [ComplementKind]
    category ObjectCoordination [ComplementKind]
    category Sentence
    category VerbPhrase [VerbForm, Voice] { gap }
    word And = "c:and"
    slot NP = NounPhrase
    slot KeywordPhrase = KeywordPhrase
    slot Amount = AmountPhrase
    slot Nominal = Nominal
    slot Objects = ObjectCoordination { unlike: [NP, KeywordPhrase, Amount] }
    Imperative: Sentence -> clause:VerbPhrase '.' where clause.VerbForm = Plain, clause.Voice = Active
    Numeral: AmountPhrase -> Lex(Numeral)
    Keyword: KeywordPhrase -> Lex(Keyword)
    Bare: NounPhrase -> Nominal
    NominalConjunct: ObjectConjunct -> NounPhrase where $.ComplementKind = NP
    KeywordConjunct: ObjectConjunct -> KeywordPhrase where $.ComplementKind = KeywordPhrase
    AmountConjunct: ObjectConjunct -> AmountPhrase where $.ComplementKind = Amount
    coordination ObjectCoordination {
        coordinators: And,
        unshared: [ComplementKind],
        serial: true,
        conjunct: ObjectConjunct,
        unlike: ComplementKind,
    }
    valence VerbPhrase { head: Verb, active: [$.Voice = Active] }
    valence Nominal { head: Noun }
"#;

/// The Readings of `text`, each checked for Admission and round-trip law 1.
fn readings(text: &str) -> Vec<Reading> {
    readings_in(GRAMMAR, text)
}

fn readings_in(grammar: &str, text: &str) -> Vec<Reading> {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(grammar).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap();
    let readings = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
    for reading in &readings {
        assert_eq!(
            grammar.admit(reading, &lexicon, &sylex_lexicon::CardScope::default()),
            Ok(()),
            "{text}"
        );
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    readings
}

fn assert_no_reading(text: &str) {
    let readings = readings(text);
    assert_eq!(readings.len(), 0, "{text}: {readings:#?}");
}

/// The Fill of the Object slot of the clause's Selected Frame.
fn object_fill(reading: &Reading) -> SlotFill {
    let clause = reading.node().unwrap().children[0].node().unwrap();
    clause.selected_frame.as_ref().unwrap().slots[0].fill.clone()
}

#[test]
fn a_head_selecting_two_kinds_takes_their_coordination_restricted_to_them() {
    for text in ["Lose banding and abilities.", "Lose abilities and banding."] {
        let readings = readings(text);
        assert_eq!(readings.len(), 1, "{text}: {readings:#?}");
        let SlotFill::Unlike(unlike) = object_fill(&readings[0]) else {
            panic!("{text}: no unlike Object: {readings:#?}");
        };
        assert_eq!(unlike.name.as_str(), "Objects");
        assert_eq!(unlike.kinds, [SlotKind::NP, SlotKind::KeywordPhrase]);
    }
}

#[test]
fn a_conjunct_of_a_kind_the_head_does_not_select_is_no_object() {
    // The coordination admits an Amount conjunct, but *lose* selects none.
    assert_no_reading("Lose banding and 2.");
    assert_no_reading("Lose 2 and abilities.");
}

#[test]
fn a_head_selecting_one_kind_takes_no_unlike_coordination() {
    assert_no_reading("Destroy abilities and banding.");
    assert_no_reading("Destroy banding and abilities.");
    assert_eq!(object_fill(&readings("Destroy abilities.")[0]), SlotFill::NP);
}

#[test]
fn each_kind_alone_keeps_its_own_frame() {
    assert_eq!(object_fill(&readings("Lose abilities.")[0]), SlotFill::NP);
    assert_eq!(
        object_fill(&readings("Lose banding.")[0]),
        SlotFill::KeywordPhrase
    );
}

#[test]
fn a_conjunct_with_no_value_of_the_unlike_feature_is_of_no_selected_kind() {
    // An Amount conjunct left unvalued: its coordination with a keyword and a
    // Noun Phrase still has unlike conjuncts, but fills no derived slot.
    let unvalued = GRAMMAR.replacen(
        "AmountConjunct: ObjectConjunct -> AmountPhrase where $.ComplementKind = Amount",
        "AmountConjunct: ObjectConjunct -> AmountPhrase",
        1,
    );
    assert_ne!(unvalued, GRAMMAR);
    assert_eq!(readings_in(&unvalued, "Lose banding, 2, and abilities.").len(), 0);
    assert_eq!(readings_in(&unvalued, "Lose banding and abilities.").len(), 1);
}

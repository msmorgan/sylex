//! A Valence schema's `slots` equations constrain its own phrases' Frame
//! Slots of a kind, beside the grammar's `slot` declaration of that kind: a
//! noun's Amount Complement is no comparative bound (*mana value 3*, never
//! *mana value less than 3* with *less than 3* its Complement), while a
//! preposition's Amount Complement is one (*reduce it to less than 1*).

use sylex_lexicon::Category;
use sylex_lexicon::Countability;
use sylex_lexicon::Lexeme;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;
use sylex_syntax::LoadError;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/schema_slot_equations.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::noun("n:value", "value", vec![Countability::Count], core()),
        Lexeme::invariant("p:to", "to", Category::Preposition, core()),
        Lexeme::invariant("d:less", "less", Category::Determinative, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "n:value": [(family: NounComplement, slots: [(relation: Complement, fill: Amount)])],
    },
    prepositions: {
        "p:to": [(complement: Amount, hosts: ComplementOnly)],
    },
)"#;

const GRAMMAR: &str = r#"
    root Sentence
    feature Bound = Upper
    category Amount [Bound]
    category Nominal
    category PrepositionPhrase
    category Sentence
    word Less = "d:less"
    slot Amount = Amount
    Number: Amount -> Lex(Numeral)
    UpperBound: Amount -> Word(Less) Lex(Numeral) where $.Bound = Upper
    Noun: Sentence -> Nominal '.'
    Preposition: Sentence -> PrepositionPhrase '!'
    valence Nominal { head: Noun, slots: [Amount -> [slot.Bound != Upper]] }
    valence PrepositionPhrase { head: Preposition }
"#;

fn bind(grammar: &str, lexicon: &Lexicon) -> Result<CompiledGrammar, LoadError> {
    CompiledGrammar::bind(
        &Grammar::from_fug_str(grammar)?,
        lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
}

fn accepted(text: &str) -> bool {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    grammar.parse(&lexicon, text).readings().next().is_some()
}

#[test]
fn the_schemas_slot_refuses_what_its_equations_exclude() {
    assert!(accepted("value 3."));
    assert!(!accepted("value less 3."));
}

#[test]
fn another_heads_slot_of_the_kind_is_unconstrained() {
    assert!(accepted("to 3!"));
    assert!(accepted("to less 3!"));
}

#[test]
fn a_preposition_schema_has_no_slots() {
    let grammar = GRAMMAR.replace(
        "valence PrepositionPhrase { head: Preposition }",
        "valence PrepositionPhrase { head: Preposition, slots: [Amount -> [slot.Bound != Upper]] }",
    );
    let error = bind(&grammar, &lexicon()).unwrap_err().to_string();
    assert!(error.contains("are not a preposition head's"), "{error}");
}

#[test]
fn a_kind_listed_twice_is_a_load_error() {
    let grammar = GRAMMAR.replace(
        "slots: [Amount -> [slot.Bound != Upper]]",
        "slots: [Amount -> [slot.Bound != Upper], Amount -> [slot.Bound != Upper]]",
    );
    let error = bind(&grammar, &lexicon()).unwrap_err().to_string();
    assert!(error.contains("lists Amount twice"), "{error}");
}

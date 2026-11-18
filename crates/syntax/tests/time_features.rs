//! The lexicon feature-bundle mapping table carries a verb form's tense into
//! the grammar as its `Time`: a present is `NonPast`, a preterite `Past`, and
//! a form with no tense leaves it unvalued: a modal's preterite, ambiguous
//! between past time and modal remoteness, fixes no time (CGEL, Ch. 3 §2.4,
//! p. 107).

use sylex_lexicon::Lexeme;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_lexicon::WordForm;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/time_features.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let mut draw = Lexeme::verb("v:draw", "draw", core());
    // *If you would draw a card while your library has no cards in it*
    // (Laboratory Maniac).
    let mut will = Lexeme::verb("v:will", "will", core());
    will.properties.modal = true;
    will.forms
        .retain(|slot| matches!(slot.form, WordForm::Present | WordForm::Preterite));
    for slot in &mut will.forms {
        if slot.form == WordForm::Preterite {
            slot.surface = Some("would".to_owned());
            slot.features.tense = None;
        }
    }
    for slot in &mut draw.forms {
        if matches!(slot.form, WordForm::Preterite) {
            slot.surface = Some("drew".to_owned());
        }
    }
    Lexicon::new([draw, will]).unwrap()
}

/// Whether the text is a verb whose `Time` is `Past`, and whether one whose
/// `Time` is `NonPast`.
fn times(text: &str) -> (bool, bool) {
    let lexicon = lexicon();
    let parses = |equations: &str| {
        let source = format!(
            "
            root Sentence
            feature Time = Past | Mixed | NonPast
            category Sentence
            Verb: Sentence -> Lex(Verb) '.' {{ capital: [head] }} {equations}
            "
        );
        let grammar = CompiledGrammar::bind(
            &Grammar::from_fug_str(&source).unwrap(),
            &lexicon,
            &FrameCatalog::default(),
        )
        .unwrap();
        grammar.parse(&lexicon, text).readings().next().is_some()
    };
    assert!(parses(""), "{text}");
    (
        parses("where head.Time := Past"),
        parses("where head.Time := NonPast"),
    )
}

#[test]
fn a_preterite_is_past_and_a_present_non_past() {
    assert_eq!(times("Drew."), (true, false));
    assert_eq!(times("Draws."), (false, true));
    assert_eq!(times("Will."), (false, true));
}

#[test]
fn a_form_with_no_tense_is_neither() {
    // *would*, a modal's preterite.
    assert_eq!(times("Would."), (false, false));
}

//! A Production can require a child to be light (one lexical leaf) or heavy
//! (more than one), as a single postpositive participle differs from a
//! participial clause.

use std::collections::BTreeSet;

use sylex_lexicon::Category;
use sylex_lexicon::Lexeme;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;

fn lexicon() -> Lexicon {
    let core = Source {
        kind: SourceKind::Core,
        path: "tests/weights.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([Lexeme::invariant("n:card", "card", Category::Noun, core)]).unwrap()
}

fn grammar(lexicon: &Lexicon, light: &str) -> Result<CompiledGrammar, sylex_syntax::LoadError> {
    let text = format!(
        r"
            root Sentence
            category Sentence
            category Words
            Word: Words -> Lex(Noun)
            MoreWords: Words -> Words next:Lex(Noun)
            Light: Sentence -> words:Words '.' {{ light: [{light}] }}
            Heavy: Sentence -> words:Words '!' {{ heavy: [words] }}
        "
    );
    CompiledGrammar::bind(
        &Grammar::from_fug_str(&text).unwrap(),
        lexicon,
        &FrameCatalog::default(),
    )
}

#[test]
fn a_light_child_is_one_leaf_and_a_heavy_child_more() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, "words").unwrap();
    for (text, readings) in [
        ("Card.", 1),
        ("Card card.", 0),
        ("Card!", 0),
        ("Card card!", 1),
    ] {
        let found = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
        assert_eq!(found.len(), readings, "{text}: {found:#?}");
        for reading in &found {
            assert_eq!(
                grammar.admit(reading, &lexicon, &BTreeSet::new()),
                Ok(()),
                "{text}"
            );
        }
    }
}

#[test]
fn a_weighted_child_must_exist() {
    assert!(grammar(&lexicon(), "nothing").is_err());
}

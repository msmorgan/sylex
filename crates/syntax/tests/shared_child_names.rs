//! An equation on a child name that several children share constrains each
//! of them that carries the feature, not only the first: two preposed
//! Adjuncts set off by one comma both bear `adjunct`, and a requirement on
//! `adjunct` holds of both.

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

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/shared_child_names.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("d:the", "the", Category::Determinative, core()),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
        Lexeme::noun("n:land", "land", vec![Countability::Count], core()),
    ])
    .unwrap()
}

fn grammar(lexicon: &Lexicon, equations: &str) -> Result<CompiledGrammar, String> {
    let text = format!(
        r#"
            root Phrase
            feature Determination = Bare | Overt
            category NounPhrase [Determination]
            category Phrase
            word The = "d:the"
            Bare: NounPhrase -> Lex(Noun) where $.Determination = Bare
            Determined: NounPhrase -> determiner:Word(The) Lex(Noun) where $.Determination = Overt
            Pair: Phrase -> adjunct:NounPhrase adjunct:NounPhrase '.' where {equations}
        "#
    );
    let grammar = Grammar::from_fug_str(&text).map_err(|e| format!("{e:?}"))?;
    CompiledGrammar::bind(&grammar, lexicon, &FrameCatalog::default()).map_err(|e| format!("{e:?}"))
}

fn count(equations: &str, text: &str) -> usize {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, equations).unwrap();
    let found = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
    for reading in &found {
        assert_eq!(
            grammar.admit(reading, &lexicon, &BTreeSet::new()),
            Ok(()),
            "{text}"
        );
    }
    found.len()
}

#[test]
fn a_requirement_on_a_shared_name_holds_of_every_child() {
    let overt = "adjunct.Determination = Overt";
    assert_eq!(count(overt, "The card the land."), 1);
    assert_eq!(count(overt, "Card the land."), 0);
    assert_eq!(count(overt, "The card land."), 0);
}

#[test]
fn an_exclusion_on_a_shared_name_holds_of_every_child() {
    let bare = "adjunct.Determination != Bare";
    assert_eq!(count(bare, "The card the land."), 1);
    assert_eq!(count(bare, "Card the land."), 0);
    assert_eq!(count(bare, "The card land."), 0);
}

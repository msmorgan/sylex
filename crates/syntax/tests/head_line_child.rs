//! The head-line child of a Production with no `head`, no coordination
//! schema, no `coordinator` or `marker` and more than one child: the first
//! child that bears no dependent function, not a dependent before it (step 6
//! of `CompiledGrammar::head_line_child`), and the first child when every
//! child bears one (step 7). Haakon, Stromgald Scourge: *but not from anywhere
//! else*, where *not* modifies the coordinate it precedes.

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
        path: "tests/head_line_child.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("a:not", "not", Category::Adverb, core()),
        Lexeme::noun(
            "n:graveyard",
            "graveyard",
            vec![Countability::Count],
            core(),
        ),
    ])
    .unwrap()
}

const GRAMMAR: &str = r#"
    root Fragment
    category Fragment
    category NounPhrase
    word Not = "a:not"
    Bare: NounPhrase -> Lex(Noun)
    Negated: Fragment -> modifier:Word(Not) filler:NounPhrase
    Dependents: Fragment -> modifier:Word(Not) adjunct:NounPhrase '.'
"#;

/// The name of the child the Production's head line runs through.
fn head_line_function(id: &str) -> String {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(GRAMMAR).unwrap(),
        &lexicon,
        &FrameCatalog::default(),
    )
    .unwrap();
    let production = grammar
        .production_index(dpsi::Ident::new(id))
        .unwrap_or_else(|| panic!("no Production `{id}`"));
    let at = grammar
        .head_line_child(production)
        .unwrap_or_else(|| panic!("`{id}` has no head-line child"));
    grammar.production_functions(production)[at].to_string()
}

#[test]
fn the_head_line_skips_a_dependent_before_the_first_other_child() {
    assert_eq!(head_line_function("Negated"), "filler");
}

#[test]
fn the_head_line_of_all_dependents_runs_through_the_first() {
    assert_eq!(head_line_function("Dependents"), "modifier");
}

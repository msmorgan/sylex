//! A local feature (`feature Name = Value { local }`) is a fact of the phrase
//! whose Production values it: a mother does not share it with its `head`
//! child implicitly, a coordination's conjuncts do not share it nor give it to
//! the coordination, and an explicit equation (`$.F = head.F`, `child.F := V`)
//! relates it as any other feature. The same grammar with an ordinary feature
//! is the control. *Shape* stands for the clause shape of a card's text, which
//! only the Production that builds a clause tells (Haakon, Stromgald Scourge:
//! *You may cast this card from your graveyard*).

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
        path: "tests/local_features.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
    ])
    .unwrap()
}

/// The grammar, its feature declared as `setting`. `Marked` values `Shape` on
/// its mother, `Unmarked` values another, `Wrapped` and `Passed` have a `Phrase` head child, `Passed`
/// passing the feature up by equation, and `Consumer` requires the carried
/// value of its head.
fn grammar(lexicon: &Lexicon, setting: &str) -> CompiledGrammar {
    let text = format!(
        r#"
            root Sentence
            feature Shape = Marked | Other {setting}
            category Phrase [Shape]
            category Sentence
            word And = "c:and"
            Marked: Phrase -> Lex(Noun) where $.Shape = Marked
            Unmarked: Phrase -> Lex(Noun) '~' where $.Shape = Other
            Wrapped: Phrase -> Phrase '!'
            Passed: Phrase -> Phrase '?' where $.Shape = head.Shape
            Free: Sentence -> Phrase ';'
            Consumer: Sentence -> Phrase '.' where head.Shape := Marked
            coordination Phrase {{ coordinators: And }}
        "#
    );
    CompiledGrammar::bind(
        &Grammar::from_fug_str(&text).unwrap(),
        lexicon,
        &FrameCatalog::default(),
    )
    .unwrap()
}

fn accepted(setting: &str, input: &str) -> bool {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, setting);
    grammar.parse(&lexicon, input).readings().next().is_some()
}

#[test]
fn a_mother_does_not_share_a_local_feature_with_its_head() {
    assert!(accepted("{ local }", "card."));
    assert!(!accepted("{ local }", "card!."));
    // Control: an ordinary feature passes up to the head's mother.
    assert!(accepted("", "card!."));
}

#[test]
fn a_coordination_neither_carries_nor_shares_a_local_feature() {
    // The coordination does not carry it, though both conjuncts do.
    assert!(!accepted("{ local }", "card and card."));
    // Control: the conjuncts share an ordinary feature, which the
    // coordination carries.
    assert!(accepted("", "card and card."));
}

#[test]
fn conjuncts_need_not_share_a_local_feature() {
    // Phrases that value it differently coordinate.
    assert!(accepted("{ local }", "card and card~;"));
    // Control: conjuncts share an ordinary feature, so these differ.
    assert!(!accepted("", "card and card~;"));
    assert!(accepted("", "card~ and card~;"));
    assert!(!accepted("{ local }", "card and card~."));
}

#[test]
fn an_explicit_equation_passes_a_local_feature_up() {
    assert!(accepted("{ local }", "card?."));
    assert!(accepted("{ local }", "card??."));
    assert!(!accepted("{ local }", "card!?."));
}

#[test]
fn a_feature_takes_no_setting_but_local() {
    assert!(Grammar::from_fug_str("root S\ncategory S\nfeature F = A { local }\n").is_ok());
    assert!(
        Grammar::from_fug_str("root S\ncategory S\nfeature F = A { local }\n")
            .unwrap()
            .local_features
            .contains("F")
    );
    for setting in [
        "{ gap }",
        "{ local: false }",
        "{ local, gap }",
        "{ cost: 2 }",
    ] {
        let source = format!("root S\ncategory S\nfeature F = A {setting}\n");
        assert!(
            matches!(Grammar::from_fug_str(&source), Err(LoadError::Format(_))),
            "{setting}"
        );
    }
}

//! `coordination over [A, B] { ... }` is one declaration for several
//! Categories: it expands to one Coordination schema per listed Category with
//! the same settings. A list of fewer than two Categories, a missing list or a
//! Category named twice is a load error.

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
use sylex_syntax::grammar::SchemaDecl;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/coordination_over.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("p:from", "from", Category::Preposition, core()),
        Lexeme::invariant("d:your", "your", Category::Determinative, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
        Lexeme::noun(
            "n:graveyard",
            "graveyard",
            vec![Countability::Count],
            core(),
        ),
        Lexeme::noun("n:hand", "hand", vec![Countability::Count], core()),
    ])
    .unwrap()
}

fn text(coordination: &str) -> String {
    format!(
        r#"
            root Sentence
            category NounPhrase
            category PrepositionPhrase
            category Sentence
            word And = "c:and"
            Determined: NounPhrase -> determiner:Lex(Determinative) Lex(Noun)
            Preposed: PrepositionPhrase -> Lex(Preposition) NounPhrase
            Phrase: Sentence -> PrepositionPhrase '.' {{ capital: [head] }}
            Noun: Sentence -> NounPhrase ';' {{ capital: [head] }}
            {coordination}
        "#
    )
}

const OVER: &str = "coordination over [NounPhrase, PrepositionPhrase] { coordinators: And, cost: 2, delimiter: ',' }";

fn format_error(coordination: &str) -> String {
    match Grammar::from_fug_str(&text(coordination)) {
        Err(LoadError::Format(message)) => message,
        other => panic!("{coordination}: expected a format error, got {other:?}"),
    }
}

#[test]
fn one_schema_is_declared_for_each_listed_category() {
    let grammar = Grammar::from_fug_str(&text(OVER)).unwrap();
    let separate = Grammar::from_fug_str(&text(
        "coordination NounPhrase { coordinators: And, cost: 2, delimiter: ',' }
            coordination PrepositionPhrase { coordinators: And, cost: 2, delimiter: ',' }",
    ))
    .unwrap();
    assert_eq!(grammar.schemas.len(), 2);
    assert_eq!(grammar.schemas, separate.schemas);
    let categories: Vec<_> = grammar
        .schemas
        .iter()
        .map(|schema| match schema {
            SchemaDecl::Coordination {
                category,
                cost,
                delimiter,
                ..
            } => (category.to_string(), *cost, *delimiter),
            other => panic!("expected a Coordination, got {other:?}"),
        })
        .collect();
    assert_eq!(
        categories,
        [
            ("NounPhrase".to_owned(), 2, Some(',')),
            ("PrepositionPhrase".to_owned(), 2, Some(','))
        ]
    );
}

#[test]
fn a_bound_grammar_coordinates_each_listed_category() {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(&text(OVER)).unwrap(),
        &lexicon,
        &FrameCatalog::default(),
    )
    .unwrap();
    for input in [
        "Your graveyard and your hand;",
        "From your graveyard and from your hand.",
    ] {
        let found = grammar
            .parse(&lexicon, input)
            .readings()
            .collect::<Vec<_>>();
        assert_eq!(found.len(), 1, "{input}: {found:#?}");
        assert_eq!(grammar.realize(&found[0], &lexicon).unwrap(), input);
    }
}

#[test]
fn a_list_of_fewer_than_two_categories_is_a_load_error() {
    for schema in [
        "coordination over [NounPhrase] { coordinators: And }",
        "coordination over [] { coordinators: And }",
    ] {
        let message = format_error(schema);
        assert!(
            message.contains("`coordination over [A, B, ...]`"),
            "{schema}: {message}"
        );
    }
}

#[test]
fn a_missing_list_is_a_load_error() {
    let message = format_error("coordination over { coordinators: And }");
    assert!(
        message.contains("`coordination over [A, B, ...]`"),
        "{message}"
    );
}

#[test]
fn a_category_named_twice_is_a_load_error() {
    let message = format_error("coordination over [NounPhrase, NounPhrase] { coordinators: And }");
    assert!(
        message.contains("`NounPhrase` is listed twice"),
        "{message}"
    );
}

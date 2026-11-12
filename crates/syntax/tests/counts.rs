//! Exact Derivation counts (`Parse::counts`) over small hand grammars:
//! checked against brute-force recounts and enumeration, and refused on a
//! forest with a unary cycle.

#[path = "support/forest_oracle.rs"]
mod forest_oracle;

use sylex_lexicon::Category;
use sylex_lexicon::Lexeme;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;

fn word(id: &str, lemma: &str, category: Category) -> Lexeme {
    Lexeme::invariant(
        id,
        lemma,
        category,
        Source {
            kind: SourceKind::Core,
            path: "tests/counts.rs".to_owned(),
            owner: "test".to_owned(),
        },
    )
}

/// `cast` is a noun and an adjective; `card` a noun only.
fn lexicon() -> Lexicon {
    Lexicon::new([
        word("n:cast", "cast", Category::Noun),
        word("a:cast", "cast", Category::Adjective),
        word("n:card", "card", Category::Noun),
    ])
    .unwrap()
}

fn bind(grammar: &str) -> CompiledGrammar {
    CompiledGrammar::bind(
        &Grammar::from_fug_str(grammar).unwrap(),
        &lexicon(),
        &FrameCatalog::default(),
    )
    .unwrap()
}

/// Every binary bracketing of a word list, each word a noun or an
/// adjective: Catalan-many groupings times the homograph choices.
const BRACKETS: &str = r"
    root Sentence
    category Sentence
    category Phrase
    Sentence: Sentence -> Phrase '.'
    Noun: Phrase -> Lex(Noun)
    Adjective: Phrase -> Lex(Adjective)
    Pair: Phrase -> first:Phrase second:Phrase
";

#[test]
fn counts_agree_with_recounts_and_enumeration() {
    let grammar = bind(BRACKETS);
    for (text, total) in [
        ("Card.", 1),
        ("Card card card.", 2),
        ("Cast card cast card.", 20),
        ("Card cast card cast card cast.", 42 * 8),
    ] {
        let parse = grammar.parse(&lexicon(), text);
        assert_eq!(parse.counts().unwrap().total(), total, "{text}");
        forest_oracle::check(&parse, u128::MAX).unwrap_or_else(|e| panic!("{text}: {e}"));
    }
}

/// Without its period a word list has no root and no Derivation, though its
/// Phrases are in the forest: no node is settled.
#[test]
fn nothing_is_settled_without_a_derivation() {
    let grammar = bind(BRACKETS);
    let parse = grammar.parse(&lexicon(), "Card card");
    let counts = parse.counts().unwrap();
    assert_eq!(counts.total(), 0);
    assert!(parse.forest.metrics().completed_nodes > 0);
    forest_oracle::check(&parse, u128::MAX).unwrap();
}

/// In *card card card* both groupings share the whole Phrase, which is
/// settled though it has two alternatives; each inner Pair is in one. A
/// Phrase after the first word spans the space before it.
#[test]
fn a_node_in_every_derivation_is_settled() {
    let grammar = bind(BRACKETS);
    let parse = grammar.parse(&lexicon(), "Card card card.");
    let counts = parse.counts().unwrap();
    let forest = &parse.forest;
    let phrase = grammar.category_id("Phrase").unwrap();
    let mut by_span = Vec::new();
    for node in 0..forest.metrics().completed_nodes {
        if forest.node_category(node) == phrase && counts.node(node) > 0 {
            by_span.push((
                forest.node_span(node),
                counts.node(node),
                counts.settled(node),
                forest.node_items(node).count(),
            ));
        }
    }
    by_span.sort_by_key(|(span, ..)| (span.start, span.end));
    assert_eq!(
        by_span,
        [
            (0..4, 2, true, 1),
            (0..9, 1, false, 1),
            (0..14, 2, true, 2),
            (4..9, 2, true, 1),
            (4..14, 1, false, 1),
            (9..14, 2, true, 1),
        ]
    );
}

/// `A` and `B` each derive the word and each other, so the forest holds a
/// unary cycle: the counts name it instead of counting.
#[test]
fn a_unary_cycle_is_an_error_naming_it() {
    let grammar = bind(
        r"
            root Sentence
            category A
            category B
            category Sentence
            SA: Sentence -> A '.'
            SB: Sentence -> B '.'
            AB: A -> B
            BA: B -> A
            AWord: A -> Lex(Noun)
            BWord: B -> Lex(Noun)
        ",
    );
    let parse = grammar.parse(&lexicon(), "Card.");
    let error = parse.counts().unwrap_err().to_string();
    assert!(
        error.starts_with("unary cycle in the Packed Forest: "),
        "{error}"
    );
    for step in ["A 0..4", "B 0..4", "`AB` 0..4", "`BA` 0..4"] {
        assert!(error.contains(step), "{error}");
    }
}

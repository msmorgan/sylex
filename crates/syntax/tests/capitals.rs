//! Capitals are positional (`docs/design/english.md` §7): a Production states
//! which children open with a capital, the parse admits a capitalized
//! spelling only there, and realization writes one only there. Derived from
//! Arcbound Wanderer's *Modular—Sunburst* and Roar of Jukai's *Splice onto
//! Arcane*.

use std::collections::BTreeMap;

use sylex_lexicon::Binding;
use sylex_lexicon::CardScope;
use sylex_lexicon::Category;
use sylex_lexicon::FeatureBundle;
use sylex_lexicon::Lexeme;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::LexicalValue;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_lexicon::WordForm;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Gap;
use sylex_syntax::Grammar;
use sylex_syntax::LoadError;
use sylex_syntax::Reading;
use sylex_syntax::ReadingNode;
use sylex_syntax::frames::ParameterPart;
use sylex_syntax::frames::SlotKind;

fn lexicon() -> Lexicon {
    let catalog = || Source {
        kind: SourceKind::Catalog,
        path: "tests/capitals.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new(vec![
        Lexeme::invariant("keyword:modular", "modular", Category::Keyword, catalog()),
        Lexeme::invariant("keyword:splice", "splice", Category::Keyword, catalog()),
        Lexeme::invariant("keyword:sunburst", "sunburst", Category::Noun, catalog()),
        Lexeme::invariant("noun:arcane", "arcane", Category::Noun, catalog()),
        Lexeme::invariant("noun:card", "card", Category::Noun, catalog()),
    ])
    .unwrap()
}

/// A line of two nouns opens with a capital, and a keyword's parameter
/// after its em dash opens with one.
const GRAMMAR: &str = r"
    root Line
    category Line
    category Item
    category KeywordParameters
    category Remnant
    slot KeywordPhrase = Item
    Pair: Line -> first:Item second:Item '.' { capital: [first] }
    Keyword: Line -> KeywordParameters
    Elided: Line -> first:KeywordParameters Remnant
    Noun: Item -> Lex(Noun)
    parameters KeywordParameters {
        head: Keyword,
        binding: Free,
        remnant: Remnant,
        capital_after: ['—'],
    }
";

fn frames() -> FrameCatalog {
    let mut frames = FrameCatalog::default();
    frames.parameters.insert(
        "keyword:modular".into(),
        vec![vec![
            ParameterPart::Head,
            ParameterPart::Mark('—', Binding::Bound),
            ParameterPart::Slot(SlotKind::KeywordPhrase),
        ]],
    );
    frames.parameters.insert(
        "keyword:splice".into(),
        vec![vec![
            ParameterPart::Head,
            ParameterPart::Mark('—', Binding::Bound),
            ParameterPart::Word("noun:arcane".into()),
        ]],
    );
    frames
}

fn bind(text: &str, lexicon: &Lexicon) -> Result<CompiledGrammar, LoadError> {
    CompiledGrammar::bind(&Grammar::from_fug_str(text).unwrap(), lexicon, &frames())
}

fn count(text: &str) -> u128 {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let parse = grammar.parse(&lexicon, text);
    if parse.derivation_count() > 0 {
        parse.verify_realization(&lexicon).unwrap();
    }
    parse.derivation_count()
}

#[test]
fn a_capital_where_no_position_states_one_has_no_reading() {
    assert_eq!(count("Card card."), 1);
    assert_eq!(count("Card Card."), 0);
}

#[test]
fn lower_case_where_a_position_states_a_capital_has_no_reading() {
    assert_eq!(count("card card."), 0);
}

#[test]
fn a_root_that_would_open_with_a_capital_has_no_reading() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let parse = |text: &str| {
        grammar
            .parse_category(&lexicon.analyze(text), "Item")
            .unwrap()
            .derivation_count()
    };
    assert_eq!(parse("card"), 1);
    assert_eq!(parse("Card"), 0);
    // `Keyword` states no capital, so a keyword line opens in lower case.
    assert_eq!(count("Modular—Sunburst"), 0);
}

#[test]
fn capital_after_marks_the_part_after_its_mark() {
    // A slot.
    assert_eq!(count("modular—Sunburst"), 1);
    assert_eq!(count("modular—sunburst"), 0);
    // A marker word.
    assert_eq!(count("splice—Arcane"), 1);
    assert_eq!(count("splice—arcane"), 0);
    // The pattern's head-elided remnant keeps the capital after its mark. A
    // remnant stands only after the phrase whose head it elides.
    assert_eq!(count("modular—Sunburst—Sunburst"), 1);
    assert_eq!(count("modular—Sunburst—sunburst"), 0);
    assert_eq!(count("splice—Arcane—Arcane"), 1);
    assert_eq!(count("splice—Arcane—arcane"), 0);
}

#[test]
fn a_capital_naming_no_child_is_a_load_error() {
    let lexicon = lexicon();
    let text = GRAMMAR.replace("{ capital: [first] }", "{ capital: [third] }");
    match bind(&text, &lexicon) {
        Err(LoadError::Invalid(message)) => {
            assert!(
                message.contains("unknown capital child `third`"),
                "{message}"
            );
        }
        other => panic!("expected a load error, got {other:?}"),
    }
}

#[test]
fn a_hand_built_reading_realizes_with_the_derived_capitals() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let node = |production: &str, category: &str, children| {
        Reading::Node(ReadingNode {
            production: production.into(),
            category: category.into(),
            features: BTreeMap::new(),
            gap: Gap::None,
            selected_frame: None,
            children,
        })
    };
    // No leaf states a case.
    let card = || {
        node(
            "Noun",
            "Item",
            vec![Reading::Leaf(LexicalReading::Word(LexicalValue {
                lexeme: "noun:card".into(),
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
            }))],
        )
    };
    let reading = node("Pair", "Line", vec![card(), card()]);
    grammar
        .admit(&reading, &lexicon, &CardScope::default())
        .unwrap();
    let text = grammar.realize(&reading, &lexicon).unwrap();
    assert_eq!(text, "Card card.");
    let parse = grammar.parse(&lexicon, &text);
    parse.verify_realization(&lexicon).unwrap();
    assert!(parse.contains(&reading));
}

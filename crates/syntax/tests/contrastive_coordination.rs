//! A Coordination schema's `negator` makes a contrastive coordination, *X but
//! not Y* (CGEL, Ch. 15 §2.6, p. 1313, [60iia]): only the binary coordination,
//! plain and with the `delimiter`, with the negator as a `modifier` child
//! between the coordinator and the last conjunct, and no serial form. It is a
//! load error with `serial`, `unlike`, `link`, `remnant` or `raising_delimiter`.
//! The grammar coordinates Preposition Phrases as Haakon, Stromgald Scourge
//! does: *from your graveyard, but not from your hand*.

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

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/contrastive_coordination.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("p:from", "from", Category::Preposition, core()),
        Lexeme::invariant("d:your", "your", Category::Determinative, core()),
        Lexeme::invariant("c:but", "but", Category::Coordinator, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
        Lexeme::invariant("a:not", "not", Category::Adverb, core()),
        Lexeme::noun(
            "n:graveyard",
            "graveyard",
            vec![Countability::Count],
            core(),
        ),
        Lexeme::noun("n:hand", "hand", vec![Countability::Count], core()),
        Lexeme::noun("n:library", "library", vec![Countability::Count], core()),
    ])
    .unwrap()
}

fn text(coordination: &str) -> String {
    format!(
        r#"
            root Sentence
            category NounPhrase
            feature Kind = Place | Zone
            category PrepositionPhrase [Kind]
            category Sentence
            word But = "c:but"
            word And = "c:and"
            word Not = "a:not"
            Determined: NounPhrase -> determiner:Lex(Determinative) Lex(Noun)
            Preposed: PrepositionPhrase -> Lex(Preposition) NounPhrase
            Fragment: Sentence -> PrepositionPhrase '.' {{ capital: [head] }}
            {coordination}
        "#
    )
}

fn grammar(
    lexicon: &Lexicon,
    coordination: &str,
) -> Result<CompiledGrammar, sylex_syntax::LoadError> {
    CompiledGrammar::bind(
        &Grammar::from_fug_str(&text(coordination)).unwrap(),
        lexicon,
        &FrameCatalog::default(),
    )
}

const CONTRASTIVE: &str =
    "coordination PrepositionPhrase { coordinators: But, negator: Not, delimiter: ',' }";

/// The Readings of `input`, each checked for Admission and round-trip law 1.
fn readings(coordination: &str, input: &str) -> Vec<Reading> {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, coordination).unwrap();
    let readings = grammar
        .parse(&lexicon, input)
        .readings()
        .collect::<Vec<_>>();
    for reading in &readings {
        assert_eq!(
            grammar.admit(reading, &lexicon, &sylex_lexicon::CardScope::default()),
            Ok(()),
            "{input}"
        );
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), input);
    }
    readings
}

fn productions(reading: &Reading, out: &mut Vec<String>) {
    if let Reading::Node(node) = reading {
        out.push(node.production.to_string());
        for child in &node.children {
            productions(child, out);
        }
    }
}

#[test]
fn a_negated_last_conjunct_parses_with_and_without_the_comma() {
    for input in [
        "From your graveyard, but not from your hand.",
        "From your graveyard but not from your hand.",
    ] {
        let found = readings(CONTRASTIVE, input);
        assert_eq!(found.len(), 1, "{input}");
        let mut ids = Vec::new();
        productions(&found[0], &mut ids);
        assert!(
            ids.iter().any(|id| id.starts_with("Coordination:")),
            "{input}: {ids:?}"
        );
    }
}

#[test]
fn the_comma_is_the_schemas_delimiter_alone() {
    let plain = "coordination PrepositionPhrase { coordinators: But, negator: Not }";
    assert_eq!(
        readings(plain, "From your graveyard but not from your hand.").len(),
        1
    );
    assert_eq!(
        readings(plain, "From your graveyard, but not from your hand.").len(),
        0
    );
}

#[test]
fn the_negator_is_required() {
    assert_eq!(
        readings(CONTRASTIVE, "From your graveyard but from your hand.").len(),
        0
    );
    assert_eq!(
        readings(CONTRASTIVE, "From your graveyard, but from your hand.").len(),
        0
    );
    // Without a negator, the same schema does coordinate them.
    let ordinary = "coordination PrepositionPhrase { coordinators: But, delimiter: ',' }";
    assert_eq!(
        readings(ordinary, "From your graveyard but from your hand.").len(),
        1
    );
}

#[test]
fn no_serial_form_exists() {
    for input in [
        "From your graveyard, from your hand, but not from your library.",
        "From your graveyard, from your hand but not from your library.",
        "From your graveyard but not from your hand but not from your library.",
    ] {
        assert_eq!(readings(CONTRASTIVE, input).len(), 0, "{input}");
    }
}

#[test]
fn a_negator_with_another_shape_is_a_load_error() {
    let lexicon = lexicon();
    for extra in [
        "serial: true",
        "unlike: Kind",
        "remnant: NounPhrase",
        "raising_delimiter: ','",
    ] {
        let schema = format!(
            "coordination PrepositionPhrase {{ coordinators: But, negator: Not, {extra} }}"
        );
        let result = grammar(&lexicon, &schema);
        let error = match result {
            Ok(_) => panic!("{extra}: expected a load error"),
            Err(error) => error.to_string(),
        };
        assert!(error.contains("a `negator` with"), "{extra}: {error}");
    }
    // A Linked Coordination has no coordinator to put the negator after.
    let linked = "coordination PrepositionPhrase { link: NounPhrase, negator: Not }";
    let error = grammar(&lexicon, linked).err().map(|e| e.to_string());
    assert!(
        error.is_some_and(|e| e.contains("a `negator` with")),
        "link: expected the negator error"
    );
}

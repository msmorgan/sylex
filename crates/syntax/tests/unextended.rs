//! A Production's `unextended` child does not end in a Preposition Phrase that
//! the Production, or a sibling with the same children up to and including
//! it, selects as a `PP` Complement right after it (an existential's
//! Extension after its Displaced Subject, CGEL Ch. 16 §6.2.1, p. 1394). A
//! phrase the Complement cannot take stays a postmodifier.

use sylex_lexicon::Category;
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
        path: "tests/unextended.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("n:card", "card", Category::Noun, core()),
        Lexeme::invariant("p:in", "in", Category::Preposition, core()),
        Lexeme::invariant("p:of", "of", Category::Preposition, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    prepositions: {
        "p:in": [(complement: NP, hosts: Nominal)],
        "p:of": [(complement: NP, hosts: Nominal)],
    },
)"#;

fn grammar(lexicon: &Lexicon, extended: &str) -> Result<CompiledGrammar, sylex_syntax::LoadError> {
    let text = format!(
        r#"
root Sentence
category NounPhrase
category Predicate
category PrepositionPhrase
category Sentence
word In = "p:in"
slot NP = NounPhrase
slot PP = PrepositionPhrase
Statement: Sentence -> Predicate '.'
Noun: NounPhrase -> Lex(Noun)
Bare: Predicate -> subject:NounPhrase {{ unextended: [subject] }}
{extended}
valence PrepositionPhrase {{ head: Preposition }}
adjunct PrepositionPhrase {{ host: NounPhrase, side: After, host_kind: Nominal }}
        "#
    );
    CompiledGrammar::bind(
        &Grammar::from_fug_str(&text).unwrap(),
        lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
}

const EXTENDED: &str =
    "Extended: Predicate -> subject:NounPhrase complement:Selected(PrepositionPhrase, In)";

#[test]
fn a_phrase_the_sibling_selects_is_its_complement_only() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, EXTENDED).unwrap();
    for text in ["Card in card.", "Card of card."] {
        let found = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
        assert_eq!(found.len(), 1, "{text}: {found:#?}");
        assert_eq!(grammar.admit(&found[0], &lexicon), Ok(()), "{text}");
    }
    // *in card* is the extension, never the postmodifier of the bare subject.
    let found = grammar
        .parse(&lexicon, "Card in card.")
        .readings()
        .collect::<Vec<_>>();
    let predicate = found[0].node().unwrap().children[0].node().unwrap();
    assert_eq!(predicate.production.as_str(), "Extended");
    // *of card* is no extension: it postmodifies the bare subject.
    let found = grammar
        .parse(&lexicon, "Card of card.")
        .readings()
        .collect::<Vec<_>>();
    let predicate = found[0].node().unwrap().children[0].node().unwrap();
    assert_eq!(predicate.production.as_str(), "Bare");
}

#[test]
fn a_child_before_a_selected_complement_does_not_end_in_one() {
    let lexicon = lexicon();
    let closed = "Extended: Predicate -> subject:NounPhrase complement:Selected(PrepositionPhrase, In) { unextended: [subject] }";
    let open = grammar(&lexicon, EXTENDED).unwrap();
    let grammar = grammar(&lexicon, closed).unwrap();
    // Open, *card in card* can be the subject before the extension *in card*.
    assert_eq!(
        open.parse(&lexicon, "Card in card in card.")
            .readings()
            .collect::<Vec<_>>()
            .len(),
        2
    );
    // Closed, the first *in* phrase begins the extension.
    let found = grammar
        .parse(&lexicon, "Card in card in card.")
        .readings()
        .collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(grammar.admit(&found[0], &lexicon), Ok(()));
    let subject = found[0].node().unwrap().children[0]
        .node()
        .unwrap()
        .children[0]
        .node()
        .unwrap();
    assert_eq!(subject.production.as_str(), "Noun");
    // A phrase the extension cannot take still ends the subject.
    let found = grammar
        .parse(&lexicon, "Card of card in card.")
        .readings()
        .collect::<Vec<_>>();
    assert!(
        found.iter().any(|reading| {
            let predicate = reading.node().unwrap().children[0].node().unwrap();
            predicate.production.as_str() == "Extended"
                && predicate.children[0].node().unwrap().production.as_str() != "Noun"
        }),
        "{found:#?}"
    );
}

#[test]
fn an_unextended_child_needs_a_sibling_with_a_selected_complement() {
    assert!(grammar(&lexicon(), "").is_err());
}

#[test]
fn an_unextended_child_before_no_selected_complement_is_a_load_error() {
    let lexicon = lexicon();
    // A first child followed by a Constituent is not `unextended` unless a
    // selected `PP` Complement follows it in some sibling.
    let first =
        "Pair: Sentence -> subject:NounPhrase object:NounPhrase '.' { unextended: [subject] }";
    assert!(grammar(&lexicon, &format!("{EXTENDED}\n{first}")).is_err());
    let unknown = "Named: Predicate -> subject:NounPhrase complement:Selected(PrepositionPhrase, In) { unextended: [nonesuch] }";
    assert!(grammar(&lexicon, unknown).is_err());
}

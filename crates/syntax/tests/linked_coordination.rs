//! A Linked Coordination has no Coordinator: the Conjunct after each link is
//! of its `link` Category, marked by its own initial connective (*Draw a
//! card, then discard a card*; CGEL, Ch. 15 §2.10, pp. 1319–1320). Each chain
//! has one tree, its serial, whose middle Conjuncts may begin with the
//! connective as a front Adjunct, and a child declared `unlinked` (a
//! subordinate clause) is never one.

use sylex_lexicon::Category;
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
        path: "tests/linked_coordination.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("pro:it", "it", Category::Pronoun, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
        Lexeme::invariant("adv:then", "then", Category::Adverb, core()),
        Lexeme::invariant("p:until", "until", Category::Preposition, core()),
        Lexeme::verb("v:draw", "draw", core()),
        Lexeme::verb("v:discard", "discard", core()),
    ])
    .unwrap()
}

fn grammar_text(linked: &str) -> String {
    format!(
        r#"
            root Sentence
            category Clause
            category Complement
            category LinkedClause : Clause
            category Sentence
            word And = "c:and"
            word Then = "adv:then"
            word Until = "p:until"
            Declarative: Sentence -> clause:Clause '.'
            Intransitive: Clause -> subject:Lex(Pronoun) Lex(Verb)
            Front: Clause -> adjunct:Word(Then) Clause
            Linked: LinkedClause -> adjunct:Word(Then) Clause
            Until: Clause -> Clause marker:Word(Until) complement:Complement
            Complement: Complement -> Clause {{ unlinked: [head] }}
            coordination Clause {{ coordinators: And, delimiter: ',' }}
            {linked}
        "#
    )
}

fn grammar(lexicon: &Lexicon, linked: &str) -> Result<CompiledGrammar, String> {
    let grammar = Grammar::from_fug_str(&grammar_text(linked)).map_err(|e| format!("{e:?}"))?;
    CompiledGrammar::bind(&grammar, lexicon, &FrameCatalog::default()).map_err(|e| format!("{e:?}"))
}

const LINKED: &str = r"coordination Clause { link: LinkedClause, delimiter: ',', delimited }";

fn uses(reading: &Reading, production: &str) -> bool {
    reading.node().is_some_and(|node| {
        node.production.as_str() == production || node.children.iter().any(|c| uses(c, production))
    })
}

fn readings(text: &str) -> Vec<Reading> {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, LINKED).unwrap();
    let parse = grammar.parse(&lexicon, text);
    let readings = parse.readings().collect::<Vec<_>>();
    assert_eq!(parse.verify_realization(&lexicon), Ok(()), "{text}");
    for reading in &readings {
        assert_eq!(
            grammar.admit(reading, &lexicon, &sylex_lexicon::CardScope::default()),
            Ok(()),
            "{text}"
        );
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    readings
}

#[test]
fn a_linked_conjunct_marks_its_link() {
    let readings = readings("it draws, then it discards.");
    assert_eq!(readings.len(), 1, "{readings:#?}");
    assert!(uses(&readings[0], "Linked"));
    // The link needs its connective, and here its comma.
    for text in ["it draws, it discards.", "it draws then it discards."] {
        assert_eq!(readings_of(text).len(), 0, "{text}");
    }
}

#[test]
fn a_chain_has_one_tree() {
    for text in [
        "it draws, then it discards, then it draws.",
        "it draws, it discards, then it draws.",
        "it draws, then it discards, it draws, then it discards.",
        "it draws, it discards, it draws, then it discards.",
    ] {
        let readings = readings(text);
        assert_eq!(readings.len(), 1, "{text}: {readings:#?}");
    }
}

#[test]
fn a_linked_conjunct_may_hold_a_coordination_by_a_coordinator() {
    // *then [it discards and it draws]*, and *[it draws, then it discards]
    // and it draws*: two groupings, both admitted.
    assert_eq!(
        readings("it draws, then it discards and it draws.").len(),
        2
    );
}

#[test]
fn an_unlinked_child_is_no_linked_coordination() {
    // *[it draws until it discards], then it draws*, never *it draws until
    // [it discards, then it draws]*.
    let readings = readings("it draws until it discards, then it draws.");
    assert_eq!(readings.len(), 1, "{readings:#?}");
}

#[test]
fn a_coordination_names_its_coordinators_or_its_link() {
    let lexicon = lexicon();
    for schema in [
        r"coordination Clause { coordinators: And, link: LinkedClause }",
        r"coordination Clause",
    ] {
        assert!(grammar(&lexicon, schema).is_err(), "{schema}");
    }
}

fn readings_of(text: &str) -> Vec<Reading> {
    let lexicon = lexicon();
    grammar(&lexicon, LINKED)
        .unwrap()
        .parse(&lexicon, text)
        .readings()
        .collect::<Vec<_>>()
}

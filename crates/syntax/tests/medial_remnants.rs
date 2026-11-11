//! Medial remnants: a coordination at the level of its final conjunct, with
//! an element at the beginning of each medial conjunct elided and recovered
//! from the initial one (CGEL, Ch. 15 §3.6, pp. 1335–1336, [31]).

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
        path: "tests/medial_remnants.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::verb("v:be", "be", core()),
        Lexeme::verb("v:have", "have", core()),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:be": [(family: Copular, slots: [(relation: PredicativeComplement, fill: Pred)])],
        "v:have": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
    },
)"#;

fn count(text: &str) -> usize {
    let source = r#"
        root Sentence
        feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
        category NounPhrase
        category Predicative
        category PredicativeRemnant
        category Sentence
        category VerbPhrase [VerbForm]
        class Coordinator = ["c:and"]
        slot NP = NounPhrase
        slot Pred = Predicative
        Imperative: Sentence -> VerbPhrase '.' where head.VerbForm = Plain
        Noun: NounPhrase -> Lex(Noun)
        NominalPredicative: Predicative -> NounPhrase
        valence VerbPhrase { head: Verb, remnants: [Pred -> PredicativeRemnant] }
        coordination VerbPhrase { coordinators: Coordinator, remnant: PredicativeRemnant }
    "#;
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(source).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap();
    let found = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
    for reading in &found {
        assert_eq!(
            grammar.admit(reading, &lexicon, &BTreeSet::new()),
            Ok(()),
            "{text}"
        );
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    found.len()
}

#[test]
fn a_medial_conjunct_may_elide_the_initial_conjuncts_copula() {
    // *is enchanted, [equipped], and has a counter on it* ([31ii]).
    assert_eq!(count("Be card, card, and have card."), 1);
    assert_eq!(count("Be card, card, card, and have card."), 1);
}

#[test]
fn a_remnant_is_never_the_initial_or_last_conjunct() {
    // A coordination whose later conjuncts were all remnants would be the
    // coordination of their Predicatives under the one copula.
    assert_eq!(count("Be card and card."), 0);
    assert_eq!(count("Be card, have card, and card."), 0);
    assert_eq!(count("Card, be card, and have card."), 0);
}

#[test]
fn the_elided_head_declares_the_remnants_frame() {
    // *have* declares no Copular frame, so it is not the head a Predicative
    // remnant elides.
    assert_eq!(count("Have card, card, and be card."), 0);
}

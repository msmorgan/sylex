//! Idiomatic coordinates: a pro-form after a declared coordinator ends a
//! coordination, standing for further conjuncts of its kind (*and so on*;
//! CGEL, Ch. 17 §7.7.3(e), p. 1540).

use sylex_lexicon::Category;
use sylex_lexicon::Countability;
use sylex_lexicon::Lexeme;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_lexicon::SurfaceStructure;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/idiomatic_coordinates.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let mut so_on = Lexeme::invariant("adv:so on", "so on", Category::Adverb, core());
    so_on.surface_structure = SurfaceStructure::Multiword;
    Lexicon::new([
        Lexeme::verb("v:give", "give", core()),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
        so_on,
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
        Lexeme::invariant("c:or", "or", Category::Coordinator, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:give": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
    },
)"#;

fn count(text: &str) -> usize {
    let source = r#"
        root Sentence
        feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
        category IdiomaticCoordinate
        category NounPhrase
        category Sentence
        category VerbPhrase [VerbForm]
        word And = "c:and"
        class CoordinateAdverb = ["adv:so on"]
        class Coordinator = ["c:and", "c:or"]
        slot NP = NounPhrase
        Imperative: Sentence -> VerbPhrase '.' where head.VerbForm = Plain
        Noun: NounPhrase -> Lex(Noun)
        IdiomaticCoordinate: IdiomaticCoordinate -> Class(CoordinateAdverb)
        valence VerbPhrase { head: Verb }
        coordination NounPhrase { coordinators: Coordinator, delimiter: ',' }
        // A second schema whose coordinators admit *and* derives no second
        // tree of the idiomatic coordinate.
        coordination NounPhrase { coordinators: Coordinator, cost: 2 }
        idiomatic_coordinate And = IdiomaticCoordinate
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
        assert_eq!(grammar.admit(reading, &lexicon), Ok(()), "{text}");
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    found.len()
}

#[test]
fn an_idiomatic_coordinate_ends_a_coordination() {
    assert_eq!(count("Give card, card, and so on."), 1);
    assert_eq!(count("Give card and so on."), 1);
}

#[test]
fn it_follows_only_its_coordinator() {
    assert_eq!(count("Give card, card, or so on."), 0);
}

#[test]
fn it_is_only_the_last_coordinate() {
    assert_eq!(count("Give so on."), 0);
    assert_eq!(count("Give so on and card."), 0);
    assert_eq!(count("Give card, so on, and card."), 0);
}

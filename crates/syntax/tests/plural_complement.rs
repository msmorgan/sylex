//! A preposition whose frames carry `plural_complement` takes a plural
//! Complement (*among*; CGEL, Ch. 7 §5.1(a), p. 636): a coordination itself,
//! or else a Noun Phrase that is not Singular. A singular Noun Phrase is
//! refused even over a coordinated Nominal, and a keyword phrase, which has
//! no Number, only as a coordination. Grounded in Abzan Charm's *distribute
//! two +1/+1 counters among one or two target creatures*, Tsabo's Assassin's
//! *among all permanents or a color tied for most common* and Elspeth
//! Resplendent's *a counter from among flying, first strike, lifelink, or
//! vigilance*.

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
    let source = |kind| Source {
        kind,
        path: "tests/plural_complement.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let core = || source(SourceKind::Core);
    let mut lexemes = vec![
        Lexeme::verb("v:distribute", "distribute", source(SourceKind::Catalog)),
        Lexeme::verb("v:choose", "choose", core()),
        Lexeme::invariant("p:among", "among", Category::Preposition, core()),
        Lexeme::invariant("d:a", "a", Category::Determinative, core()),
        Lexeme::invariant("c:or", "or", Category::Coordinator, core()),
        Lexeme::invariant(
            "k:flying",
            "flying",
            Category::Keyword,
            source(SourceKind::Catalog),
        ),
        Lexeme::invariant(
            "k:vigilance",
            "vigilance",
            Category::Keyword,
            source(SourceKind::Catalog),
        ),
    ];
    for noun in ["counter", "creature", "permanent", "color"] {
        lexemes.push(Lexeme::noun(
            format!("n:{noun}"),
            noun,
            vec![Countability::Count],
            core(),
        ));
    }
    Lexicon::new(lexemes).unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:distribute": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "v:choose": [(family: Intransitive, slots: [])],
    },
    prepositions: {
        "p:among": [
            (complement: NP, hosts: Predicate, plural_complement: true),
            (complement: KeywordPhrase, hosts: Predicate, plural_complement: true),
        ],
    },
)"#;

// *A* fixes a singular Noun Phrase over its Nominal, coordinated or not; a
// bare Nominal is a plural Noun Phrase.
const GRAMMAR: &str = r#"
    root Sentence
    feature Number = Singular | Plural
    feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
    category KeywordPhrase
    category Nominal [Number]
    category NounPhrase [Number]
    category ComplementNounPhrase : NounPhrase [Number]
    category PrepositionPhrase
    category Sentence
    category VerbPhrase [VerbForm]
    word A = "d:a"
    class Coordinator = ["c:or"]
    slot NP = ComplementNounPhrase
    slot KeywordPhrase = KeywordPhrase
    Imperative: Sentence -> clause:VerbPhrase '.' where clause.VerbForm = Plain
    Wrapped: ComplementNounPhrase -> NounPhrase
    Singular: NounPhrase -> determiner:Word(A) Nominal where $.Number = Singular
    BarePlural: NounPhrase -> Nominal where head.Number = Plural
    Keyword: KeywordPhrase -> Lex(Keyword)
    coordination NounPhrase { coordinators: Coordinator }
    coordination Nominal { coordinators: Coordinator }
    coordination KeywordPhrase { coordinators: Coordinator }
    valence VerbPhrase { head: Verb }
    valence Nominal { head: Noun }
    valence PrepositionPhrase { head: Preposition }
    adjunct PrepositionPhrase { host: VerbPhrase, side: After, host_kind: Predicate }
"#;

/// How many Readings `text` has, each checked for Admission and round-trip
/// law 1.
fn count(text: &str) -> usize {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(GRAMMAR).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap();
    let readings = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
    for reading in &readings {
        assert_eq!(
            grammar.admit(reading, &lexicon, &sylex_lexicon::CardScope::default()),
            Ok(()),
            "{text}"
        );
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    readings.len()
}

#[test]
fn a_plural_noun_phrase_is_admitted_and_a_singular_one_refused() {
    assert_eq!(count("Distribute counters among creatures."), 1);
    assert_eq!(count("Distribute counters among a creature."), 0);
}

#[test]
fn a_coordination_of_noun_phrases_is_admitted_whatever_its_number() {
    assert_eq!(
        count("Distribute counters among a permanent or a color."),
        1
    );
}

#[test]
fn a_singular_noun_phrase_over_a_coordinated_nominal_is_refused() {
    assert_eq!(
        count("Distribute counters among a creature or permanent."),
        0
    );
}

#[test]
fn a_keyword_is_admitted_only_as_a_coordination() {
    assert_eq!(count("Choose among flying."), 0);
    assert_eq!(count("Choose among flying or vigilance."), 1);
}

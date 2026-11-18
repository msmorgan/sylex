//! A `slot Kind = Category where ...` declaration's equations may name the
//! derived phrase `$` as well as the slot child `slot`: a temporal
//! preposition's phrase takes its Clause's time (*Whenever you cast a
//! noncreature spell*, Kykar, Zephyr Awakener), which a consumer's equation
//! reads from the phrase. With no such equation the phrase carries no time.

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
        path: "tests/slot_phrase_equations.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("p:when", "when", Category::Preposition, core()),
        Lexeme::invariant("a:cast", "cast", Category::Adverb, core()),
        Lexeme::invariant("a:casts", "casts", Category::Adverb, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    prepositions: {
        "p:when": [(complement: Clause(Temporal), hosts: Predicate)],
    },
)"#;

fn grammar(lexicon: &Lexicon, equations: &str) -> CompiledGrammar {
    let source = format!(
        r#"
            root Sentence
            feature Time = Past | NonPast
            category PrepositionPhrase [Time]
            category Sentence
            category TemporalClause [Time]
            word Cast = "a:cast"
            word Casts = "a:casts"
            slot TemporalClause = TemporalClause {equations}
            Old: TemporalClause -> Word(Cast) where $.Time = Past
            Current: TemporalClause -> Word(Casts) where $.Time = NonPast
            Free: Sentence -> PrepositionPhrase ';'
            AsPast: Sentence -> pp:PrepositionPhrase '.' where pp.Time := Past
            AsNonPast: Sentence -> pp:PrepositionPhrase '!' where pp.Time := NonPast
            valence PrepositionPhrase {{ head: Preposition }}
        "#
    );
    CompiledGrammar::bind(
        &Grammar::from_fug_str(&source).unwrap(),
        lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap()
}

fn accepted(equations: &str, text: &str) -> bool {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, equations);
    grammar.parse(&lexicon, text).readings().next().is_some()
}

const TAKES_TIME: &str = "where $.Time = slot.Time";

#[test]
fn a_phrase_takes_its_slots_feature() {
    assert!(accepted(TAKES_TIME, "when cast."));
    assert!(accepted(TAKES_TIME, "when casts!"));
    assert!(!accepted(TAKES_TIME, "when cast!"));
    assert!(!accepted(TAKES_TIME, "when casts."));
    assert!(accepted(TAKES_TIME, "when cast;"));
}

#[test]
fn without_the_equation_the_phrase_carries_no_time() {
    assert!(accepted("", "when cast;"));
    assert!(!accepted("", "when cast."));
    assert!(!accepted("", "when casts!"));
}

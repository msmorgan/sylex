//! A mixed feature (`feature F = ... { mixed: Value }`): every coordination
//! of a Category that carries it has its conjuncts' common value, or the
//! mixed one when any two of them differ, in whatever order. A conjunct
//! without a value decides nothing. The grammar here mixes the time of
//! coordinated Verb Phrases, past where each is (*you gained life*), as the
//! English grammar's `Time` is.

use std::collections::BTreeSet;

use sylex_lexicon::Category;
use sylex_lexicon::Countability;
use sylex_lexicon::Lexeme;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_lexicon::WordForm;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;
use sylex_syntax::Reading;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/mixed_features.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let mut gain = Lexeme::verb("v:gain", "gain", core());
    for slot in &mut gain.forms {
        if matches!(slot.form, WordForm::Preterite | WordForm::PastParticiple) {
            slot.surface = Some("gained".to_owned());
        }
    }
    // *Would*, a modal's preterite, which has no tense.
    let mut would = Lexeme::verb("v:will", "will", core());
    would.properties.modal = true;
    would
        .forms
        .retain(|slot| matches!(slot.form, WordForm::Present | WordForm::Preterite));
    for slot in &mut would.forms {
        if slot.form == WordForm::Preterite {
            slot.surface = Some("would".to_owned());
            slot.features.tense = None;
        }
    }
    Lexicon::new([
        gain,
        would,
        Lexeme::noun("n:life", "life", vec![Countability::Mass], core()),
        Lexeme::invariant("pr:you", "you", Category::Pronoun, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:gain": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "v:will": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
    },
)"#;

fn source(feature: &str, coordination: &str) -> String {
    format!(
        r#"
            root Sentence
            feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
            {feature}
            category Clause
            category NounPhrase
            category Sentence
            category VerbPhrase [VerbForm, Time]
            word And = "c:and"
            word You = "pr:you"
            slot NP = NounPhrase
            Noun: NounPhrase -> Lex(Noun)
            valence VerbPhrase {{ head: Verb }}
            Declarative: Sentence -> clause:Clause '.' {{ capital: [clause] }}
            PastClause: Clause -> subject:Word(You) VerbPhrase
                where head.VerbForm = Finite, head.Time := Past
            NonPastClause: Clause -> subject:Word(You) VerbPhrase
                where head.VerbForm = Finite, head.Time := NonPast
            MixedClause: Clause -> subject:Word(You) VerbPhrase
                where head.VerbForm = Finite, head.Time := Mixed
            UntimedClause: Clause -> subject:Word(You) VerbPhrase
                where head.VerbForm = Finite, head.Time != Past | Mixed | NonPast
            {coordination}
        "#
    )
}

const MIXED: &str = "feature Time = Past | Mixed | NonPast { mixed: Mixed }";
const COORDINATION: &str = "coordination VerbPhrase { coordinators: And }";

fn grammar(
    lexicon: &Lexicon,
    feature: &str,
    coordination: &str,
) -> Result<CompiledGrammar, String> {
    let grammar =
        Grammar::from_fug_str(&source(feature, coordination)).map_err(|e| format!("{e:?}"))?;
    CompiledGrammar::bind(
        &grammar,
        lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .map_err(|e| format!("{e:?}"))
}

fn clause(reading: &Reading) -> Option<String> {
    let Reading::Node(node) = reading else {
        return None;
    };
    let name = node.production.to_string();
    if name.ends_with("Clause") {
        return Some(name);
    }
    node.children.iter().find_map(clause)
}

/// The Clause Productions whose time the text's Readings have.
fn times(text: &str) -> BTreeSet<String> {
    times_under(COORDINATION, text)
}

fn times_under(coordination: &str, text: &str) -> BTreeSet<String> {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, MIXED, coordination).unwrap();
    grammar
        .parse(&lexicon, text)
        .readings()
        .map(|reading| clause(&reading).expect("a Clause"))
        .collect()
}

fn one(name: &str) -> BTreeSet<String> {
    BTreeSet::from([name.to_owned()])
}

#[test]
fn conjuncts_that_differ_anywhere_are_mixed() {
    for text in [
        "You gained life, gain life, and gain life.",
        "You gain life, gained life, and gain life.",
        "You gain life, gain life, and gained life.",
        "You gained life and gain life.",
    ] {
        assert_eq!(times(text), one("MixedClause"), "{text}");
    }
}

#[test]
fn a_mixed_feature_folds_beside_a_resolved_one() {
    // A schema's `resolve` feature (every conjunct but the last shares it)
    // and the mixed one in one coordination, as Oracle's disjunctive Verb
    // Phrases resolve VerbKind beside Time.
    let both = "coordination VerbPhrase { coordinators: And, resolve: [VerbForm -> Plain] }";
    for text in [
        "You gained life, gain life, and gain life.",
        "You gain life, gain life, and gained life.",
    ] {
        assert_eq!(times_under(both, text), one("MixedClause"), "{text}");
    }
    assert_eq!(
        times_under(both, "You gained life, gained life, and gained life."),
        one("PastClause")
    );
}

#[test]
fn conjuncts_that_agree_share_their_value() {
    assert_eq!(
        times("You gained life, gained life, and gained life."),
        one("PastClause")
    );
    assert_eq!(
        times("You gain life, gain life, and gain life."),
        one("NonPastClause")
    );
}

#[test]
fn a_conjunct_without_a_value_decides_nothing() {
    assert_eq!(
        times("You would life, gained life, and would life."),
        one("PastClause")
    );
    assert_eq!(
        times("You would life and would life."),
        one("UntimedClause")
    );
}

#[test]
fn a_misdeclared_mixed_feature_is_a_load_error() {
    let lexicon = lexicon();
    for (feature, coordination, defect) in [
        (
            "feature Time = Past | Mixed | NonPast { mixed: Mixed, local }",
            COORDINATION,
            "a local feature is never mixed",
        ),
        (
            "feature Time = Past | Mixed | NonPast { mixed: Both }",
            COORDINATION,
            "`mixed: Both` names no value of the feature",
        ),
        (
            MIXED,
            "coordination VerbPhrase { coordinators: And, unshared: [Time] }",
            "names `Time`, which every coordination mixes",
        ),
        (
            MIXED,
            "coordination VerbPhrase { coordinators: And, resolve: [Time -> Mixed] }",
            "names `Time`, which every coordination mixes",
        ),
    ] {
        let Err(error) = grammar(&lexicon, feature, coordination) else {
            panic!("{feature} {coordination}: loads");
        };
        assert!(error.contains(defect), "{error}");
    }
}

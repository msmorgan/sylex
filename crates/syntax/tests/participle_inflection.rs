//! A coordination of participles carries the Inflection its conjuncts share
//! and leaves it unvalued when they differ, as `crates/english/grammar/
//! english.fug` declares it: a schema sharing Inflection, and an `unlike`
//! one leaving it unvalued, so each coordination has one tree. A gate that
//! `Carry`s a gerund-participle then admits a coordination of them and no
//! other. Minimal repros of Altaïr Ibn-La'Ahad's *tapped and attacking
//! token*, an adjective and a gerund-participle coordinated as unlike
//! premodifiers, whose second conjunct is Aethertow's *attacking or
//! blocking*.

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
        path: "tests/participle_inflection.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("c:or", "or", Category::Coordinator, core()),
        Lexeme::invariant("adj:tapped", "tapped", Category::Adjective, core()),
        Lexeme::verb("v:attack", "attack", core()),
        Lexeme::verb("v:block", "block", core()),
        Lexeme::verb("v:destroy", "destroy", core()),
        Lexeme::noun("n:token", "token", vec![Countability::Count], core()),
    ])
    .unwrap()
}

/// *attack* and *block* may stand without their Object, so their
/// gerund-participles premodify; *destroy* may not.
const FRAMES: &str = r#"(
    frames: {
        "v:attack": [(family: Intransitive, slots: []), (family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "v:block": [(family: Intransitive, slots: []), (family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "v:destroy": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
    },
)"#;

const GRAMMAR: &str = r#"
    root Sentence
    feature Inflection = Invariant | IndependentGenitive | Singular | Plural | Plain | Present | Preterite | Irrealis | GerundParticiple | PastParticiple
    feature ComplementKind = AdjP | Gerund
    category Nominal
    category NounPhrase
    category AdjectivePhrase
    category Participle [Inflection]
    category Attributes [ComplementKind]
    category Attribute : Attributes [ComplementKind]
    category Sentence
    slot NP = NounPhrase
    word Or = "c:or"
    Bare: NounPhrase -> Nominal
    Fragment: Sentence -> NounPhrase ';'
    Adjective: AdjectivePhrase -> Lex(Adjective)
    ParticipialAttribute: Attribute -> filler:Participle
        where filler.Inflection := GerundParticiple,
              $.ComplementKind = Gerund
    AdjectivalAttribute: Attribute -> filler:AdjectivePhrase where $.ComplementKind = AdjP
    AttributedNominal: Nominal -> modifier:Attributes Nominal
    premodification Participle {
        head: Verb,
        host: Nominal,
        forms: [GerundParticiple, PastParticiple],
    }
    valence Nominal { head: Noun }
    coordination Attributes {
        coordinators: Or,
        unshared: [ComplementKind],
        serial: true,
        conjunct: Attribute,
        unlike: ComplementKind,
    }
    coordination Participle { coordinators: Or }
    coordination Participle {
        coordinators: Or,
        unshared: [Inflection],
        serial: true,
        unlike: Inflection,
    }
"#;

/// The number of Readings of `text`, each checked for Admission and
/// round-trip law 1.
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
fn each_participle_coordination_has_one_tree() {
    for text in [
        "attacking or blocking token;",
        "blocking or destroyed token;",
        "destroyed or attacking token;",
        "attacking, blocking, or destroyed token;",
        "destroyed, attacking, or destroyed token;",
    ] {
        assert_eq!(count(text), 1, "{text}");
    }
}

#[test]
fn a_coordination_of_gerund_participles_is_a_gerund_participial_attribute() {
    assert_eq!(count("tapped or attacking token;"), 1);
    assert_eq!(count("tapped or attacking or blocking token;"), 1);
}

#[test]
fn a_coordination_with_a_past_participle_is_no_gerund_participial_attribute() {
    assert_eq!(count("tapped or destroyed token;"), 0);
    assert_eq!(count("tapped or attacked or destroyed token;"), 0);
    assert_eq!(count("tapped or attacking or destroyed token;"), 0);
}

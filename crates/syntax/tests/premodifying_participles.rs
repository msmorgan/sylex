//! Which participles a Premodification Schema admits. An attributive VP
//! takes no post-head dependent (CGEL, Ch. 5 §14.2, p. 444), so an active
//! gerund-participle premodifies only when its verb may stand without a
//! Complement: *attacking Zombies*, but not *declaring Zombies* as "Zombies
//! that declare". A passive past participle's Object is the nominal it
//! modifies, so a transitive verb's premodifies (*destroyed Zombies*), unless a
//! declared Adjective spelled like it (`AdjectivalPassive: Lexical`) is its
//! Adjectival Passive and premodifies in its place.

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
        path: "tests/premodifying_participles.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let mut tap = Lexeme::verb("v:tap", "tap", core());
    for slot in &mut tap.forms {
        match slot.form {
            sylex_lexicon::WordForm::Preterite | sylex_lexicon::WordForm::PastParticiple => {
                slot.surface = Some("tapped".to_owned());
            }
            sylex_lexicon::WordForm::GerundParticiple => {
                slot.surface = Some("tapping".to_owned());
            }
            _ => {}
        }
    }
    tap.properties
        .features
        .insert("AdjectivalPassive".into(), "Lexical".into());
    Lexicon::new([
        Lexeme::verb("v:attack", "attack", core()),
        Lexeme::verb("v:declare", "declare", core()),
        Lexeme::verb("v:destroy", "destroy", core()),
        Lexeme::verb("v:wait", "wait", core()),
        tap,
        Lexeme::noun("n:Zombie", "Zombie", vec![Countability::Count], core()),
    ])
    .unwrap()
}

/// *attack* may stand without its Object; *declare*, *destroy* and *tap* may
/// not. *wait* declares no Frame.
const FRAMES: &str = r#"(
    frames: {
        "v:attack": [(family: Intransitive, slots: []), (family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "v:declare": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "v:destroy": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "v:tap": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
    },
)"#;

const GRAMMAR: &str = r"
    root Sentence
    feature AdjectivalPassive = Lexical
    feature Inflection = Invariant | IndependentGenitive | Singular | Plural | Plain | Present | Preterite | Irrealis | GerundParticiple | PastParticiple
    category Nominal
    category NounPhrase
    category Participle [Inflection]
    category Sentence
    slot NP = NounPhrase
    Bare: NounPhrase -> Nominal
    Fragment: Sentence -> NounPhrase ';'
    premodification Participle {
        head: Verb,
        host: Nominal,
        forms: [GerundParticiple, PastParticiple],
        equations: [PastParticiple -> [head.AdjectivalPassive != Lexical]],
    }
    valence Nominal { head: Noun }
";

fn count(text: &str) -> usize {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(GRAMMAR).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap();
    grammar
        .parse(&lexicon, text)
        .readings()
        .collect::<Vec<_>>()
        .len()
}

#[test]
fn a_gerund_participle_premodifies_when_its_complement_is_optional() {
    assert_eq!(count("Attacking Zombies;"), 1);
    assert_eq!(count("Waiting Zombies;"), 1);
}

#[test]
fn an_obligatorily_transitive_gerund_participle_does_not_premodify() {
    assert_eq!(count("Declaring Zombies;"), 0);
    assert_eq!(count("Destroying Zombies;"), 0);
}

#[test]
fn a_transitive_past_participle_premodifies() {
    assert_eq!(count("Destroyed Zombies;"), 1);
    assert_eq!(count("Declared Zombies;"), 1);
}

#[test]
fn a_lexical_adjectival_passive_premodifies_in_place_of_its_participle() {
    assert_eq!(count("Tapped Zombies;"), 0);
}

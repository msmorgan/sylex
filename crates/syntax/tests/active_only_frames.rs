//! A frame restricted to active voice derives no passive of any kind: *cost*
//! and *have* are transitive but do not passivize (CGEL, Ch. 16 §10.1.2,
//! p. 1432, [17]). The same frame without the restriction keeps its verbal,
//! prepositional and adjectival passives, and its participle premodifies.

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
        path: "tests/active_only_frames.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let core = || source(SourceKind::Core);
    let mut cost = Lexeme::verb("v:cost", "cost", core());
    for slot in &mut cost.forms {
        if matches!(
            slot.form,
            sylex_lexicon::WordForm::Preterite | sylex_lexicon::WordForm::PastParticiple
        ) {
            slot.surface = Some("cost".to_owned());
        }
    }
    Lexicon::new([
        Lexeme::verb("v:destroy", "destroy", core()),
        cost,
        Lexeme::verb("v:look", "look", core()),
        Lexeme::verb("v:gaze", "gaze", core()),
        Lexeme::noun("n:Zombie", "Zombie", vec![Countability::Count], core()),
        Lexeme::invariant("p:by", "by", Category::Preposition, core()),
        Lexeme::invariant("p:at", "at", Category::Preposition, core()),
    ])
    .unwrap()
}

/// *destroy* and *look at* passivize; *cost* and *gaze at* are declared
/// active-only.
const FRAMES: &str = r#"(
    frames: {
        "v:destroy": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "v:cost": [(family: Monotransitive, slots: [(relation: Object, fill: NP)], active_only: true)],
        "v:look": [(family: Prepositional, slots: [(relation: Complement, fill: PP(["p:at"]))])],
        "v:gaze": [(family: Prepositional, slots: [(relation: Complement, fill: PP(["p:at"]))], active_only: true)],
    },
    prepositions: {
        "p:by": [(complement: NP, hosts: ComplementOnly)],
        "p:at": [(complement: NP, hosts: ComplementOnly)],
    },
)"#;

const GRAMMAR: &str = r#"
    root Sentence
    feature Inflection = Invariant | IndependentGenitive | Singular | Plural | Plain | Present | Preterite | Irrealis | GerundParticiple | PastParticiple
    feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
    feature Voice = Active | Passive
    category AdjectivePhrase [Voice]
    category Nominal
    category NounPhrase
    category Participle
    category PrepositionPhrase { gap }
    category Sentence
    category VerbPhrase [VerbForm, Voice] { gap }
    word By = "p:by"
    slot NP = NounPhrase
    slot PP = PrepositionPhrase
    Imperative: Sentence -> clause:VerbPhrase '.' where clause.VerbForm = Plain, clause.Voice = Active
    Passive: Sentence -> clause:VerbPhrase '!' where clause.Voice = Passive
    Adjectival: Sentence -> AdjectivePhrase '?'
    Bare: NounPhrase -> Nominal
    Fragment: Sentence -> NounPhrase ';'
    premodification Participle { head: Verb, host: Nominal, forms: [PastParticiple] }
    valence VerbPhrase {
        head: Verb,
        active: [$.Voice = Active],
        passive: By,
        verbal_passive: [$.Voice = Passive, $.VerbForm = PastParticiple],
        prepositional_passive: [$.Voice = Passive, $.VerbForm = PastParticiple],
        adjectival: AdjectivePhrase,
        adjectival_passive: [head.VerbForm = PastParticiple, $.Voice = Passive],
    }
    valence PrepositionPhrase { head: Preposition }
    valence Nominal { head: Noun }
"#;

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
fn an_active_only_frame_keeps_its_active() {
    assert_eq!(count("Destroy Zombies."), 1);
    assert_eq!(count("Cost Zombies."), 1);
    assert_eq!(count("Look at Zombies."), 1);
    assert_eq!(count("Gaze at Zombies."), 1);
}

#[test]
fn an_active_only_frame_has_no_verbal_passive() {
    assert_eq!(count("Destroyed!"), 1);
    assert_eq!(count("Destroyed by Zombies!"), 1);
    assert_eq!(count("Cost!"), 0);
    assert_eq!(count("Cost by Zombies!"), 0);
}

#[test]
fn an_active_only_frame_has_no_prepositional_passive() {
    assert_eq!(count("Looked at!"), 1);
    assert_eq!(count("Gazed at!"), 0);
}

#[test]
fn an_active_only_frame_has_no_adjectival_passive() {
    assert_eq!(count("Destroyed?"), 1);
    assert_eq!(count("Cost?"), 0);
}

#[test]
fn an_active_only_verb_has_no_premodifying_participle() {
    assert_eq!(count("Destroyed Zombies;"), 1);
    assert_eq!(count("Cost Zombies;"), 0);
}

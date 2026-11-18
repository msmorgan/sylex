//! A frame restricted to active voice derives no passive of any kind: *cost*
//! and *have* are transitive but do not passivize (CGEL, Ch. 16 §10.1.2,
//! p. 1432, [17]). A frame restricted to passive voice derives no active:
//! *deal* takes two objects only in the recipient passive (*this creature is
//! dealt damage*), as a verb may be restricted to the passive with a
//! particular complementation (CGEL, Ch. 16 §10.1.2, p. 1435, [31]). The
//! same frame without a restriction has both voices, and its participle
//! premodifies.

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
        path: "tests/frame_voices.rs".to_owned(),
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
    let mut deal = Lexeme::verb("v:deal", "deal", core());
    let mut give = Lexeme::verb("v:give", "give", core());
    for (verb, past) in [(&mut deal, "dealt"), (&mut give, "given")] {
        for slot in &mut verb.forms {
            if slot.form == sylex_lexicon::WordForm::PastParticiple {
                slot.surface = Some(past.to_owned());
            }
        }
    }
    Lexicon::new([
        Lexeme::verb("v:destroy", "destroy", core()),
        cost,
        Lexeme::verb("v:look", "look", core()),
        Lexeme::verb("v:gaze", "gaze", core()),
        deal,
        give,
        Lexeme::noun("n:Zombie", "Zombie", vec![Countability::Count], core()),
        Lexeme::invariant("p:by", "by", Category::Preposition, core()),
        Lexeme::invariant("p:at", "at", Category::Preposition, core()),
    ])
    .unwrap()
}

/// *destroy*, *look at* and ditransitive *give* have both voices; *cost*
/// and *gaze at* are declared Active-only, ditransitive *deal*
/// Passive-only.
const FRAMES: &str = r#"(
    frames: {
        "v:destroy": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "v:cost": [(family: Monotransitive, slots: [(relation: Object, fill: NP)], voice: ActiveOnly)],
        "v:look": [(family: Prepositional, slots: [(relation: Complement, fill: PP(["p:at"]))])],
        "v:gaze": [(family: Prepositional, slots: [(relation: Complement, fill: PP(["p:at"]))], voice: ActiveOnly)],
        "v:give": [(family: Ditransitive, slots: [(relation: IndirectObject, fill: NP), (relation: Object, fill: NP)])],
        "v:deal": [(family: Ditransitive, slots: [(relation: IndirectObject, fill: NP), (relation: Object, fill: NP)], voice: PassiveOnly)],
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
    Imperative: Sentence -> clause:VerbPhrase '.' { capital: [clause] } where clause.VerbForm = Plain, clause.Voice = Active
    Passive: Sentence -> clause:VerbPhrase '!' { capital: [clause] } where clause.Voice = Passive
    Adjectival: Sentence -> AdjectivePhrase '?' { capital: [head] }
    Bare: NounPhrase -> Nominal
    Fragment: Sentence -> NounPhrase ';' { capital: [head] }
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

#[test]
fn a_passive_only_frame_has_no_active() {
    assert_eq!(count("Give Zombies Zombies."), 1);
    assert_eq!(count("Deal Zombies Zombies."), 0);
}

#[test]
fn a_passive_only_frame_keeps_its_passive() {
    assert_eq!(count("Given Zombies!"), 1);
    assert_eq!(count("Given Zombies by Zombies!"), 1);
    assert_eq!(count("Dealt Zombies!"), 1);
    assert_eq!(count("Dealt Zombies by Zombies!"), 1);
}

#[test]
fn a_passive_only_frame_is_one_that_passivizes() {
    let intransitive =
        r#"(frames: { "v:deal": [(family: Intransitive, slots: [], voice: PassiveOnly)] })"#;
    let error = FrameCatalog::from_ron_str(intransitive)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("is Passive-only but has no passive"),
        "{error}"
    );
}

/// The error binding `grammar` with `frames`.
fn bind_error(grammar: &str, frames: &str) -> String {
    let lexicon = lexicon();
    CompiledGrammar::bind(
        &Grammar::from_fug_str(grammar).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(frames).unwrap(),
    )
    .map(|_| ())
    .unwrap_err()
    .to_string()
}

#[test]
fn only_a_verbs_frame_restricts_its_voice() {
    let noun = r#"(frames: {
        "v:destroy": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "n:Zombie": [(family: NounComplement, slots: [(relation: Complement, fill: PP(["p:at"]))], voice: ActiveOnly)],
    }, prepositions: { "p:by": [(complement: NP, hosts: ComplementOnly)], "p:at": [(complement: NP, hosts: ComplementOnly)] })"#;
    let error = bind_error(GRAMMAR, noun);
    assert!(error.contains("only a verb's has one"), "{error}");
}

#[test]
fn a_passive_only_frame_needs_the_schemas_passive() {
    let active = GRAMMAR
        .replace("        passive: By,\n", "")
        .replace(
            "        verbal_passive: [$.Voice = Passive, $.VerbForm = PastParticiple],\n",
            "",
        )
        .replace(
            "        prepositional_passive: [$.Voice = Passive, $.VerbForm = PastParticiple],\n",
            "",
        );
    assert_ne!(active, GRAMMAR);
    let error = bind_error(&active, FRAMES);
    assert!(error.contains("needs the schema's passive"), "{error}");
}

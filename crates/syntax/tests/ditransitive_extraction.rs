//! Only a ditransitive's Direct Object extracts: an Indirect Object resists
//! prenuclear position (*The key he gave Sue*, but *?Sue he gave the key*),
//! while the Direct Object takes it as readily as a monotransitive Object
//! (CGEL, Ch. 4 §4.3(d), pp. 248–250, [9]). Grounded in *give* (*Animation
//! Module*'s *Give that permanent or player another counter of that kind*):
//! the active Gaps only its Direct Object, and the recipient passive, whose
//! Indirect Object is the Subject, keeps its Direct Object's Gap.

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
        path: "tests/ditransitive_extraction.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let mut give = Lexeme::verb("v:give", "give", core());
    for slot in &mut give.forms {
        if slot.form == sylex_lexicon::WordForm::PastParticiple {
            slot.surface = Some("given".to_owned());
        }
    }
    Lexicon::new([
        give,
        Lexeme::noun("n:Zombie", "Zombie", vec![Countability::Count], core()),
        Lexeme::invariant("p:by", "by", Category::Preposition, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:give": [(family: Ditransitive, slots: [
            (relation: IndirectObject, fill: NP),
            (relation: Object, fill: NP),
        ])],
    },
    prepositions: {
        "p:by": [(complement: NP, hosts: ComplementOnly)],
    },
)"#;

/// A `;` fragment is an active Verb Phrase with an NP Gap, discharged
/// there, and a `!` fragment a passive one.
const GRAMMAR: &str = r#"
    root Sentence
    feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
    feature Voice = Active | Passive
    category Nominal
    category NounPhrase
    category PrepositionPhrase
    category Sentence
    category VerbPhrase [VerbForm, Voice] { gap }
    word By = "p:by"
    slot NP = NounPhrase
    slot PP = PrepositionPhrase
    Imperative: Sentence -> clause:VerbPhrase '.' { capital: [clause] } where clause.VerbForm = Plain, clause.Voice = Active
    Gapped: Sentence -> clause:VerbPhrase ';' { capital: [clause] } where clause.Gap = NP, clause.VerbForm = Plain, clause.Voice = Active
    PassiveGapped: Sentence -> clause:VerbPhrase '!' { capital: [clause] } where clause.Gap = NP, clause.Voice = Passive
    Bare: NounPhrase -> Nominal
    valence VerbPhrase {
        head: Verb,
        active: [$.Voice = Active],
        passive: By,
        verbal_passive: [$.Voice = Passive, $.VerbForm = PastParticiple],
        prepositional_passive: [$.Voice = Passive, $.VerbForm = PastParticiple],
    }
    valence PrepositionPhrase { head: Preposition }
    valence Nominal { head: Noun }
"#;

/// The Productions of the Verb Phrase of each Reading of `text`.
fn productions(text: &str) -> Vec<String> {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(GRAMMAR).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap();
    let mut found: Vec<String> = grammar
        .parse(&lexicon, text)
        .readings()
        .map(|reading| {
            let clause = reading.node().unwrap().children[0].node().unwrap();
            clause.production.to_string()
        })
        .collect();
    found.sort();
    found
}

#[test]
fn an_active_ditransitive_gaps_only_its_direct_object() {
    assert_eq!(
        productions("Give Zombies;"),
        ["Valence:VerbPhrase/Ditransitive(NP,NP)/gap1"]
    );
    assert_eq!(
        productions("Give Zombies Zombies."),
        ["Valence:VerbPhrase/Ditransitive(NP,NP)"]
    );
}

#[test]
fn a_recipient_passive_gaps_its_direct_object() {
    assert_eq!(
        productions("Given!"),
        ["Valence:VerbPhrase/Ditransitive(NP,NP)/passive/gap0"]
    );
}

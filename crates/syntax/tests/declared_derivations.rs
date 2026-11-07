//! What a derived phrase carries beyond its frame comes from the grammar's
//! declarations, never from feature or value names in the engine
//! (`docs/decisions/english.md` §1): a Valence schema's voice
//! and passive equations and its `complement_kind`, a Premodification's
//! per-form equations, the grammar's `slot_equations` and `high_attachment`. A
//! derivation whose declaration is missing is a load error, never a silent
//! default.

use sylex_lexicon::Category;
use sylex_lexicon::Countability;
use sylex_lexicon::Lexeme;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;
use sylex_syntax::LoadError;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/declared_derivations.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let mut he = Lexeme::noun("n:he", "he", vec![Countability::Count], core());
    he.properties
        .features
        .insert("Case".into(), "Nominative".into());
    Lexicon::new([
        Lexeme::verb("v:destroy", "destroy", core()),
        Lexeme::noun("n:Zombie", "Zombie", vec![Countability::Count], core()),
        he,
        Lexeme::invariant("p:by", "by", Category::Preposition, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:destroy": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
    },
    prepositions: {
        "p:by": [(complement: NP, hosts: ComplementOnly)],
    },
)"#;

/// The verb Valence schema's declarations, the equations of the `NP` slot, a
/// High Attachment declaration and a Premodification are spliced in.
fn grammar(
    valence: &str,
    np_slot_equations: &str,
    high_attachment: &str,
    premodification: &str,
) -> String {
    format!(
        r#"
            root Sentence
            feature Case = Nominative | Accusative | Genitive
            feature ComplementKind = NP | Intransitive
            feature Inflection = Invariant | IndependentGenitive | Singular | Plural | Plain | Present | Preterite | Irrealis | GerundParticiple | PastParticiple
            feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
            feature Voice = Active | Passive
            category AdjectivePhrase [Voice]
            category Nominal [Case]
            category NounPhrase [Case]
            category Participle [Inflection]
            category PrepositionPhrase [ComplementKind] {{ gap }}
            category Sentence
            category VerbPhrase [VerbForm, Voice] {{ gap }}
            word By = "p:by"
            slot NP = NounPhrase {np_slot_equations}
            slot PP = PrepositionPhrase
            {high_attachment}
            Imperative: Sentence -> clause:VerbPhrase '.' where clause.VerbForm = Plain
            Passive: Sentence -> clause:VerbPhrase '!' where clause.Voice = Passive
            Adjectival: Sentence -> AdjectivePhrase '?'
            Bare: NounPhrase -> Nominal
            Fragment: Sentence -> NounPhrase ';'
            {premodification}
            valence VerbPhrase {{ head: Verb, {valence} }}
            valence PrepositionPhrase {{ head: Preposition, complement_kind: ComplementKind }}
            valence Nominal {{ head: Noun }}
        "#
    )
}

const VOICES: &str = "active: [$.Voice = Active], passive: By, verbal_passive: [$.Voice = Passive, $.VerbForm = PastParticiple], prepositional_passive: [$.Voice = Passive, $.VerbForm = PastParticiple], adjectival: AdjectivePhrase, adjectival_passive: [head.VerbForm = PastParticiple, $.Voice = Passive]";

const PREMODIFICATION: &str =
    "premodification Participle { head: Verb, host: Nominal, forms: [PastParticiple] }";

fn bind(text: &str) -> Result<(Lexicon, CompiledGrammar), LoadError> {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(text).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )?;
    Ok((lexicon, grammar))
}

fn count(text: &str, input: &str) -> usize {
    let (lexicon, grammar) = bind(text).unwrap();
    grammar
        .parse(&lexicon, input)
        .readings()
        .collect::<Vec<_>>()
        .len()
}

fn load_error(text: &str) -> String {
    bind(text).map(|_| ()).unwrap_err().to_string()
}

#[test]
fn the_declared_equations_value_each_derived_voice() {
    let text = grammar(VOICES, "", "", "");
    assert_eq!(count(&text, "Destroy Zombie."), 1);
    assert_eq!(count(&text, "Destroyed!"), 1);
    assert_eq!(count(&text, "Destroyed by Zombie!"), 1);
    assert_eq!(count(&text, "Destroyed?"), 1);
}

#[test]
fn a_passive_without_its_equations_is_a_load_error() {
    let error = load_error(&grammar("passive: By", "", "", ""));
    assert!(error.contains("verbal_passive"), "{error}");
}

#[test]
fn an_adjectival_passive_without_its_equations_is_a_load_error() {
    let valence = VOICES.replace(
        ", adjectival_passive: [head.VerbForm = PastParticiple, $.Voice = Passive]",
        "",
    );
    let error = load_error(&grammar(&valence, "", "", ""));
    assert!(error.contains("adjectival_passive"), "{error}");
}

#[test]
fn a_complement_kind_the_phrase_does_not_carry_is_a_load_error() {
    let valence = format!("{VOICES}, complement_kind: ComplementKind");
    let error = load_error(&grammar(&valence, "", "", ""));
    assert!(error.contains("does not carry"), "{error}");
}

#[test]
fn slot_equations_constrain_every_frame_slot_of_their_kind() {
    let undeclared = grammar(VOICES, "", "", "");
    assert_eq!(count(&undeclared, "Destroy he."), 1);
    let declared = grammar(VOICES, "where slot.Case != Nominative", "", "");
    assert_eq!(count(&declared, "Destroy he."), 0);
    assert_eq!(count(&declared, "Destroy Zombie."), 1);
}

#[test]
fn a_high_attachment_declaration_names_declared_values() {
    let error = load_error(&grammar(
        VOICES,
        "",
        "high_attachment PrepositionPhrase where $.ComplementKind = Distributive",
        "",
    ));
    assert!(error.contains("high attachment"), "{error}");
}

#[test]
fn premodification_equations_constrain_their_form() {
    let undeclared = grammar(VOICES, "", "", PREMODIFICATION);
    assert_eq!(count(&undeclared, "Destroyed Zombie;"), 1);
    let excluded = grammar(
        VOICES,
        "",
        "",
        "premodification Participle { head: Verb, host: Nominal, forms: [PastParticiple], equations: [PastParticiple -> [head.Inflection != PastParticiple]] }",
    );
    assert_eq!(count(&excluded, "Destroyed Zombie;"), 0);
}

#[test]
fn premodification_equations_name_only_its_forms() {
    let error = load_error(&grammar(
        VOICES,
        "",
        "",
        "premodification Participle { head: Verb, host: Nominal, forms: [PastParticiple], equations: [GerundParticiple -> []] }",
    ));
    assert!(error.contains("GerundParticiple"), "{error}");
}

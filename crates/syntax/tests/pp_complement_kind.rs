//! A Frame Slot `PP(prepositions, Some(kind))` selects a Preposition Phrase
//! by its preposition and its Complement's kind: *ask for [Zombie]*, a cited
//! word, is a Complement, while *ask for [the Zombie]* stays an Adjunct, not
//! pre-empted by same-head complement priority. A slot that names no kind
//! selects its preposition with any Complement but a `named_only` one.

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
use sylex_syntax::Reading;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/pp_complement_kind.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::verb("v:ask", "ask", core()),
        Lexeme::verb("v:wait", "wait", core()),
        Lexeme::invariant("p:for", "for", Category::Preposition, core()),
        Lexeme::invariant("d:the", "the", Category::Determinative, core()),
        Lexeme::noun("n:Zombie", "Zombie", vec![Countability::Count], core()),
    ])
    .unwrap()
}

fn frames(ask: &str) -> String {
    format!(
        r#"(
    frames: {{
        "v:ask": [(family: Intransitive, slots: []), (family: Prepositional, slots: [(relation: Complement, fill: {ask})])],
        "v:wait": [(family: Intransitive, slots: []), (family: Prepositional, slots: [(relation: Complement, fill: PP(["p:for"]))])],
    }},
    prepositions: {{
        "p:for": [(complement: NP, hosts: Predicate), (complement: Mention, hosts: ComplementOnly, named_only: true)],
    }},
)"#
    )
}

const GRAMMAR: &str = r"
    root Sentence
    feature Inflection = Invariant | IndependentGenitive | Singular | Plural | Plain | Present | Preterite | Irrealis | GerundParticiple | PastParticiple
    feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
    category Mention
    category Nominal
    category NounPhrase
    category PrepositionPhrase
    category Sentence
    category VerbPhrase [VerbForm]
    slot NP = NounPhrase
    slot PP = PrepositionPhrase
    slot Mention = Mention
    Imperative: Sentence -> clause:VerbPhrase '.' where clause.VerbForm = Plain
    Determined: NounPhrase -> determiner:Lex(Determinative) Nominal
    WordMention: Mention -> Lex(Noun)
    valence VerbPhrase { head: Verb }
    valence PrepositionPhrase { head: Preposition }
    valence Nominal { head: Noun }
    adjunct PrepositionPhrase { host: VerbPhrase, side: After, host_kind: Predicate }
";

fn grammar(lexicon: &Lexicon, ask: &str) -> Result<CompiledGrammar, LoadError> {
    CompiledGrammar::bind(
        &Grammar::from_fug_str(GRAMMAR).unwrap(),
        lexicon,
        &FrameCatalog::from_ron_str(&frames(ask)).unwrap(),
    )
}

fn readings(text: &str) -> Vec<Reading> {
    let lexicon = lexicon();
    grammar(&lexicon, r#"PP(["p:for"], Some(Mention))"#)
        .unwrap()
        .parse(&lexicon, text)
        .readings()
        .collect::<Vec<_>>()
}

fn productions(reading: &Reading, out: &mut Vec<String>) {
    if let Reading::Node(node) = reading {
        out.push(node.production.to_string());
        for child in &node.children {
            productions(child, out);
        }
    }
}

/// The one Reading of `text`, and whether its *for*-phrase is a selected
/// Complement rather than an Adjunct.
fn selected(text: &str) -> bool {
    let readings = readings(text);
    assert_eq!(readings.len(), 1, "{text}: {readings:#?}");
    let mut ids = Vec::new();
    productions(&readings[0], &mut ids);
    let complement = ids.iter().any(|id| id.contains("/Prepositional("));
    let adjunct = ids.iter().any(|id| id.starts_with("Adjunct:"));
    assert_ne!(complement, adjunct, "{text}: {ids:?}");
    complement
}

#[test]
fn a_slot_naming_its_complement_kind_selects_only_that_kind() {
    assert!(selected("Ask for Zombie."));
    // Not pre-empted: the slot does not select *for* with a Noun Phrase.
    assert!(!selected("Ask for the Zombie."));
}

#[test]
fn a_slot_naming_no_kind_selects_its_preposition_with_a_noun_phrase() {
    // Same-head complement priority pre-empts the Adjunct.
    assert!(selected("Wait for the Zombie."));
}

#[test]
fn a_named_only_frame_is_selected_only_by_a_slot_naming_its_kind() {
    // *for* + Mention is `named_only`, and not an Adjunct: *wait* selects
    // *for* alone, so the cited word has no site.
    assert_eq!(readings("Wait for Zombie.").len(), 0);
}

#[test]
fn a_named_kind_must_be_one_of_the_prepositions_frames() {
    let lexicon = lexicon();
    let error = grammar(&lexicon, r#"PP(["p:for"], Some(Gerund))"#).unwrap_err();
    assert!(error.to_string().contains("no frame for"), "{error}");
}

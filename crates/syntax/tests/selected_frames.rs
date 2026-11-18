//! Frames that the grammar places itself: Valence schemas that take only the
//! frames selecting named prepositions (`selects`, `variant`,
//! `complemented`), the symbols that read a head's frames (`Selecting`,
//! `SelectedBy`), and the bare lexical head of an authored Production, whose
//! omitted Complement is the Preposition Phrase right after it.

use sylex_lexicon::Category;
use sylex_lexicon::Countability;
use sylex_lexicon::Lexeme;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;
use sylex_syntax::Reading;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/selected_frames.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let mut lexemes = vec![
        Lexeme::verb("v:see", "see", core()),
        Lexeme::verb("v:stay", "stay", core()),
        Lexeme::verb("v:lie", "lie", core()),
    ];
    for word in [
        "the", "that", "those", "each", "most", "many", "more", "all",
    ] {
        lexemes.push(Lexeme::invariant(
            format!("d:{word}"),
            word,
            Category::Determinative,
            core(),
        ));
    }
    for word in ["of", "than", "but", "in", "on"] {
        lexemes.push(Lexeme::invariant(
            format!("p:{word}"),
            word,
            Category::Preposition,
            core(),
        ));
    }
    lexemes.push(Lexeme::noun(
        "n:card",
        "card",
        vec![Countability::Count],
        core(),
    ));
    Lexicon::new(lexemes).unwrap()
}

/// The frames, with *lie*'s prepositions as given.
fn frames(lie: &str) -> String {
    format!(
        r#"(
            frames: {{
                "v:see": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
                "v:stay": [(family: Prepositional, slots: [(relation: Complement, fill: PP(["p:in", "p:on"]))])],
                "v:lie": [(family: Prepositional, slots: [(relation: Complement, fill: PP([{lie}]))])],
                "d:each": [(family: DeterminativeComplement, slots: [(relation: Complement, fill: PP(["p:of"]))])],
                "d:most": [(family: DeterminativeComplement, slots: [(relation: Complement, fill: PP(["p:of"]))])],
                "d:many": [(family: DeterminativeComplement, slots: [(relation: Complement, fill: PP(["p:of"]))])],
                "d:more": [
                    (family: DeterminativeComplement, slots: [(relation: Complement, fill: PP(["p:of"]))]),
                    (family: DeterminativeComplement, slots: [(relation: Complement, fill: PP(["p:than"]))]),
                ],
                "d:all": [(family: DeterminativeComplement, slots: [(relation: Complement, fill: PP(["p:but"]))])],
            }},
            prepositions: {{
                "p:of": [(complement: NP, hosts: Nominal)],
                "p:than": [(complement: NP, hosts: ComplementOnly)],
                "p:but": [(complement: NP, hosts: ComplementOnly)],
                "p:in": [(complement: NP, hosts: ComplementOnly)],
                "p:on": [(complement: NP, hosts: ComplementOnly)],
            }},
        )"#
    )
}

const GRAMMAR: &str = r#"
    root Sentence
    feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
    category DeterminerPhrase
    category Nominal
    category NounPhrase
    category PartitiveHead
    category PrepositionPhrase
    category Sentence
    category VerbPhrase [VerbForm]
    word Of = "p:of"
    word Than = "p:than"
    word But = "p:but"
    word That = "d:that"
    class Fused = ["d:each", "d:those"]
    class Degree = ["d:many", "d:most"]
    class Locating = ["v:stay", "v:lie"]
    slot NP = NounPhrase
    slot PP = PrepositionPhrase
    Imperative: Sentence -> clause:VerbPhrase '.' { capital: [clause] } where clause.VerbForm = Plain
    Determined: NounPhrase -> determiner:DeterminerPhrase Nominal
    Partitive: NounPhrase -> PartitiveHead
    FusedHead: NounPhrase -> Class(Fused)
    DegreeDeterminer: DeterminerPhrase -> modifier:Word(That) Class(Degree)
    DegreePartitive: NounPhrase -> DeterminerPhrase complement:Selected(PrepositionPhrase, Of) {
        heavy: [head],
    }
    IndirectComplement: NounPhrase ->
        determiner:Selecting(Determinative, Than) Nominal complement:Selected(PrepositionPhrase, Than)
    Located: VerbPhrase ->
        Class(Locating) subject:NounPhrase complement:SelectedBy(PrepositionPhrase, Locating)
    adjunct PrepositionPhrase { host: NounPhrase, side: After, host_kind: Nominal }
    valence VerbPhrase { head: Verb }
    valence PrepositionPhrase { head: Preposition }
    valence Nominal { head: Noun }
    valence DeterminerPhrase { head: Determinative, selects: [But] }
    valence DeterminerPhrase { variant: Comparative, head: Determinative, selects: [Than], complemented }
    valence PartitiveHead { head: Determinative, selects: [Of], complemented }
"#;

fn bind(grammar: &str, lie: &str) -> Result<CompiledGrammar, sylex_syntax::LoadError> {
    CompiledGrammar::bind(
        &Grammar::from_fug_str(grammar).unwrap(),
        &lexicon(),
        &FrameCatalog::from_ron_str(&frames(lie)).unwrap(),
    )
}

/// Every Reading of `text`, each checked for Admission and round-trip law 1.
fn readings(text: &str) -> Vec<Reading> {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, r#""p:in", "p:on""#).unwrap();
    let readings = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
    for reading in &readings {
        assert_eq!(
            grammar.admit(reading, &lexicon, &sylex_lexicon::CardScope::default()),
            Ok(())
        );
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    readings
}

/// Whether some Constituent of `reading` is built by Production `id`.
fn built_by(reading: &Reading, id: &str) -> bool {
    reading.node().is_some_and(|node| {
        node.production.as_str() == id || node.children.iter().any(|c| built_by(c, id))
    })
}

const OF: &str = "Valence:PartitiveHead/DeterminativeComplement(PP[p:of])";
const THAN: &str = "Valence:DeterminerPhrase/DeterminativeComplement(PP[p:than])/Comparative";
const BUT: &str = "Valence:DeterminerPhrase/DeterminativeComplement(PP[p:but])";

#[test]
fn a_schema_takes_only_the_frames_it_selects() {
    let grammar = bind(GRAMMAR, r#""p:in", "p:on""#).unwrap();
    let ids: Vec<&str> = grammar.production_ids().collect();
    for id in [OF, THAN, BUT, "Valence:DeterminerPhrase/Bare"] {
        assert!(ids.contains(&id), "{id}");
    }
    // Each frame is one schema's, and a `complemented` schema has no bare
    // phrase.
    for id in [
        "Valence:DeterminerPhrase/DeterminativeComplement(PP[p:of])",
        "Valence:DeterminerPhrase/DeterminativeComplement(PP[p:than])",
        "Valence:PartitiveHead/DeterminativeComplement(PP[p:but])",
        "Valence:PartitiveHead/Bare",
    ] {
        assert!(!ids.contains(&id), "{id}");
    }
}

#[test]
fn a_selected_frame_projects_to_its_schemas_phrase() {
    let [reading] = &readings("See most of the cards.")[..] else {
        panic!("one Reading");
    };
    assert!(built_by(reading, OF));
    let [reading] = &readings("See more than the card cards.")[..] else {
        panic!("one Reading");
    };
    assert!(built_by(reading, THAN));
    let [reading] = &readings("See all but the card cards.")[..] else {
        panic!("one Reading");
    };
    assert!(built_by(reading, BUT));
}

#[test]
fn selecting_admits_the_heads_with_the_frame() {
    let [reading] = &readings("See more cards than the card.")[..] else {
        panic!("one Reading");
    };
    assert!(built_by(reading, "IndirectComplement"));
    // *Most* selects no *than* phrase.
    assert_eq!(readings("See most cards than the card.").len(), 0);
}

#[test]
fn selected_by_takes_the_prepositions_of_the_class_frames() {
    let [reading] = &readings("Stay the card in the cards.")[..] else {
        panic!("one Reading");
    };
    assert!(built_by(reading, "Located"));
    assert_eq!(readings("Lie the card on the cards.").len(), 1);
    assert_eq!(readings("Stay the card than the cards.").len(), 0);
}

#[test]
fn selected_by_refuses_a_class_whose_members_select_unlike_prepositions() {
    let error = bind(GRAMMAR, r#""p:in""#).unwrap_err().to_string();
    assert!(error.contains("selects other prepositions"), "{error}");
}

#[test]
fn an_authored_bare_head_takes_the_phrase_its_frame_selects() {
    // *Each* selects *of*: the phrase after it is its Complement, never the
    // fused head's Modifier.
    let [reading] = &readings("See each of the cards.")[..] else {
        panic!("one Reading");
    };
    assert!(built_by(reading, OF));
    // *Those* selects nothing, so the phrase modifies its fused head.
    let [reading] = &readings("See those of the cards.")[..] else {
        panic!("one Reading");
    };
    assert!(built_by(reading, "FusedHead"));
}

#[test]
fn a_production_takes_its_bare_heads_complement_itself() {
    let [reading] = &readings("See that many of the cards.")[..] else {
        panic!("one Reading");
    };
    assert!(built_by(reading, "DegreePartitive"));
}

#[test]
fn a_frame_half_selected_is_a_load_error() {
    let grammar = GRAMMAR.replace(
        "valence VerbPhrase { head: Verb }",
        "word In = \"p:in\"\n    valence VerbPhrase { head: Verb, selects: [In] }",
    );
    let error = bind(&grammar, r#""p:in", "p:on""#).unwrap_err().to_string();
    assert!(error.contains("only some of its prepositions"), "{error}");
}

#[test]
fn a_verb_schema_is_never_complemented() {
    let grammar = GRAMMAR.replace(
        "valence VerbPhrase { head: Verb }",
        "valence VerbPhrase { head: Verb, complemented }",
    );
    let error = bind(&grammar, r#""p:in", "p:on""#).unwrap_err().to_string();
    assert!(error.contains("`complemented` on a verb head"), "{error}");
}

#[test]
fn selecting_no_head_is_a_load_error() {
    let grammar = GRAMMAR.replace(
        "determiner:Selecting(Determinative, Than)",
        "determiner:Selecting(Noun, Than)",
    );
    let error = bind(&grammar, r#""p:in", "p:on""#).unwrap_err().to_string();
    assert!(error.contains("`Selecting`: no Noun"), "{error}");
}

#[test]
fn an_unshared_feature_is_one_the_phrase_carries() {
    let grammar = GRAMMAR.replace(
        "selects: [Than], complemented }",
        "selects: [Than], complemented, unshared: [VerbForm] }",
    );
    let error = bind(&grammar, r#""p:in", "p:on""#).unwrap_err().to_string();
    assert!(error.contains("unshared feature `VerbForm`"), "{error}");
}

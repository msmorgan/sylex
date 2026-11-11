//! An intransitive preposition heads a Preposition Phrase alone, as *out* in
//! *phase out*, and a verb frame can select it, alone or as the fixed
//! preposition of an Idiom (*come up heads*).

use sylex_lexicon::Category;
use sylex_lexicon::Lexeme;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;
use sylex_syntax::frames::FrameFamily;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/intransitive_prepositions.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::verb("v:phase", "phase", core()),
        Lexeme::invariant("p:out", "out", Category::Preposition, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:phase": [(family: Prepositional, slots: [(relation: Complement, fill: PP(["p:out"]))])],
    },
    prepositions: {
        "p:out": [(complement: Intransitive, hosts: ComplementOnly)],
    },
)"#;

const GRAMMAR: &str = r"
    root Sentence
    feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
    category PrepositionPhrase
    category Sentence
    category VerbPhrase [VerbForm]
    slot PP = PrepositionPhrase
    Imperative: Sentence -> clause:VerbPhrase '.' where clause.VerbForm = Plain
    valence VerbPhrase { head: Verb }
    valence PrepositionPhrase { head: Preposition }
";

#[test]
fn a_verb_selects_an_intransitive_preposition() {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(GRAMMAR).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap();
    let mut readings = grammar
        .parse(&lexicon, "Phase out.")
        .readings()
        .collect::<Vec<_>>();
    assert_eq!(readings.len(), 1, "{readings:#?}");
    let reading = readings.remove(0);
    assert_eq!(
        grammar.admit(&reading, &lexicon, &sylex_lexicon::CardScope::default()),
        Ok(())
    );
    assert_eq!(grammar.realize(&reading, &lexicon).unwrap(), "Phase out.");
    let clause = reading.node().unwrap().children[0].node().unwrap();
    assert_eq!(
        clause.selected_frame.as_ref().map(|frame| frame.family),
        Some(FrameFamily::Prepositional)
    );
    assert!(
        grammar
            .parse(&lexicon, "Phase.")
            .readings()
            .collect::<Vec<_>>()
            .is_empty(),
        "the particle is selected, not optional"
    );
}

#[test]
fn only_a_preposition_has_no_complement() {
    let noun = r#"(frames: {"n:x": [(family: NounComplement, slots: [(relation: Complement, fill: Intransitive)])]})"#;
    assert!(FrameCatalog::from_ron_str(noun).is_err());
}

#[test]
fn an_idiom_selects_its_particle_and_a_predicative() {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/intransitive_prepositions.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let lexicon = Lexicon::new([
        Lexeme::verb("v:come", "come", core()),
        Lexeme::invariant("p:up", "up", Category::Preposition, core()),
        Lexeme::invariant("n:heads", "heads", Category::Noun, core()),
    ])
    .unwrap();
    let frames = r#"(
        frames: {
            "v:come": [(family: Idiom, slots: [
                (relation: Complement, fill: PP(["p:up"])),
                (relation: PredicativeComplement, fill: Pred),
            ])],
        },
        prepositions: { "p:up": [(complement: Intransitive, hosts: ComplementOnly)] },
    )"#;
    let grammar = r"
        root Sentence
        feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
        category Predicative { function }
        category PrepositionPhrase
        category Sentence
        category VerbPhrase [VerbForm]
        slot PP = PrepositionPhrase
        slot Pred = Predicative
        Imperative: Sentence -> clause:VerbPhrase '.' where clause.VerbForm = Plain
        NounPredicative: Predicative -> Lex(Noun)
        valence VerbPhrase { head: Verb }
        valence PrepositionPhrase { head: Preposition }
    ";
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(grammar).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(frames).unwrap(),
    )
    .unwrap();
    let mut readings = grammar
        .parse(&lexicon, "Come up heads.")
        .readings()
        .collect::<Vec<_>>();
    assert_eq!(readings.len(), 1, "{readings:#?}");
    let reading = readings.remove(0);
    assert_eq!(
        grammar.admit(&reading, &lexicon, &sylex_lexicon::CardScope::default()),
        Ok(())
    );
    let clause = reading.node().unwrap().children[0].node().unwrap();
    assert_eq!(
        clause.selected_frame.as_ref().map(|frame| frame.family),
        Some(FrameFamily::Idiom)
    );
    for text in ["Come up.", "Come heads."] {
        assert!(
            grammar
                .parse(&lexicon, text)
                .readings()
                .collect::<Vec<_>>()
                .is_empty(),
            "{text}"
        );
    }
}

#[test]
fn an_idiom_has_a_fixed_preposition() {
    for (slots, ok) in [
        (
            r#"(relation: Complement, fill: PP(["p:up"])), (relation: PredicativeComplement, fill: Pred)"#,
            true,
        ),
        (r"(relation: PredicativeComplement, fill: Pred)", false),
        (
            r#"(relation: Complement, fill: PP(["p:up"])), (relation: Object, fill: NP)"#,
            false,
        ),
    ] {
        let catalog = format!(r#"(frames: {{"v:come": [(family: Idiom, slots: [{slots}])]}})"#);
        assert_eq!(FrameCatalog::from_ron_str(&catalog).is_ok(), ok, "{slots}");
    }
}

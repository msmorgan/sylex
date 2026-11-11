//! A Valence schema's `central` Adjunct stands between a central head and its
//! first Complement, from every frame of that head but a catenative one (*is
//! still on top of your library*, Commander Spock; CGEL, Ch. 3 §2.1.5,
//! p. 102), and between any head of a central frame family and its
//! Complement (*become no longer suspected*, Deadly Complication). A lexical
//! verb that shares a central head's other frame takes none there (*remains
//! on top of your library*, Temporal Aperture), and the derived phrase keeps
//! the frames' preference of a locative Complement over a predicative one.

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
        path: "tests/central_adjuncts.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::verb("v:be", "be", core()),
        Lexeme::verb("v:remain", "remain", core()),
        Lexeme::verb("v:become", "become", core()),
        Lexeme::invariant("p:on", "on", Category::Preposition, core()),
        Lexeme::invariant("n:top", "top", Category::Noun, core()),
        Lexeme::invariant("adv:still", "still", Category::Adverb, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:be": [
            (family: Copular, slots: [(relation: PredicativeComplement, fill: Pred)]),
            (family: Prepositional, slots: [(relation: Complement, fill: PP(["p:on"]))]),
        ],
        "v:become": [(family: Copular, slots: [(relation: PredicativeComplement, fill: Pred)])],
        "v:remain": [
            (family: Copular, slots: [(relation: PredicativeComplement, fill: Pred)]),
            (family: Prepositional, slots: [(relation: Complement, fill: PP(["p:on"]))]),
        ],
    },
    prepositions: {
        "p:on": [(complement: NP, hosts: NominalAndPredicate)],
    },
)"#;

fn grammar(valence: &str) -> Result<CompiledGrammar, sylex_syntax::LoadError> {
    let text = format!(
        r#"
root Sentence
feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
category AdverbPhrase
category CentralAdjunct {{ function }}
category NounPhrase
category Predicative {{ function }}
category PrepositionPhrase
category Sentence
category VerbPhrase [VerbForm]
class BeVerb = ["v:be"]
slot NP = NounPhrase
slot PP = PrepositionPhrase
slot Pred = Predicative
Imperative: Sentence -> clause:VerbPhrase '.' where clause.VerbForm = Plain
Noun: NounPhrase -> Lex(Noun)
Adverb: AdverbPhrase -> Lex(Adverb)
CentralAdverb: CentralAdjunct -> AdverbPhrase
PrepositionalPredicative: Predicative -> Hosted(PrepositionPhrase, Nominal)
{valence}
valence PrepositionPhrase {{ head: Preposition }}
        "#
    );
    CompiledGrammar::bind(
        &Grammar::from_fug_str(&text).unwrap(),
        &lexicon(),
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
}

const CENTRAL: &str = "valence VerbPhrase { head: Verb, central: CentralAdjunct, central_heads: BeVerb, central_families: [Copular] }";

#[test]
fn a_central_head_takes_the_adjunct_before_its_locative_complement() {
    let lexicon = lexicon();
    let grammar = grammar(CENTRAL).unwrap();
    for (text, adverb) in [("Be still on top.", true), ("Be on top.", false)] {
        let readings = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
        // One analysis: the locative Complement, never also a predicative.
        assert_eq!(readings.len(), 1, "{text}: {readings:#?}");
        let reading = &readings[0];
        assert_eq!(
            grammar.admit(reading, &lexicon, &sylex_lexicon::CardScope::default()),
            Ok(()),
            "{text}"
        );
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
        let phrase = reading.node().unwrap().children[0].node().unwrap();
        assert_eq!(
            phrase.selected_frame.as_ref().map(|frame| frame.family),
            Some(FrameFamily::Prepositional),
            "{text}"
        );
        assert_eq!(
            phrase.production.as_str().ends_with("/central"),
            adverb,
            "{text}"
        );
    }
}

#[test]
fn a_central_family_takes_the_adjunct_whatever_its_head() {
    let lexicon = lexicon();
    let central = grammar(CENTRAL).unwrap();
    let readings = central
        .parse(&lexicon, "Become still on top.")
        .readings()
        .collect::<Vec<_>>();
    assert_eq!(readings.len(), 1, "{readings:#?}");
    let phrase = readings[0].node().unwrap().children[0].node().unwrap();
    assert_eq!(
        phrase.selected_frame.as_ref().map(|frame| frame.family),
        Some(FrameFamily::Copular)
    );
    assert!(phrase.production.as_str().ends_with("/central"));
    // Without the family, only a central head takes one.
    let heads_only = grammar(
        "valence VerbPhrase { head: Verb, central: CentralAdjunct, central_heads: BeVerb }",
    )
    .unwrap();
    assert!(
        heads_only
            .parse(&lexicon, "Become still on top.")
            .readings()
            .next()
            .is_none()
    );
}

#[test]
fn a_lexical_verb_sharing_the_frame_takes_none() {
    let lexicon = lexicon();
    let grammar = grammar(CENTRAL).unwrap();
    // *Remain*'s Prepositional frame takes no central Adjunct, and its
    // Copular one yields to that frame's locative Complement.
    assert!(
        grammar
            .parse(&lexicon, "Remain still on top.")
            .readings()
            .next()
            .is_none()
    );
    assert_eq!(
        grammar.parse(&lexicon, "Remain on top.").readings().count(),
        1
    );
}

#[test]
fn a_central_adjunct_needs_its_heads_and_a_verb() {
    for (valence, defect) in [
        (
            "valence VerbPhrase { head: Verb, central: CentralAdjunct }",
            "needs `central_heads`",
        ),
        (
            "valence VerbPhrase { head: Verb, central: CentralAdjunct, central_heads: Nonesuch }",
            "unknown class `Nonesuch`",
        ),
        (
            "valence VerbPhrase { head: Verb }\nvalence NounPhrase { head: Noun, central: CentralAdjunct, central_heads: BeVerb }",
            "`central` on a non-verb head",
        ),
    ] {
        let error = grammar(valence).unwrap_err();
        assert!(
            format!("{error:?}").contains(defect),
            "{valence}: {error:?}"
        );
    }
}

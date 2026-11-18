//! A Valence schema's `interleaved_core` equations constrain an `interleaved`
//! Adjunct that stands before any selected Complement but a passive's
//! Internalized Complement, a non-core Complement (CGEL, Ch. 4 §3.2.1, p. 241):
//! an Adjunct that interleaves only before the *by* phrase fails them. Raiding
//! Party's *weren't chosen this way by any player* has *this way* before the
//! Internalized Complement, while Beast Hunt's *Put all creature cards revealed
//! this way into your hand* has it only after the Object. Here *thus* stands
//! for *this way* and *soon* for any other Adjunct.

use sylex_lexicon::Category;
use sylex_lexicon::Countability;
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
        path: "tests/interleaved_core.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::verb("v:reduce", "reduce", core()),
        Lexeme::noun("n:Zombie", "Zombie", vec![Countability::Count], core()),
        Lexeme::invariant("b:by", "by", Category::Preposition, core()),
        Lexeme::invariant("a:thus", "thus", Category::Adverb, core()),
        Lexeme::invariant("a:soon", "soon", Category::Adverb, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:reduce": [
            (family: Monotransitive, slots: [(relation: Object, fill: NP)]),
            (family: TransitiveWithPP, slots: [
                (relation: Object, fill: NP),
                (relation: Complement, fill: PP(["b:by"])),
            ]),
        ],
    },
    prepositions: {
        "b:by": [(complement: NP, hosts: ComplementOnly)],
    },
)"#;

fn grammar(lexicon: &Lexicon, core: &str) -> CompiledGrammar {
    let source = format!(
        r#"
            root Sentence
            feature Manner = Thus | Other
            feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
            feature Voice = Active | Passive
            category Interleaved [Manner] {{ function }}
            category Nominal
            category NounPhrase
            category PrepositionPhrase
            category Sentence
            category VerbPhrase [VerbForm, Voice]
            word By = "b:by"
            word Thus = "a:thus"
            word Soon = "a:soon"
            slot NP = NounPhrase
            slot PP = PrepositionPhrase
            Passive: Sentence -> clause:VerbPhrase '.' {{ capital: [clause] }}
                where clause.VerbForm = PastParticiple, clause.Voice = Passive
            Active: Sentence -> clause:VerbPhrase '!' {{ capital: [clause] }}
                where clause.VerbForm = Plain, clause.Voice = Active
            Bare: NounPhrase -> Nominal
            Way: Interleaved -> Word(Thus) where $.Manner = Thus
            Time: Interleaved -> Word(Soon) where $.Manner = Other
            valence VerbPhrase {{
                head: Verb,
                active: [$.Voice = Active],
                passive: By,
                verbal_passive: [$.Voice = Passive, $.VerbForm = PastParticiple],
                prepositional_passive: [$.Voice = Passive, $.VerbForm = PastParticiple],
                interleaved: Interleaved,
                {core}
            }}
            valence PrepositionPhrase {{ head: Preposition }}
            valence Nominal {{ head: Noun }}
        "#
    );
    CompiledGrammar::bind(
        &Grammar::from_fug_str(&source).unwrap(),
        lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap()
}

const CORE: &str = "interleaved_core: [adjunct.Manner != Thus],";

/// The frame families of the text's Readings under the schema with `core`
/// spliced in, sorted.
fn families(core: &str, text: &str) -> Vec<FrameFamily> {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, core);
    let readings = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
    for reading in &readings {
        assert_eq!(
            grammar.admit(reading, &lexicon, &sylex_lexicon::CardScope::default()),
            Ok(()),
            "{text}"
        );
    }
    let mut families: Vec<_> = readings
        .iter()
        .map(|reading| {
            let clause = reading.node().unwrap().children[0].node().unwrap();
            clause.selected_frame.as_ref().unwrap().family
        })
        .collect();
    families.sort_by_key(|family| format!("{family:?}"));
    families
}

const BOTH: [FrameFamily; 2] = [FrameFamily::Monotransitive, FrameFamily::TransitiveWithPP];

#[test]
fn an_adjunct_interleaves_before_an_internalized_complement() {
    // Raiding Party: *chosen this way by any player*. Unconstrained, the *by*
    // phrase is the Internalized Complement or the selected one.
    assert_eq!(families("", "Reduced thus by Zombies."), BOTH);
    // The core equations refuse the selected reading alone.
    assert_eq!(
        families(CORE, "Reduced thus by Zombies."),
        [FrameFamily::Monotransitive]
    );
}

#[test]
fn an_adjunct_failing_the_core_equations_stands_before_no_other_complement() {
    // Beast Hunt: *this way* does not stand before a selected phrase.
    assert_eq!(
        families("", "Reduce Zombies thus by Zombies!"),
        [FrameFamily::TransitiveWithPP]
    );
    assert_eq!(families(CORE, "Reduce Zombies thus by Zombies!").len(), 0);
}

#[test]
fn an_adjunct_passing_the_core_equations_interleaves_anywhere() {
    assert_eq!(
        families(CORE, "Reduce Zombies soon by Zombies!"),
        [FrameFamily::TransitiveWithPP]
    );
    assert_eq!(families(CORE, "Reduced soon by Zombies."), BOTH);
}

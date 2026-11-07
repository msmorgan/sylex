//! Right Node Raising set off by commas (`raising_delimiter`: *some amount
//! of, or all of, the mana*, Illusionary Mask), and a Preposition Phrase that
//! passes up no Gap from inside its Complement (`closed_complement`).

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
        path: "tests/raising_gaps.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("d:some", "some", Category::Determinative, core()),
        Lexeme::invariant("c:or", "or", Category::Coordinator, core()),
        Lexeme::invariant("p:of", "of", Category::Preposition, core()),
        Lexeme::noun("n:amount", "amount", vec![Countability::Count], core()),
        Lexeme::noun("n:mana", "mana", vec![Countability::Mass], core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "n:amount": [(family: NounComplement, slots: [(relation: Complement, fill: PP(["p:of"]))])],
    },
    prepositions: {
        "p:of": [(complement: NP, hosts: Nominal)],
    },
)"#;

/// The grammar, with `coordination` and `preposition` the fields of the Noun
/// Phrase coordination and the Preposition Phrase valence.
fn grammar(
    lexicon: &Lexicon,
    coordination: &str,
    preposition: &str,
) -> Result<CompiledGrammar, String> {
    let text = format!(
        r#"
            root Sentence
            category Sentence
            category NounPhrase {{ gap }}
            category Nominal {{ gap }}
            category PrepositionPhrase {{ gap }}
            slot NP = NounPhrase
            slot PP = PrepositionPhrase
            word Some = "d:some"
            word Or = "c:or"
            Statement: Sentence -> NounPhrase '.'
            Bare: NounPhrase -> Nominal {{ transparent: [head] }}
            Determined: NounPhrase -> determiner:Word(Some) Nominal {{ transparent: [head] }}
            valence Nominal {{ head: Noun }}
            valence PrepositionPhrase {{ head: Preposition, {preposition} }}
            coordination NounPhrase {{ coordinators: Or, {coordination} }}
        "#
    );
    let grammar = Grammar::from_fug_str(&text).map_err(|e| format!("{e:?}"))?;
    CompiledGrammar::bind(
        &grammar,
        lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .map_err(|e| format!("{e:?}"))
}

fn count(coordination: &str, preposition: &str, text: &str) -> usize {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, coordination, preposition).unwrap();
    let found = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
    for reading in &found {
        assert_eq!(grammar.admit(reading, &lexicon), Ok(()), "{text}");
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    found.len()
}

#[test]
fn a_raising_delimiter_sets_off_the_final_coordinate() {
    let text = "Some amount of, or some amount of, mana.";
    assert_eq!(count("raising_delimiter: ','", "", text), 1);
    assert_eq!(count("", "", text), 0);
    // One comma alone sets off nothing.
    assert_eq!(
        count(
            "raising_delimiter: ','",
            "",
            "Some amount of, or some amount of mana."
        ),
        0
    );
    assert_eq!(
        count(
            "raising_delimiter: ','",
            "",
            "Some amount of or some amount of, mana."
        ),
        0
    );
}

#[test]
fn a_closed_complement_passes_up_no_gap() {
    // Shared by the conjuncts *amount of __*, or by *amount of amount of __*
    // and *amount of __*, whose Gap is inside the Complement of *of*.
    let text = "Amount of amount of or amount of mana.";
    let open = count("", "", text);
    assert_eq!(count("", "closed_complement", text), open - 1);
}

#[test]
fn a_raising_delimiter_needs_right_node_raising() {
    let lexicon = lexicon();
    let text = r#"
        root Sentence
        category Sentence
        category NounPhrase
        word Or = "c:or"
        Statement: Sentence -> NounPhrase '.'
        Name: NounPhrase -> Lex(Noun)
        coordination NounPhrase { coordinators: Or, raising_delimiter: ',' }
    "#;
    let grammar = Grammar::from_fug_str(text).unwrap();
    let error = CompiledGrammar::bind(&grammar, &lexicon, &FrameCatalog::default()).unwrap_err();
    assert!(
        format!("{error:?}").contains("raising_delimiter"),
        "{error:?}"
    );
}

#[test]
fn a_closed_complement_is_a_preposition_s() {
    let lexicon = lexicon();
    let text = r"
        root Nominal
        category Nominal { gap }
        valence Nominal { head: Noun, closed_complement }
    ";
    let grammar = Grammar::from_fug_str(text).unwrap();
    let error = CompiledGrammar::bind(
        &grammar,
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap_err();
    assert!(
        format!("{error:?}").contains("closed_complement"),
        "{error:?}"
    );
}

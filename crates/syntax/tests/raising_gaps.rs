//! Right Node Raising set off by commas (`raising_delimiter`: *some amount
//! of, or all of, the mana*, Illusionary Mask), a Preposition Phrase that
//! passes up no Gap from inside its Complement (`closed_complement`), and a
//! Gap reaching a Shared Complement through an Adjectival Passive's `by`
//! phrase (*blocks or becomes blocked by a creature*, Dream Fighter).

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
        assert_eq!(
            grammar.admit(reading, &lexicon, &sylex_lexicon::CardScope::default()),
            Ok(()),
            "{text}"
        );
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

#[test]
fn an_adjectival_passive_passes_a_gap_through_its_by_phrase() {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/raising_gaps.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let lexicon = Lexicon::new([
        Lexeme::verb("v:block", "block", core()),
        Lexeme::verb("v:become", "become", core()),
        Lexeme::noun("n:creature", "creature", vec![Countability::Count], core()),
        Lexeme::invariant("c:or", "or", Category::Coordinator, core()),
        Lexeme::invariant("p:by", "by", Category::Preposition, core()),
    ])
    .unwrap();
    let frames = FrameCatalog::from_ron_str(
        r#"(
            frames: {
                "v:block": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
                "v:become": [(family: Copular, slots: [(relation: PredicativeComplement, fill: Pred)])],
            },
            prepositions: {
                "p:by": [(complement: NP, hosts: ComplementOnly)],
            },
        )"#,
    )
    .unwrap();
    let parse_count = |adjective_gap: &str, predicative_gap: &str| -> usize {
        let source = format!(
            r#"
                root Sentence
                feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
                feature Voice = Active | Passive
                category Sentence
                category VerbPhrase [VerbForm, Voice] {{ gap }}
                category Predicative {{ {predicative_gap}function }}
                category AdjectivePhrase [Voice] {adjective_gap}
                category PrepositionPhrase {{ gap }}
                category NounPhrase {{ gap }}
                slot NP = NounPhrase
                slot PP = PrepositionPhrase
                slot Pred = Predicative
                word By = "p:by"
                word Or = "c:or"
                Statement: Sentence -> clause:VerbPhrase '.' where clause.VerbForm = Finite
                AdjectivalPredicative: Predicative -> AdjectivePhrase {{ transparent: [head] }}
                Bare: NounPhrase -> Lex(Noun)
                valence VerbPhrase {{
                    head: Verb,
                    active: [$.Voice = Active],
                    passive: By,
                    verbal_passive: [$.Voice = Passive, $.VerbForm = PastParticiple],
                    prepositional_passive: [$.Voice = Passive, $.VerbForm = PastParticiple],
                    adjectival: AdjectivePhrase,
                    adjectival_passive: [head.VerbForm = PastParticiple, $.Voice = Passive],
                }}
                valence PrepositionPhrase {{ head: Preposition }}
                coordination VerbPhrase {{ coordinators: Or }}
            "#
        );
        let grammar =
            CompiledGrammar::bind(&Grammar::from_fug_str(&source).unwrap(), &lexicon, &frames)
                .unwrap();
        let text = "Blocks or becomes blocked by creature.";
        let readings = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
        for reading in &readings {
            assert_eq!(
                grammar.admit(reading, &lexicon, &sylex_lexicon::CardScope::default()),
                Ok(()),
                "{text}"
            );
            assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
        }
        readings.len()
    };

    // With both `Predicative` and `AdjectivePhrase` carrying a Gap, the shared
    // right-hand NP fills `blocks`'s Object and the adjectival passive's `by`
    // Complement; closing either Category blocks propagation.
    assert_eq!(parse_count("{ gap }", "gap, "), 1);
    assert_eq!(parse_count("", "gap, "), 0);
    assert_eq!(parse_count("{ gap }", ""), 0);
}

#[test]
fn a_gapped_adjunct_raises_its_gap_and_yields_to_a_selected_infinitival() {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/raising_gaps.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let lexicon = Lexicon::new([
        Lexeme::verb("v:cost", "cost", core()),
        Lexeme::verb("v:spend", "spend", core()),
        Lexeme::verb("v:cast", "cast", core()),
        Lexeme::noun("n:spell", "spell", vec![Countability::Count], core()),
        Lexeme::noun("n:mana", "mana", vec![Countability::Mass], core()),
        Lexeme::invariant("s:to", "to", Category::Subordinator, core()),
    ])
    .unwrap();
    let frames = FrameCatalog::from_ron_str(
        r#"(
            frames: {
                "v:cost": [
                    (family: Monotransitive, slots: [(relation: Object, fill: NP)]),
                    (family: TransitiveWithComplement, slots: [
                        (relation: Object, fill: NP),
                        (relation: Complement, fill: HollowToInf),
                    ]),
                ],
                "v:spend": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
                "v:cast": [
                    (family: Intransitive, slots: []),
                    (family: Monotransitive, slots: [(relation: Object, fill: NP)]),
                ],
            },
        )"#,
    )
    .unwrap();
    let source = r#"
        root Sentence
        feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
        category Sentence
        category NounPhrase { gap }
        category Nominal { gap }
        category Clause { gap }
        category ToInf : Clause { gap }
        category HollowInf : Clause
        category VerbPhrase [VerbForm] { gap }
        slot NP = NounPhrase
        slot ToInf = ToInf
        slot HollowToInf = HollowInf
        word To = "s:to"
        Statement: Sentence -> subject:NounPhrase clause:VerbPhrase '.' where clause.VerbForm = Finite
        Bare: NounPhrase -> Nominal { transparent: [head] }
        Head: Nominal -> Lex(Noun)
        BareRelative: Nominal -> Nominal modifier:Clause
            where modifier.Gap = NP
        RelClause: Clause -> subject:NounPhrase VerbPhrase { transparent: [head] }
            where head.VerbForm = Finite
        ToInfinitival: ToInf -> marker:Word(To) VerbPhrase { transparent: [head] }
            where head.VerbForm = Plain
        HollowInfinitival: HollowInf -> marker:Word(To) VerbPhrase
            where head.VerbForm = Plain,
                  head.Gap = NP
        valence VerbPhrase { head: Verb }
        adjunct ToInf { host: VerbPhrase, side: After, host_kind: Predicate }
        adjunct ToInf { host: VerbPhrase, side: After, host_kind: Predicate }
            where adjunct.Gap = NP
    "#;
    let grammar =
        CompiledGrammar::bind(&Grammar::from_fug_str(source).unwrap(), &lexicon, &frames).unwrap();

    // Non-selecting *spend* keeps its gapped purpose adjunct inside a relative
    // clause.
    let spend_readings = grammar
        .parse(&lexicon, "Spell mana spends mana to cast casts.")
        .readings()
        .collect::<Vec<_>>();
    assert_eq!(spend_readings.len(), 1, "{spend_readings:#?}");
    assert_eq!(
        grammar.admit(
            &spend_readings[0],
            &lexicon,
            &sylex_lexicon::CardScope::default()
        ),
        Ok(())
    );

    // Selecting *cost* (`HollowToInf`) suppresses both ungapped and gapped
    // `adjunct ToInf`: main-clause `costs mana to cast` takes
    // `HollowInfinitival` alone, and relative-clause `mana costs mana to cast`
    // cannot license a relative Gap through gapped `adjunct ToInf`.
    let cost_readings = grammar
        .parse(&lexicon, "Spell costs mana to cast.")
        .readings()
        .collect::<Vec<_>>();
    assert_eq!(cost_readings.len(), 1, "{cost_readings:#?}");
    assert_eq!(
        grammar.admit(
            &cost_readings[0],
            &lexicon,
            &sylex_lexicon::CardScope::default()
        ),
        Ok(())
    );
    assert_eq!(
        grammar
            .parse(&lexicon, "Spell mana costs mana to cast casts.")
            .readings()
            .count(),
        0
    );

    // A `.Gap` requirement on `head` or a comma-delimited adjunct is rejected
    // at bind time.
    for bad in [
        "adjunct ToInf { host: VerbPhrase, side: After, host_kind: Predicate } where head.Gap = NP",
        "adjunct ToInf { host: VerbPhrase, side: After, host_kind: Predicate, delimiter: ',' } where adjunct.Gap = NP",
    ] {
        let bad_source = format!("{source}\n        {bad}");
        let err = CompiledGrammar::bind(
            &Grammar::from_fug_str(&bad_source).unwrap(),
            &lexicon,
            &frames,
        )
        .unwrap_err();
        assert!(format!("{err:?}").contains("Gap"), "{err:?}");
    }
}

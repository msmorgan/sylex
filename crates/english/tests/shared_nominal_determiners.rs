//! Coordinated determiners each with its own premodifiers over one shared
//! nominal (`SharedNominalDetermined`, CGEL 5§14.4:449, 15§4.4(d):1344),
//! agreement across conjuncts and the clause, canonicalization beside bare
//! determiner coordination, and the admission gate that a Cluster's
//! `Premodifier` is no noun
//! (`docs/design/english.md` §6, §8).

use sylex_lexicon::CardScope;
use sylex_lexicon::LexicalReading;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Grammar;
use sylex_syntax::Reading;
use sylex_syntax::ReadingNode;
use sylex_vocabulary::LexicalEnvironment;

/// Word lexeme ids in surface order under `reading`.
fn word_lexemes(reading: &Reading, out: &mut Vec<String>) {
    match reading {
        Reading::Node(node) => {
            for child in &node.children {
                word_lexemes(child, out);
            }
        }
        Reading::Leaf(LexicalReading::Word(word)) => out.push(word.lexeme.to_string()),
        Reading::Leaf(
            LexicalReading::OwnName { .. }
            | LexicalReading::IntroducedName { .. }
            | LexicalReading::Numeral { .. }
            | LexicalReading::FlavorWord { .. }
            | LexicalReading::SetName { .. }
            | LexicalReading::AnchorWord { .. },
        ) => {}
    }
}

/// Every `ReadingNode` built by `production` under `reading`.
fn nodes_with_production<'a>(
    reading: &'a Reading,
    production: &str,
    out: &mut Vec<&'a ReadingNode>,
) {
    match reading {
        Reading::Node(node) => {
            if node.production == production {
                out.push(node);
            }
            for child in &node.children {
                nodes_with_production(child, production, out);
            }
        }
        Reading::Leaf(_) => {}
    }
}

/// Ranked Readings of `text`, optionally on `card`.
fn parse_readings(
    grammar: &CompiledGrammar,
    env: &LexicalEnvironment,
    scope: &CardScope,
    text: &str,
) -> Vec<Reading> {
    let analyzed = env.lexicon.analyze_source_scoped(text, None, scope);
    grammar.parse_analyzed(&analyzed).readings().collect()
}

/// Asserts that `reading` contains one `SharedNominalDetermined` node whose
/// coordinated `NominalCluster` conjuncts have the expected lexeme sequences,
/// whose shared `Nominal` contains `expected_nominal` headed by
/// `expected_head`, and whose determiners and premodifiers bear `determiner`
/// and `modifier` in the shared nominal's `NounPhrase` (`Constituent::also`,
/// CGEL 15§4.4(d):1344 [25]) without forming a `DeterminerPhrase` over any
/// cluster.
#[expect(
    clippy::too_many_arguments,
    reason = "explicit structural and function assertions"
)]
fn has_shared_nominal_reading(
    grammar: &CompiledGrammar,
    env: &LexicalEnvironment,
    reading: &Reading,
    first_conjunct_lexemes: &[&str],
    second_conjunct_lexemes: &[&str],
    expected_nominal: &str,
    expected_head: &str,
    expected_modifiers: &[(&str, &str)],
) -> bool {
    let mut shared = Vec::new();
    nodes_with_production(reading, "SharedNominalDetermined", &mut shared);
    if shared.len() != 1 {
        return false;
    }
    let np = shared[0];
    assert_eq!(np.children.len(), 2);
    let Reading::Node(coord) = &np.children[0] else {
        panic!("clusters child of SharedNominalDetermined must be a Node");
    };
    assert_eq!(coord.category, "NominalCluster");
    assert_eq!(coord.children.len(), 3);

    let mut first_words = Vec::new();
    word_lexemes(&coord.children[0], &mut first_words);
    let mut second_words = Vec::new();
    word_lexemes(&coord.children[2], &mut second_words);

    let (realized, constituents) = grammar.constituents(reading, &env.lexicon).unwrap();
    let has_nominal = constituents
        .iter()
        .any(|c| c.category == "Nominal" && realized.get(c.span.clone()) == Some(expected_nominal));
    let Some(clusters_coord) = constituents
        .iter()
        .find(|c| c.function.map(|f| f.as_str()) == Some("clusters"))
    else {
        return false;
    };
    let Some(np_index) = clusters_coord.parent else {
        return false;
    };
    if constituents[np_index].category != "NounPhrase"
        || constituents[np_index].head.map(|id| id.as_str()) != Some(expected_head)
        || !realized
            .get(constituents[np_index].span.clone())
            .is_some_and(|s| s.ends_with(expected_nominal))
    {
        return false;
    }

    // No DeterminerPhrase spans an entire cluster or cluster coordination, and
    // no cluster carries a head of its own.
    for cluster in constituents
        .iter()
        .filter(|c| c.category == "NominalCluster")
    {
        assert_eq!(cluster.head, None);
        let cluster_surface = &realized[cluster.span.clone()];
        assert!(
            !constituents.iter().any(|c| c.category == "DeterminerPhrase"
                && &realized[c.span.clone()] == cluster_surface),
            "cluster `{cluster_surface}` must not be a DeterminerPhrase"
        );
    }

    // Both conjunct determiners bear `determiner` in the shared NounPhrase.
    let cluster_determiners: Vec<_> = constituents
        .iter()
        .filter(|c| {
            c.category == "DeterminerPhrase"
                && c.also.contains(&(dpsi::Ident::new("determiner"), np_index))
        })
        .collect();
    if cluster_determiners.len() != 2 {
        return false;
    }

    // Every premodifier bears `modifier` in the shared NounPhrase, never on a
    // determiner.
    for &(category, surface) in expected_modifiers {
        let matched = constituents.iter().any(|c| {
            c.category == category
                && realized.get(c.span.clone()) == Some(surface)
                && c.also == [(dpsi::Ident::new("modifier"), np_index)]
                && c.parent.is_some_and(|p| constituents[p].head.is_none())
        });
        if !matched {
            return false;
        }
    }

    first_words == first_conjunct_lexemes && second_words == second_conjunct_lexemes && has_nominal
}

#[test]
fn balthor_and_participial_shared_nominal_determiners() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();

    let balthor_line = "{B}{B}{B}, Exile Balthor: Each player returns all black and all red creature cards from their graveyard to the battlefield.";
    let balthor = parse_readings(
        &grammar,
        &env,
        &CardScope::every_form(["Balthor the Defiled"]),
        balthor_line,
    );

    assert!(
        balthor.iter().any(|reading| {
            let mut wide_coord = Vec::new();
            nodes_with_production(
                reading,
                "CoordinatedNounPhrasePostmodifier",
                &mut wide_coord,
            );
            wide_coord.len() == 1
        }),
        "expected a wide CoordinatedNounPhrasePostmodifier reading: {balthor:#?}",
    );
    assert!(
        balthor.iter().any(|reading| {
            has_shared_nominal_reading(
                &grammar,
                &env,
                reading,
                &["det:all.count", "adj:black"],
                &["det:all.count", "adj:red"],
                "creature cards from their graveyard",
                "noun:card",
                &[("AdjectivePhrase", "black"), ("AdjectivePhrase", "red")],
            )
        }),
        "expected SharedNominalDetermined over `creature cards from their graveyard`: {balthor:#?}",
    );
    assert!(
        balthor.iter().any(|reading| {
            has_shared_nominal_reading(
                &grammar,
                &env,
                reading,
                &["det:all.count", "adj:black"],
                &["det:all.count", "adj:red"],
                "creature cards",
                "noun:card",
                &[("AdjectivePhrase", "black"), ("AdjectivePhrase", "red")],
            )
        }),
        "expected SharedNominalDetermined over `creature cards`: {balthor:#?}",
    );

    // Ticket witness derived from Balthor the Defiled and Evacuation:
    let participial_line =
        "Each player returns all tapped and all attacking creatures to their owners' hands.";
    let participial = parse_readings(&grammar, &env, &CardScope::default(), participial_line);
    assert_eq!(participial.len(), 1);
    assert!(
        has_shared_nominal_reading(
            &grammar,
            &env,
            &participial[0],
            &["det:all.count", "status_adj:tapped"],
            &["det:all.count", "verb:attack"],
            "creatures",
            "type:creature",
            &[("AdjectivePhrase", "tapped"), ("VerbPhrase", "attacking")],
        ),
        "participial reading: {:#?}",
        participial[0]
    );

    // Stacked premodifiers in a cluster (derived from Balthor the Defiled and
    // Evacuation): one analysis attaches "blocking" inside the second cluster
    // ([all untapped blocking] + [creature cards]); the other attaches
    // "blocking" on the shared nominal ([all untapped] + [blocking creature
    // cards]).
    let stacked_line = "Each player returns all tapped attacking and all untapped blocking creature cards to their owners' hands.";
    let stacked = parse_readings(&grammar, &env, &CardScope::default(), stacked_line);
    assert_eq!(stacked.len(), 2);
    assert!(
        stacked.iter().any(|reading| {
            has_shared_nominal_reading(
                &grammar,
                &env,
                reading,
                &["det:all.count", "status_adj:tapped", "verb:attack"],
                &["det:all.count", "status_adj:untapped", "verb:block"],
                "creature cards",
                "noun:card",
                &[
                    ("AdjectivePhrase", "tapped"),
                    ("VerbPhrase", "attacking"),
                    ("AdjectivePhrase", "untapped"),
                    ("VerbPhrase", "blocking"),
                ],
            )
        }),
        "expected cluster-internal `blocking` reading: {stacked:#?}",
    );
    assert!(
        stacked.iter().any(|reading| {
            has_shared_nominal_reading(
                &grammar,
                &env,
                reading,
                &["det:all.count", "status_adj:tapped", "verb:attack"],
                &["det:all.count", "status_adj:untapped"],
                "blocking creature cards",
                "noun:card",
                &[
                    ("AdjectivePhrase", "tapped"),
                    ("VerbPhrase", "attacking"),
                    ("AdjectivePhrase", "untapped"),
                ],
            )
        }),
        "expected shared-nominal `blocking creature cards` reading: {stacked:#?}",
    );
}

#[test]
fn number_agreement_across_conjuncts_and_clause() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();

    // Minimal agreement variants derived from Balthor the Defiled:
    for grammatical in [
        "A tapped and an untapped creature fight.",
        "Each tapped and each untapped creature gets +1/+1 until end of turn.",
        "Target tapped or target untapped creature gets +1/+1 until end of turn.",
        "Two tapped or two untapped creatures fight.",
        "A tapped and/or an untapped creature gets +1/+1 until end of turn.",
        "A tapped and/or an untapped creature get +1/+1 until end of turn.",
    ] {
        let readings = parse_readings(&grammar, &env, &CardScope::default(), grammatical);
        assert_eq!(readings.len(), 1, "{grammatical}");
    }

    for ungrammatical in [
        "A tapped and an untapped creature fights.",
        "A tapped and an untapped creatures fight.",
        "All tapped and an untapped creature fight.",
        "All tapped and an untapped creatures fight.",
        "Each tapped and each untapped creature get +1/+1 until end of turn.",
        "Target tapped or target untapped creature get +1/+1 until end of turn.",
    ] {
        let readings = parse_readings(&grammar, &env, &CardScope::default(), ungrammatical);
        assert!(readings.is_empty(), "{ungrammatical}: {readings:#?}");
    }
}

#[test]
fn bare_determiners_stay_on_the_canonical_path() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();

    // Counterintelligence: bare coordinated determiners keep a single Reading.
    let counterintelligence = parse_readings(
        &grammar,
        &env,
        &CardScope::default(),
        "Return one or two target creatures to their owners' hands.",
    );
    assert_eq!(counterintelligence.len(), 1);

    // Simplified from Balthor the Defiled: bare determiner coordination +
    // premodifier.
    let analyzed = env.lexicon.analyze("one or two black creatures");
    let readings: Vec<Reading> = grammar
        .parse_category(&analyzed, "NounPhrase")
        .unwrap()
        .readings()
        .collect();
    assert_eq!(readings.len(), 1);
}

#[test]
fn cluster_premodifier_gate_blocks_tautological_noun_doubling() {
    let env = LexicalEnvironment::load().unwrap();
    let gated = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let warpath = "Warpath deals 3 damage to each blocking creature and each blocked creature.";

    let gated_readings = parse_readings(&gated, &env, &CardScope::full_names(["Warpath"]), warpath);
    assert_eq!(gated_readings.len(), 1);

    let ungated_source = format!(
        "{}\nNounPremodifierInCluster: Premodifier -> NounPremodifier\n",
        sylex_english::GRAMMAR.as_str()
    );
    let ungated = CompiledGrammar::bind(
        &Grammar::from_fug_str(&ungated_source).unwrap(),
        &env.lexicon,
        &env.frames,
    )
    .unwrap();
    let ungated_readings =
        parse_readings(&ungated, &env, &CardScope::full_names(["Warpath"]), warpath);
    assert_eq!(ungated_readings.len(), 2);
}

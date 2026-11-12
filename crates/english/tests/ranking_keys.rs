//! The Ranking Key extracted from the Packed Forest equals the key computed
//! from each Reading's tree, over the judgment set.

use sylex_english::judgment::Judge;
use sylex_vocabulary::LexicalEnvironment;

#[test]
fn forest_keys_equal_tree_keys_over_the_judgment_set() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let judge = Judge {
        grammar: &grammar,
        lexicon: &env.lexicon,
    };
    let files = sylex_english::judgment::load_dir(&sylex_english::judgments_dir()).unwrap();
    let mut checked = 0;
    let mut failures = Vec::new();
    for (file, entries) in files {
        if file == "seed.txt" {
            continue;
        }
        for entry in entries.iter().filter(|e| e.grammatical) {
            let parse = judge
                .parse(entry)
                .unwrap_or_else(|why| panic!("{}: {why}", entry.location));
            for (key, reading) in parse.best_ranked(usize::MAX) {
                checked += 1;
                if grammar.ranking_key(&reading, &env.lexicon) != Ok(key) {
                    failures.push(format!("{}: {}", entry.location, entry.text));
                    break;
                }
            }
        }
    }
    assert!(checked > 0);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// A distributive adjunct (*for each …*) attaches high: to the matrix
/// predicate, not inside the hollow infinitival.
#[test]
fn a_distributive_adjunct_attaches_to_the_outermost_predicate() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let judge = Judge {
        grammar: &grammar,
        lexicon: &env.lexicon,
    };
    let entry = &sylex_english::judgment::parse_file(
        "test",
        "+ Creature spells you cast cost {1} less to cast for each +1/+1 counter on this creature. => PrepositionPhrase \"for each +1/+1 counter on this creature\" as Adjunct of cost",
    )
    .unwrap()[0];
    let parse = judge.parse(entry).unwrap();
    let (key, best) = parse.best_ranked(1).next().unwrap();
    assert!(key.right_association_words < 0, "{key:?}");
    let (text, constituents) = grammar.constituents(&best, &env.lexicon).unwrap();
    let located = sylex_english::judgment::Located { text, constituents };
    assert!(judge.endorses(entry, &located));
}

/// On Nicol Bolas, the Deceiver, *Nicol Bolas* is the face's own Shortened
/// Name and also the full name of another card. As a Subject it is the
/// face's own name alone: another card's name stands only after *named*,
/// *Partner with* or *from among*.
#[test]
fn a_subject_spelled_like_another_card_s_name_is_the_face_s_own() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let own = sylex_vocabulary::card_name_id("Nicol Bolas, the Deceiver");
    let scope = sylex_lexicon::CardScope::own_names([own]);
    let text = "Nicol Bolas deals 7 damage to each opponent.";
    let parse = grammar.parse_analyzed(&env.lexicon.analyze_source_scoped(text, None, &scope));
    let ranked = parse.best_ranked(usize::MAX).collect::<Vec<_>>();
    assert_eq!(ranked.len(), 1);
    assert!(
        names(&grammar, &env, &ranked[0].1)
            .iter()
            .all(|h| *h == own)
    );
}

/// After *named*, the face's own Shortened Name and another card's full name
/// spelled alike are two Readings with one key: neither ranks first. A minimal
/// repro on Nicol Bolas, the Ravager, whose own text shortens its name to
/// *Nicol Bolas*; no card prints *named* before its own Shortened Name.
#[test]
fn after_named_a_face_s_own_name_ties_with_another_card_s_spelled_alike() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let own = sylex_vocabulary::card_name_id("Nicol Bolas, the Ravager");
    let scope = sylex_lexicon::CardScope::own_names([own]);
    let text = "Search your library for a card named Nicol Bolas.";
    let parse = grammar.parse_analyzed(&env.lexicon.analyze_source_scoped(text, None, &scope));
    let ranked = parse.best_ranked(usize::MAX).collect::<Vec<_>>();
    assert_eq!(ranked.len(), 2);
    assert_eq!(ranked[0].0, ranked[1].0);
    let other = sylex_vocabulary::card_name_id("Nicol Bolas");
    let mut heads: Vec<_> = ranked
        .iter()
        .map(|(_, reading)| {
            let mut names = names(&grammar, &env, reading);
            names.dedup();
            names
        })
        .collect();
    heads.sort();
    let mut expected = vec![vec![own], vec![other]];
    expected.sort();
    assert_eq!(heads, expected);
    for (key, reading) in &ranked {
        assert_eq!(grammar.ranking_key(reading, &env.lexicon), Ok(*key));
    }
}

/// The card names that head a Reading's Constituents.
fn names(
    grammar: &sylex_syntax::CompiledGrammar,
    env: &LexicalEnvironment,
    reading: &sylex_syntax::Reading,
) -> Vec<sylex_lexicon::LexemeId> {
    let (_, constituents) = grammar.constituents(reading, &env.lexicon).unwrap();
    constituents
        .iter()
        .filter_map(|c| c.head)
        .filter(|h| env.lexicon.lexemes()[h].card_name)
        .collect()
}

/// One forest item packs prefixes that divide its text differently: here the
/// postposed relative's head and Object meet before or after *tapped*, a
/// depictive in the head or a premodifier in the Object. The Adjunct *at
/// random* reads its head's words, so each prefix keeps its own key, and every
/// Reading's forest key is the key of its own tree.
#[test]
fn forest_keys_follow_each_packed_prefix_s_own_division_of_the_text() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let scope = sylex_lexicon::CardScope::default();
    let text = "Return to the battlefield tapped creature cards at random that were put into your graveyard this turn.";
    let parse = grammar.parse_analyzed(&env.lexicon.analyze_source_scoped(text, None, &scope));
    let ranked = parse.best_ranked(usize::MAX).collect::<Vec<_>>();
    let mut objects = std::collections::BTreeSet::new();
    for (key, reading) in &ranked {
        assert_eq!(grammar.ranking_key(reading, &env.lexicon), Ok(*key));
        let (realized, constituents) = grammar.constituents(reading, &env.lexicon).unwrap();
        objects.extend(
            constituents
                .iter()
                .filter(|c| c.function.is_some_and(|f| f.as_str() == "object"))
                .map(|c| realized[c.span.clone()].to_owned()),
        );
    }
    assert!(objects.contains("tapped creature cards"), "{objects:?}");
    assert!(objects.contains("creature cards"), "{objects:?}");
    let (least, _) = parse.cheapest_tier().unwrap();
    assert_eq!(least, ranked[0].0);
}

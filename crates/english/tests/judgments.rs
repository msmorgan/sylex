//! The judgment set (`judgments/README.md`): every file parses; the seed file
//! holds on the seed lexicon; every entry at or below the completed Stratum
//! holds on the full lexical environment, except one whose text is a deferred
//! unit (`deferred.txt`), which is pending while it stays deferred.

#[allow(dead_code, reason = "the seed module also serves seed_proof.rs")]
mod seed;

use std::sync::OnceLock;

use sylex_english::COMPLETED_STRATUM;
use sylex_english::corpus::UnitKind;
use sylex_english::judgment::Judge;
use sylex_english::judgment::Outcome;
use sylex_english::profile::Profiler;
use sylex_syntax::CompiledGrammar;
use sylex_vocabulary::LexicalEnvironment;

fn environment() -> &'static (LexicalEnvironment, CompiledGrammar) {
    static ENVIRONMENT: OnceLock<(LexicalEnvironment, CompiledGrammar)> = OnceLock::new();
    ENVIRONMENT.get_or_init(|| {
        let env = LexicalEnvironment::load().unwrap();
        let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
        (env, grammar)
    })
}

fn files() -> Vec<(String, Vec<sylex_english::judgment::Judgment>)> {
    sylex_english::judgment::load_dir(&sylex_english::judgments_dir())
        .unwrap_or_else(|e| panic!("{e}"))
}

#[test]
fn the_seed_judgments_hold() {
    let lexicon = seed::lexicon();
    let grammar = sylex_english::bind(&lexicon, &seed::frames()).unwrap();
    let judge = Judge {
        grammar: &grammar,
        lexicon: &lexicon,
    };
    let (_, entries) = files()
        .into_iter()
        .find(|(name, _)| name == "seed.txt")
        .expect("judgments/seed.txt");
    let failures: Vec<String> = entries
        .iter()
        .filter_map(|e| match judge.evaluate(e) {
            Outcome::Holds => None,
            Outcome::Fails(why) => Some(format!("{}: {} — {why}", e.location, e.text)),
        })
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn judgments_at_or_below_the_completed_stratum_hold() {
    check_judgments(COMPLETED_STRATUM);
}

#[test]
fn catalog_determinative_judgments_hold() {
    let (env, grammar) = environment();
    let judge = Judge {
        grammar,
        lexicon: &env.lexicon,
    };
    let (_, entries) = files()
        .into_iter()
        .find(|(name, _)| name == "oracle_determinatives.txt")
        .unwrap();
    for entry in entries {
        assert!(
            matches!(judge.evaluate(&entry), Outcome::Holds),
            "{}: {}: {:?}",
            entry.location,
            entry.text,
            judge.evaluate(&entry)
        );
    }
}

fn check_judgments(completed: Option<u8>) {
    let files = files();
    let (env, grammar) = environment();
    let deferred: std::collections::BTreeSet<&str> = sylex_english::census::deferrals()
        .into_iter()
        .map(|(_, text)| text)
        .collect();
    let deferred = &deferred;
    let entries: Vec<_> = files
        .iter()
        .filter(|(name, _)| name != "seed.txt")
        .flat_map(|(_, entries)| entries)
        .collect();
    let threads = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    let chunk = entries.len().div_ceil(threads).max(1);
    let (enforced, pending, failures) = std::thread::scope(|scope| {
        let workers: Vec<_> = entries
            .chunks(chunk)
            .map(|entries| {
                scope.spawn(move || {
                    let profiler = Profiler::new(env);
                    let judge = Judge {
                        grammar,
                        lexicon: &env.lexicon,
                    };
                    let mut enforced = 0;
                    let mut pending = 0;
                    let mut failures = Vec::new();
                    for entry in entries {
                        let scope = match entry.scope(&env.lexicon) {
                            Ok(scope) => scope,
                            Err(why) => {
                                failures.push(why);
                                continue;
                            }
                        };
                        let analyzed = env.lexicon.analyze_source_scoped(&entry.text, None, &scope);
                        let stratum = profiler.profile(UnitKind::Line, &analyzed).stratum;
                        if completed.is_none_or(|done| stratum > done)
                            || deferred.contains(entry.text.as_str())
                        {
                            pending += 1;
                            continue;
                        }
                        enforced += 1;
                        if let Outcome::Fails(why) = judge.evaluate(entry) {
                            failures.push(format!(
                                "{} (Stratum {stratum}): {} — {why}",
                                entry.location, entry.text
                            ));
                        }
                    }
                    (enforced, pending, failures)
                })
            })
            .collect();
        // Joining contiguous chunks in input order keeps diagnostics
        // deterministic.
        workers.into_iter().fold(
            (0, 0, Vec::new()),
            |(enforced, pending, mut failures), worker| {
                let (worker_enforced, worker_pending, worker_failures) =
                    worker.join().expect("judgment worker");
                failures.extend(worker_failures);
                (
                    enforced + worker_enforced,
                    pending + worker_pending,
                    failures,
                )
            },
        )
    });
    println!("judgments: {enforced} enforced, {pending} pending");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn false_judgments_fail() {
    let lexicon = seed::lexicon();
    let grammar = sylex_english::bind(&lexicon, &seed::frames()).unwrap();
    let judge = Judge {
        grammar: &grammar,
        lexicon: &lexicon,
    };
    let false_entries = r##"
+ Destroy target creature. => NounPhrase "target creature" as Subject of destroy
+ Destroy target creature. => NounPhrase "target creature" as Object of exile
+ Destroy target creature. => NounPhrase "creature target"
+ Destroy target creature. => PrepositionPhrase "target creature"
+ Destroy target creature. => readings 2
+ Search your library for a card. => PrepositionPhrase "for a card" as Modifier of library
+ Search your library for a card. => !NounPhrase "a card"
+ Exile target creature with a counter on it. => PrepositionPhrase "with a counter on it" as Adjunct of creature
+ Exile the card you put into your hand. => Clause "you put into your hand" gap "hand"
+ Exile the card you put into your hand. => NounPhrase "the card"#2
+ Put the card.
* Destroy target creature.
+ Destroy target wombat.
*NounPhrase target creature
+Nonesuch target creature
"##;
    for entry in sylex_english::judgment::parse_file("false", false_entries).unwrap() {
        assert!(
            matches!(judge.evaluate(&entry), Outcome::Fails(_)),
            "{}: {}",
            entry.location,
            entry.text
        );
    }
}

#[test]
fn every_word_a_judgment_names_is_declared() {
    let (env, _) = environment();
    let lexemes = env.lexicon.lexemes();
    // A card name is no lemma: *Greater Good* does not declare *greater*.
    let declared = |name: &str| {
        lexemes.iter().any(|(id, l)| {
            id.as_str() == name || !l.card_name && l.lemma.eq_ignore_ascii_case(name)
        })
    };
    let mut unknown = Vec::new();
    for (name, entries) in files() {
        if name == "seed.txt" {
            continue;
        }
        for entry in entries {
            for assertion in entry.groups.iter().flatten().chain(&entry.negated) {
                let words = assertion
                    .head
                    .iter()
                    .chain(assertion.function.iter().map(|(_, of)| of));
                for word in words.filter(|w| !declared(w)) {
                    unknown.push(format!("{}: {word}", entry.location));
                }
            }
        }
    }
    assert!(
        unknown.is_empty(),
        "not a lemma or lexeme id:\n{}",
        unknown.join("\n")
    );
}

#[test]
fn stratum_zero_spines_hold_over_seed_payloads() {
    let (env, grammar) = environment();
    let judge = Judge {
        grammar,
        lexicon: &env.lexicon,
    };
    let (_, entries) = files()
        .into_iter()
        .find(|(name, _)| name == "stratum_zero.txt")
        .unwrap();
    let failures: Vec<_> = entries
        .iter()
        .filter_map(|entry| match judge.evaluate(entry) {
            Outcome::Holds => None,
            Outcome::Fails(why) => Some(format!("{}: {} — {why}", entry.location, entry.text)),
        })
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// An entry's Card Scope is computed from the name it states, with no card
/// looked up: *Jace, Cunning Castaway* shortens its own name; *Insectile
/// Aberration* is the back face of *Delver of Secrets*, with a name of its own
/// [CR#712.6]; *Gisela, the Broken Blade* is a meld card, whose
/// scope does not hold its meld result.
#[test]
fn an_entry_s_scope_is_the_own_name_of_the_card_it_states() {
    use sylex_lexicon::CardScope;
    use sylex_lexicon::NameForm;
    use sylex_vocabulary::card_name_id;

    let (env, _) = environment();
    let entry = |line: &str| {
        sylex_english::judgment::parse_file("test", line)
            .unwrap()
            .remove(0)
    };
    assert_eq!(
        entry("+ Draw a card.").scope(&env.lexicon),
        Ok(CardScope::default())
    );
    for name in [
        "Jace, Cunning Castaway",
        "Insectile Aberration",
        "Gisela, the Broken Blade",
    ] {
        let scope = entry(&format!("+ <card={name}> Draw a card."))
            .scope(&env.lexicon)
            .unwrap();
        assert_eq!(scope, CardScope::own_names([card_name_id(name)]), "{name}");
        assert!(scope.holds(card_name_id(name), NameForm::Shortened));
    }
    let gisela = entry("+ <card=Gisela, the Broken Blade> Draw a card.")
        .scope(&env.lexicon)
        .unwrap();
    assert!(!gisela.holds_card(card_name_id("Brisela, Voice of Nightmares")));
}

/// A stated name that is no card name is refused: a misspelled name
/// (*Lightning Blot*), and a whole card's combined name, which is the name
/// of no face (*Delver of Secrets // Insectile Aberration*).
#[test]
fn a_stated_name_that_is_no_card_name_is_refused_naming_the_entry() {
    let (env, _) = environment();
    for name in [
        "Lightning Blot",
        "Delver of Secrets // Insectile Aberration",
    ] {
        let entry =
            sylex_english::judgment::parse_file("test", &format!("+ <card={name}> Draw a card."))
                .unwrap()
                .remove(0);
        assert_eq!(
            entry.scope(&env.lexicon),
            Err(format!(
                "test:1: `<card={name}>` states a name that is no card name of the lexicon"
            ))
        );
    }
}

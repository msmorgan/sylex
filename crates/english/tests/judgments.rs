//! The judgment set (`judgments/README.md`): every file parses; the seed file
//! holds on the seed lexicon; every entry at or below the completed Stratum
//! holds on the full lexical environment, except one whose text is a deferred
//! unit (`deferred.txt`), which is pending while it stays deferred.

#[allow(dead_code, reason = "the seed module also serves seed_proof.rs")]
mod seed;

use std::sync::OnceLock;

use sylex_english::COMPLETED_STRATUM;
use sylex_english::corpus::Corpus;
use sylex_english::corpus::UnitKind;
use sylex_english::judgment::CardScopes;
use sylex_english::judgment::Judge;
use sylex_english::judgment::Outcome;
use sylex_english::profile::Profiler;
use sylex_syntax::CompiledGrammar;
use sylex_vocabulary::LexicalEnvironment;

fn environment() -> &'static (LexicalEnvironment, CompiledGrammar, CardScopes) {
    static ENVIRONMENT: OnceLock<(LexicalEnvironment, CompiledGrammar, CardScopes)> =
        OnceLock::new();
    ENVIRONMENT.get_or_init(|| {
        let env = LexicalEnvironment::load().unwrap();
        let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
        let corpus = Corpus::load(|_| true).unwrap();
        (env, grammar, CardScopes::new(&corpus.faces))
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
        scopes: &CardScopes::default(),
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
    let (env, grammar, scopes) = environment();
    let judge = Judge {
        grammar,
        lexicon: &env.lexicon,
        scopes,
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
    let (env, grammar, scopes) = environment();
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
                        scopes,
                    };
                    let mut enforced = 0;
                    let mut pending = 0;
                    let mut failures = Vec::new();
                    for entry in entries {
                        let scope = match scopes.scope(entry) {
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
        scopes: &CardScopes::default(),
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
    let (env, _, _) = environment();
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
    let (env, grammar, scopes) = environment();
    let judge = Judge {
        grammar,
        lexicon: &env.lexicon,
        scopes,
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

//! A unit's forks (`sylex_syntax::ForkForest::forks`) as rows of the
//! `words`, `forks`, `fork_aliases`, `fork_options` and `fork_answers`
//! tables.

use anyhow::Context;
use anyhow::Result;
use rusqlite::params;
use sylex_syntax::Answer;
use sylex_syntax::Dependency;
use sylex_syntax::ForkKind;
use sylex_syntax::ForkWord;
use sylex_syntax::Parse;

use super::LeafRow;
use super::index;
use super::int;

/// A unit's words and forks, with words and leaves by index.
pub(super) struct ForkRows {
    words: Vec<ForkWord>,
    forks: Vec<ForkRow>,
}

struct ForkRow {
    kind: ForkKind,
    word: usize,
    aliases: Vec<(ForkKind, usize)>,
    context: u128,
    conditional: bool,
    options: Vec<OptionRow>,
}

struct OptionRow {
    derivations: u128,
    /// The answer to the fork's own question, then to each alias's.
    answers: Vec<AnswerRow>,
}

enum AnswerRow {
    /// The leaf starting at the word with the answer's lexical reading.
    Lexical(usize),
    Root,
    Dependent {
        governor: Option<usize>,
        function: &'static str,
        governor_category: &'static str,
    },
    /// The head line's frame Productions, from the leaf up.
    Frame(Vec<String>),
}

pub(super) fn kind_name(kind: ForkKind) -> &'static str {
    match kind {
        ForkKind::Lexical => "lexical",
        ForkKind::Dependency => "dependency",
        ForkKind::Frame => "frame",
    }
}

/// The forks of `parse`, whose stored `leaves` are those some Derivation
/// uses.
pub(super) fn rows(parse: &Parse<'_>, leaves: &[LeafRow]) -> Result<ForkRows> {
    let forest = parse.fork_forest()?;
    let words = forest.words().to_vec();
    let word = |w: &ForkWord| {
        words
            .iter()
            .position(|x| x.names(w))
            .with_context(|| format!("fork word `{}` #{} is no word", w.surface, w.occurrence))
    };
    let answer = |kind: ForkKind, at: &ForkWord, answer: &Answer| -> Result<AnswerRow> {
        Ok(match (kind, answer) {
            (ForkKind::Lexical, Answer::Lexical { reading }) => AnswerRow::Lexical(
                leaves
                    .iter()
                    .find(|leaf| leaf.span.start == at.start && leaf.reading == *reading)
                    .with_context(|| {
                        format!("no stored leaf at `{}` reads {reading:?}", at.surface)
                    })?
                    .leaf,
            ),
            (ForkKind::Dependency, Answer::Dependency(Dependency::Root)) => AnswerRow::Root,
            (
                ForkKind::Dependency,
                Answer::Dependency(Dependency::Dependent {
                    governor,
                    function,
                    governor_category,
                }),
            ) => AnswerRow::Dependent {
                governor: governor.as_ref().map(word).transpose()?,
                function: function.as_str(),
                governor_category: governor_category.as_str(),
            },
            (ForkKind::Frame, Answer::Frame(productions)) => {
                AnswerRow::Frame(productions.iter().map(ToString::to_string).collect())
            }
            (kind, answer) => anyhow::bail!("a {kind:?} question answers {answer:?}"),
        })
    };
    let forks = forest
        .forks()
        .iter()
        .map(|fork| {
            let options = fork
                .options
                .iter()
                .map(|option| {
                    anyhow::ensure!(
                        option.aliases.len() == fork.aliases.len(),
                        "an option answers {} of {} aliases",
                        option.aliases.len(),
                        fork.aliases.len()
                    );
                    let mut answers = vec![answer(fork.kind, &fork.word, &option.answer)?];
                    for ((kind, at), alias) in fork.aliases.iter().zip(&option.aliases) {
                        answers.push(answer(*kind, at, alias)?);
                    }
                    Ok(OptionRow {
                        derivations: option.derivations,
                        answers,
                    })
                })
                .collect::<Result<_>>()?;
            Ok(ForkRow {
                kind: fork.kind,
                word: word(&fork.word)?,
                aliases: fork
                    .aliases
                    .iter()
                    .map(|(kind, at)| Ok((*kind, word(at)?)))
                    .collect::<Result<_>>()?,
                context: fork.context,
                conditional: fork.conditional,
                options,
            })
        })
        .collect::<Result<_>>()?;
    Ok(ForkRows { words, forks })
}

pub(super) fn write(tx: &rusqlite::Transaction<'_>, u_id: i64, rows: &ForkRows) -> Result<()> {
    let mut word_insert = tx.prepare_cached("INSERT INTO words VALUES (?1, ?2, ?3, ?4, ?5)")?;
    for (w, word) in rows.words.iter().enumerate() {
        word_insert.execute(params![
            u_id,
            index(w),
            index(word.start),
            word.surface,
            index(word.occurrence),
        ])?;
    }
    let mut fork_insert = tx.prepare_cached("INSERT INTO forks VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?;
    let mut alias_insert =
        tx.prepare_cached("INSERT INTO fork_aliases VALUES (?1, ?2, ?3, ?4, ?5)")?;
    let mut option_insert =
        tx.prepare_cached("INSERT INTO fork_options VALUES (?1, ?2, ?3, ?4)")?;
    let mut answer_insert = tx.prepare_cached(
        "INSERT INTO fork_answers VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
    )?;
    for (f, fork) in rows.forks.iter().enumerate() {
        fork_insert.execute(params![
            u_id,
            index(f),
            kind_name(fork.kind),
            index(fork.word),
            int(fork.context)?,
            fork.conditional,
        ])?;
        for (a, (kind, word)) in fork.aliases.iter().enumerate() {
            alias_insert.execute(params![
                u_id,
                index(f),
                index(a + 1),
                kind_name(*kind),
                index(*word)
            ])?;
        }
        for (o, option) in fork.options.iter().enumerate() {
            option_insert.execute(params![u_id, index(f), index(o), int(option.derivations)?])?;
            for (q, answer) in option.answers.iter().enumerate() {
                let (leaf, dependency, governor, function, category, frames) = match answer {
                    AnswerRow::Lexical(leaf) => (Some(index(*leaf)), None, None, None, None, None),
                    AnswerRow::Root => (None, Some("root"), None, None, None, None),
                    AnswerRow::Dependent {
                        governor,
                        function,
                        governor_category,
                    } => (
                        None,
                        Some("dependent"),
                        governor.map(index),
                        Some(*function),
                        Some(*governor_category),
                        None,
                    ),
                    AnswerRow::Frame(productions) => (
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(serde_json::to_string(productions)?),
                    ),
                };
                answer_insert.execute(params![
                    u_id,
                    index(f),
                    index(o),
                    index(q),
                    leaf,
                    dependency,
                    governor,
                    function,
                    category,
                    frames,
                ])?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use rusqlite::Connection;
    use sylex_syntax::Fork;
    use sylex_syntax::ForkOption;
    use sylex_vocabulary::LexicalEnvironment;

    use super::*;

    /// A stored index or count as `usize`.
    fn at(r: &rusqlite::Row<'_>, i: usize) -> usize {
        usize::try_from(r.get::<_, i64>(i).unwrap()).unwrap()
    }

    fn kind(name: &str) -> ForkKind {
        match name {
            "lexical" => ForkKind::Lexical,
            "dependency" => ForkKind::Dependency,
            "frame" => ForkKind::Frame,
            other => panic!("no fork kind `{other}`"),
        }
    }

    /// One unit's forks, read back from the stored rows alone; a lexical
    /// answer's leaf is read in `parse`, the same parse of the unit.
    fn stored(db: &Connection, unit: i64, parse: &Parse<'_>) -> Vec<Fork> {
        let words: Vec<ForkWord> = db
            .prepare("SELECT start, surface, occurrence FROM words WHERE unit = ?1 ORDER BY word")
            .unwrap()
            .query_map([unit], |r| {
                Ok(ForkWord {
                    start: at(r, 0),
                    surface: r.get(1)?,
                    occurrence: at(r, 2),
                })
            })
            .unwrap()
            .map(Result::unwrap)
            .collect();
        let word = |w: usize| words[w].clone();
        // A fork's aliases in order; a fork without aliases has no row.
        let mut alias_rows = db
            .prepare(
                "SELECT kind, word FROM fork_aliases WHERE unit = ?1 AND fork = ?2 ORDER BY alias",
            )
            .unwrap();
        let mut aliases = |fork: usize| -> Vec<(ForkKind, ForkWord)> {
            alias_rows
                .query_map(rusqlite::params![unit, index(fork)], |r| {
                    Ok((r.get::<_, String>(0)?, at(r, 1)))
                })
                .unwrap()
                .map(|row| {
                    let (k, w) = row.unwrap();
                    (kind(&k), word(w))
                })
                .collect()
        };
        let mut answers: BTreeMap<(usize, usize), Vec<Answer>> = BTreeMap::new();
        let mut statement = db
            .prepare(
                "SELECT fork, option, leaf, dependency, governor, function, governor_category, \
                 frames FROM fork_answers WHERE unit = ?1 ORDER BY fork, option, question",
            )
            .unwrap();
        let mut query = statement.query([unit]).unwrap();
        while let Some(r) = query.next().unwrap() {
            let leaf = r
                .get::<_, Option<i64>>(2)
                .unwrap()
                .map(|l| usize::try_from(l).unwrap());
            let dependency: Option<String> = r.get(3).unwrap();
            let frames: Option<String> = r.get(7).unwrap();
            let answer = match (leaf, dependency.as_deref(), frames) {
                (Some(leaf), None, None) => Answer::Lexical {
                    reading: parse.forest.leaf_reading(leaf).clone(),
                },
                (None, Some("root"), None) => Answer::Dependency(Dependency::Root),
                (None, Some("dependent"), None) => Answer::Dependency(Dependency::Dependent {
                    governor: r
                        .get::<_, Option<i64>>(4)
                        .unwrap()
                        .map(|w| word(usize::try_from(w).unwrap())),
                    function: r.get::<_, String>(5).unwrap().as_str().into(),
                    governor_category: r.get::<_, String>(6).unwrap().as_str().into(),
                }),
                (None, None, Some(frames)) => Answer::Frame(
                    serde_json::from_str::<Vec<String>>(&frames)
                        .unwrap()
                        .iter()
                        .map(|p| p.as_str().into())
                        .collect(),
                ),
                other => panic!("an answer row of no kind: {other:?}"),
            };
            answers
                .entry((at(r, 0), at(r, 1)))
                .or_default()
                .push(answer);
        }
        let mut options: BTreeMap<usize, Vec<ForkOption>> = BTreeMap::new();
        let mut statement = db
            .prepare(
                "SELECT fork, option, derivations FROM fork_options WHERE unit = ?1 ORDER BY \
                 fork, option",
            )
            .unwrap();
        for row in statement
            .query_map([unit], |r| Ok((at(r, 0), at(r, 1), r.get::<_, i64>(2)?)))
            .unwrap()
        {
            let (fork, option, derivations) = row.unwrap();
            let mut answers = answers.remove(&(fork, option)).unwrap().into_iter();
            options.entry(fork).or_default().push(ForkOption {
                answer: answers.next().unwrap(),
                aliases: answers.collect(),
                derivations: u128::try_from(derivations).unwrap(),
            });
        }
        assert!(answers.is_empty(), "answers of no option: {answers:?}");
        db.prepare(
            "SELECT fork, kind, word, context, conditional FROM forks WHERE unit = ?1 ORDER BY \
             fork",
        )
        .unwrap()
        .query_map([unit], |r| {
            Ok((
                at(r, 0),
                r.get::<_, String>(1)?,
                at(r, 2),
                r.get::<_, i64>(3)?,
                r.get::<_, bool>(4)?,
            ))
        })
        .unwrap()
        .map(|row| {
            let (fork, k, w, context, conditional) = row.unwrap();
            Fork {
                kind: kind(&k),
                word: word(w),
                aliases: aliases(fork),
                context: u128::try_from(context).unwrap(),
                conditional,
                options: options.remove(&fork).unwrap(),
            }
        })
        .collect()
    }

    /// Real units for what their forks exercise: a dependency fork
    /// (*Crook of Condemnation*), two attachments one vertex packs
    /// (*Counterbore*), many forks of a large forest with aliases (*Remove
    /// Enchantments*, 9,216 Derivations), frame Productions the dependency
    /// answers determine (*Auriok Salvagers*), a frame fork (*Citadel
    /// Gate*), a lexical fork (*Up the Beanstalk*), two leaves in one word
    /// (*The Eternal Wanderer*) and conditional forks (*Turtles in Time*).
    /// Each unit's stored forks equal what `ForkForest::forks` computes.
    #[test]
    fn the_stored_forks_equal_the_fork_forest_s() {
        let cards = [
            "Crook of Condemnation",
            "Counterbore",
            "Remove Enchantments",
            "Auriok Salvagers",
            "Citadel Gate",
            "Up the Beanstalk",
            "The Eternal Wanderer",
            "Turtles in Time",
        ];
        let (_dir, path, corpus) = super::super::tests::database(&cards);
        let db = Connection::open(&path).unwrap();
        let env = LexicalEnvironment::load().unwrap();
        let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
        let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
        for (unit, u) in corpus.units.iter().enumerate() {
            let face = &corpus.faces[u.face];
            let parsed: String = db
                .query_row(
                    "SELECT parsed FROM units WHERE unit = ?1",
                    [index(unit)],
                    |r| r.get(0),
                )
                .unwrap();
            let analyzed = env
                .lexicon
                .analyze_source_scoped(&parsed, None, &face.scope());
            let parse = grammar.parse_analyzed(&analyzed);
            let expected = parse.fork_forest().unwrap().forks();
            for fork in &expected {
                *seen.entry(kind_name(fork.kind)).or_default() += 1;
                if fork.conditional {
                    *seen.entry("conditional").or_default() += 1;
                }
                if !fork.aliases.is_empty() {
                    *seen.entry("aliased").or_default() += 1;
                }
            }
            assert_eq!(stored(&db, index(unit), &parse), expected, "{}", u.text);
        }
        for what in ["lexical", "dependency", "frame", "conditional", "aliased"] {
            assert!(seen.contains_key(what), "no {what} fork: {seen:?}");
        }
    }

    /// Wide and narrow attachments over a coordination on real units: a
    /// relative clause over *creature or planeswalker* or on *planeswalker*
    /// (*Iroas's Blessing*), *until end of turn* over *gets … and gains …* or
    /// on *gains* (*Bull's Strength*), a trigger over a Linked Coordination
    /// (*look …, then put …*) or on its first conjunct (*Sage Owl*), and
    /// premodifiers over a serial coordination or on its first conjunct
    /// (*Sands of Time*). *Crook of Condemnation*'s one dependency fork has
    /// no coordination.
    #[test]
    fn wide_and_narrow_attachments_over_a_coordination() {
        let (_dir, path, _) = super::super::tests::database(&[
            "Iroas's Blessing",
            "Bull's Strength",
            "Sage Owl",
            "Sands of Time",
            "Crook of Condemnation",
        ]);
        let db = Connection::open(&path).unwrap();
        let rows: Vec<String> = db
            .prepare(
                "SELECT DISTINCT u.face_name, a.dependent_surface, a.function, \
                 a.governor_category, a.coordinator_surface, a.narrow_governor_surface, \
                 a.wide_derivations, a.narrow_derivations, u.derivations FROM \
                 wide_narrow_attachments a JOIN units u USING (unit) ORDER BY 1, 2",
            )
            .unwrap()
            .query_map([], |r| {
                Ok((0..9)
                    .map(|i| match r.get::<_, rusqlite::types::Value>(i).unwrap() {
                        rusqlite::types::Value::Text(s) => s,
                        rusqlite::types::Value::Integer(n) => n.to_string(),
                        other => panic!("{other:?}"),
                    })
                    .collect::<Vec<_>>()
                    .join(" | "))
            })
            .unwrap()
            .map(Result::unwrap)
            .collect();
        assert_eq!(
            rows,
            [
                "Bull's Strength | until | adjunct | VerbPhrase | and | gains | 1 | 1 | 2",
                "Iroas's Blessing | controls | modifier | Nominal | or | planeswalker | 1 | 1 | 2",
                "Sage Owl | When | adjunct | Clause | then | look | 2 | 2 | 4",
                "Sands of Time | control | modifier | Nominal | and | land | 60 | 40 | 100",
                "Sands of Time | simultaneously | adjunct | VerbPhrase | and | untaps | 50 | 50 | 100",
                "Sands of Time | tapped | modifier | Nominal | and | artifact | 60 | 40 | 100",
                "Sands of Time | untapped | modifier | Nominal | and | artifact | 60 | 40 | 100",
            ]
        );
    }
}

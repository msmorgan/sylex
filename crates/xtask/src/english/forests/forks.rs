//! A unit's forks (`sylex_syntax::ForkForest::forks`) as rows of the
//! `words`, `forks`, `fork_aliases`, `fork_options` and `fork_answers`
//! tables, and its nodes' Head Lines (`ForkForest::head_lines`) as rows of
//! `head_lines`.

use std::collections::BTreeMap;

use anyhow::Context;
use anyhow::Result;
use rusqlite::Connection;
use sylex_syntax::Answer;
use sylex_syntax::Dependency;
use sylex_syntax::ForkKind;
use sylex_syntax::ForkWord;
use sylex_syntax::HeadLineNode;
use sylex_syntax::Parse;

use super::LeafRow;
use super::batch::BatchInsert;
use super::batch::Val;
use super::index;
use super::int;

/// A unit's words and forks, with words and leaves by index.
pub(super) struct ForkRows {
    words: Vec<ForkWord>,
    head_lines: Vec<HeadLineNode>,
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
    /// The head line's frame Productions as a JSON array, from the leaf up.
    Frame(String),
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
                let ids: Vec<&str> = productions
                    .iter()
                    .map(sylex_lexicon::LexemeId::as_str)
                    .collect();
                AnswerRow::Frame(serde_json::to_string(&ids)?)
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
    Ok(ForkRows {
        words,
        head_lines: forest.head_lines(),
        forks,
    })
}

/// The `INSERT`s of a unit's fork rows, prepared once for the transaction.
pub(super) struct Inserts<'t> {
    word: BatchInsert<'t, 5, 64>,
    head_line: BatchInsert<'t, 4, 128>,
    fork: BatchInsert<'t, 6, 64>,
    alias: BatchInsert<'t, 5, 64>,
    option: BatchInsert<'t, 4, 128>,
    answer: BatchInsert<'t, 10, 64>,
}

impl<'t> Inserts<'t> {
    pub(super) fn prepare(tx: &'t Connection) -> Result<Self> {
        Ok(Self {
            word: BatchInsert::prepare(tx, "words")?,
            head_line: BatchInsert::prepare(tx, "head_lines")?,
            fork: BatchInsert::prepare(tx, "forks")?,
            alias: BatchInsert::prepare(tx, "fork_aliases")?,
            option: BatchInsert::prepare(tx, "fork_options")?,
            answer: BatchInsert::prepare(tx, "fork_answers")?,
        })
    }

    /// Flushes every table's remainder and records its row count in `rows`.
    pub(super) fn finish(&mut self, rows: &mut BTreeMap<&'static str, i64>) -> Result<()> {
        rows.extend([
            self.word.finish()?,
            self.head_line.finish()?,
            self.fork.finish()?,
            self.alias.finish()?,
            self.option.finish()?,
            self.answer.finish()?,
        ]);
        Ok(())
    }
}

pub(super) fn write(inserts: &mut Inserts<'_>, u_id: i64, rows: &ForkRows) -> Result<()> {
    for (w, word) in rows.words.iter().enumerate() {
        inserts.word.push([
            Val::Int(u_id),
            Val::Int(index(w)),
            Val::Int(index(word.start)),
            Val::Text(&word.surface),
            Val::Int(index(word.occurrence)),
        ])?;
    }
    for line in &rows.head_lines {
        inserts.head_line.push([
            Val::Int(u_id),
            Val::Int(index(line.node)),
            line.word.map_or(Val::Null, |w| Val::Int(index(w))),
            Val::Int(int(line.derivations)?),
        ])?;
    }
    for (f, fork) in rows.forks.iter().enumerate() {
        inserts.fork.push([
            Val::Int(u_id),
            Val::Int(index(f)),
            Val::Static(kind_name(fork.kind)),
            Val::Int(index(fork.word)),
            Val::Int(int(fork.context)?),
            Val::Bool(fork.conditional),
        ])?;
        for (a, (kind, word)) in fork.aliases.iter().enumerate() {
            inserts.alias.push([
                Val::Int(u_id),
                Val::Int(index(f)),
                Val::Int(index(a + 1)),
                Val::Static(kind_name(*kind)),
                Val::Int(index(*word)),
            ])?;
        }
        for (o, option) in fork.options.iter().enumerate() {
            inserts.option.push([
                Val::Int(u_id),
                Val::Int(index(f)),
                Val::Int(index(o)),
                Val::Int(int(option.derivations)?),
            ])?;
            for (q, answer) in option.answers.iter().enumerate() {
                let (leaf, dependency, governor, function, category, frames) = match answer {
                    AnswerRow::Lexical(leaf) => (
                        Val::Int(index(*leaf)),
                        Val::Null,
                        Val::Null,
                        Val::Null,
                        Val::Null,
                        Val::Null,
                    ),
                    AnswerRow::Root => (
                        Val::Null,
                        Val::Static("root"),
                        Val::Null,
                        Val::Null,
                        Val::Null,
                        Val::Null,
                    ),
                    AnswerRow::Dependent {
                        governor,
                        function,
                        governor_category,
                    } => (
                        Val::Null,
                        Val::Static("dependent"),
                        governor.map_or(Val::Null, |g| Val::Int(index(g))),
                        Val::Static(function),
                        Val::Static(governor_category),
                        Val::Null,
                    ),
                    AnswerRow::Frame(frames) => (
                        Val::Null,
                        Val::Null,
                        Val::Null,
                        Val::Null,
                        Val::Null,
                        Val::Text(frames.as_str()),
                    ),
                };
                inserts.answer.push([
                    Val::Int(u_id),
                    Val::Int(index(f)),
                    Val::Int(index(o)),
                    Val::Int(index(q)),
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
                    reading: parse.forest().leaf_reading(leaf).clone(),
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
    /// Enchantments*), frame Productions the dependency
    /// answers determine (*Auriok Salvagers*), a frame fork (*Citadel
    /// Gate*), a lexical fork (*Up the Beanstalk*), two leaves in one word
    /// (*The Eternal Wanderer*) and a node on two words' Head Lines (*Helga,
    /// Skittish Seer*). Each unit's stored forks and Head Line rows equal what
    /// `ForkForest::forks` and `ForkForest::head_lines` compute.
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
            "Helga, Skittish Seer",
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
            let analyzed = env.lexicon.analyze_source_scoped(
                &parsed,
                None,
                &face.scope(&env.lexicon, &grammar),
            );
            let parse = grammar.parse_analyzed(&analyzed);
            let forest = parse.fork_forest().unwrap();
            let expected = forest.forks();
            let lines: Vec<HeadLineNode> = db
                .prepare(
                    "SELECT node, word, derivations FROM head_lines WHERE unit = ?1 ORDER BY \
                     node, word",
                )
                .unwrap()
                .query_map([index(unit)], |r| {
                    Ok(HeadLineNode {
                        node: at(r, 0),
                        word: r
                            .get::<_, Option<i64>>(1)?
                            .map(|w| usize::try_from(w).unwrap()),
                        derivations: u128::try_from(r.get::<_, i64>(2)?).unwrap(),
                    })
                })
                .unwrap()
                .map(Result::unwrap)
                .collect();
            assert_eq!(lines, forest.head_lines(), "{}", u.text);
            for fork in &expected {
                *seen.entry(kind_name(fork.kind)).or_default() += 1;
                if !fork.aliases.is_empty() {
                    *seen.entry("aliased").or_default() += 1;
                }
            }
            assert_eq!(stored(&db, index(unit), &parse), expected, "{}", u.text);
        }
        for what in ["lexical", "dependency", "frame", "aliased"] {
            assert!(seen.contains_key(what), "no {what} fork: {seen:?}");
        }
        // Each node's Head Line rows partition its Derivations, and Helga's
        // coordination of *creature spells with mana value 4 or greater or
        // creature spells with {X}* is on either *or*'s Head Line.
        for (what, sql) in [
            (
                "a node whose Head Line rows do not sum to its Derivations",
                "SELECT count(*) FROM nodes n WHERE derivations != (SELECT sum(derivations) \
                 FROM head_lines h WHERE h.unit = n.unit AND h.node = n.node)",
            ),
            (
                "a Head Line row of no stored node",
                "SELECT count(*) FROM head_lines h WHERE NOT EXISTS (SELECT 1 FROM nodes n \
                 WHERE n.unit = h.unit AND n.node = h.node)",
            ),
        ] {
            let n: i64 = db.query_row(sql, [], |r| r.get(0)).unwrap();
            assert_eq!(n, 0, "{what}");
        }
        let two: i64 = db
            .query_row(
                "SELECT count(*) FROM (SELECT h.node FROM head_lines h JOIN units u USING (unit) \
                 JOIN words w ON w.unit = h.unit AND w.word = h.word WHERE u.face_name = 'Helga, \
                 Skittish Seer' AND w.surface = 'or' GROUP BY h.unit, h.node HAVING count(*) = \
                 2)",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(two > 0);
    }

    /// Wide and narrow attachments over a coordination on real units: a
    /// relative clause over *creature or planeswalker* or on *planeswalker*
    /// (*Iroas's Blessing*), *until end of turn* over *gets … and gains …* or
    /// on *gains* (*Bull's Strength*), over a Clause coordination or on the
    /// Verb Phrase *gain* of its last conjunct (*Lisette, Dean of the Root*),
    /// a trigger and *in any order* over a Linked Coordination (*look …, then
    /// put …*) or inside one of its conjuncts (*Sage Owl*), and premodifiers
    /// over a serial coordination or on its first conjunct, the relative
    /// *they control* over it or on its last, and *simultaneously* over
    /// *untaps … and taps …* or on *untaps* (*Sands of Time*). *Court Hussar*'s
    /// *in any order* and *on the bottom* attach over *look …, then put …* or
    /// inside its last conjunct, among them on the Complement Cluster *the rest
    /// on the bottom*, whose Head Line ends in *rest* though no `head` child
    /// leads there; each also attaches over the noun phrase coordination *one
    /// of them … and the rest* or inside its last conjunct. Not *Kessig
    /// Naturalist*'s *Until end of turn, you don't lose this mana as steps and
    /// phases end*: every Clause headed by *don't* that holds *Until* also
    /// holds *and*, which there joins only *steps and phases*; the options
    /// differ in what is coordinated. Its back face, *Lord of the Ulvenwald*,
    /// has *you control* over *Wolves and Werewolves* or on *Werewolves*.
    /// *Crook of Condemnation*'s one dependency fork has no coordination.
    #[test]
    fn wide_and_narrow_attachments_over_a_coordination() {
        let (_dir, path, _) = super::super::tests::database(&[
            "Iroas's Blessing",
            "Bull's Strength",
            "Valentin, Dean of the Vein // Lisette, Dean of the Root",
            "Sage Owl",
            "Sands of Time",
            "Court Hussar",
            "Kessig Naturalist // Lord of the Ulvenwald",
            "Crook of Condemnation",
        ]);
        let db = Connection::open(&path).unwrap();
        // Kessig Naturalist's *Until* has a dependency fork with an option
        // governed by *and*, which is no wide attachment.
        let kessig: i64 = db
            .query_row(
                "SELECT count(*) FROM forks f JOIN units u USING (unit) JOIN words w ON \
                 w.unit = f.unit AND w.word = f.word JOIN fork_answers a ON a.unit = f.unit AND \
                 a.fork = f.fork JOIN words g ON g.unit = a.unit AND g.word = a.governor WHERE \
                 u.face_name = 'Kessig Naturalist' AND f.kind = 'dependency' AND w.surface = \
                 'Until' AND g.surface = 'and'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(kessig > 0);
        let rows: Vec<String> = db
            .prepare(
                "SELECT DISTINCT u.face_name, a.dependent_surface, a.wide_function, \
                 a.wide_governor_category, a.coordinator_surface, a.narrow_function, \
                 a.narrow_governor_category, a.narrow_governor_surface, a.wide_derivations, \
                 a.narrow_derivations, u.derivations FROM wide_narrow_attachments a JOIN \
                 units u USING (unit) ORDER BY 1, 2",
            )
            .unwrap()
            .query_map([], |r| {
                Ok((0..11)
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
                "Bull's Strength | until | adjunct | VerbPhrase | and | adjunct | VerbPhrase | \
                 gains | 1 | 1 | 2",
                "Court Hussar | When | adjunct | Clause | then | adjunct | Clause | look | 25 | 25 \
                 | 50",
                "Court Hussar | in | adjunct | Clause | then | adjunct | VerbPhrase | put | 10 | 8 \
                 | 50",
                "Court Hussar | in | adjunct | Clause | then | modifier | NounPhrase | and | 10 | \
                 4 | 50",
                "Court Hussar | in | adjunct | Clause | then | adjunct | ComplementCluster | rest \
                 | 10 | 2 | 50",
                "Court Hussar | in | adjunct | Clause | then | complement | ComplementCluster | \
                 rest | 10 | 4 | 50",
                "Court Hussar | in | adjunct | Clause | then | modifier | Nominal | rest | 10 | 2 \
                 | 50",
                "Court Hussar | in | adjunct | Clause | then | modifier | Nominal | bottom | 10 | \
                 10 | 50",
                "Court Hussar | in | adjunct | Clause | then | modifier | Nominal | library | 10 | \
                 10 | 50",
                "Court Hussar | in | modifier | NounPhrase | and | adjunct | ComplementCluster | \
                 rest | 4 | 2 | 50",
                "Court Hussar | in | modifier | NounPhrase | and | complement | ComplementCluster \
                 | rest | 4 | 4 | 50",
                "Court Hussar | in | modifier | NounPhrase | and | modifier | Nominal | rest | 4 | \
                 2 | 50",
                "Court Hussar | in | modifier | NounPhrase | and | modifier | Nominal | bottom | 4 \
                 | 10 | 50",
                "Court Hussar | in | modifier | NounPhrase | and | modifier | Nominal | library | \
                 4 | 10 | 50",
                "Court Hussar | on | adjunct | Clause | then | adjunct | VerbPhrase | put | 6 | 8 \
                 | 50",
                "Court Hussar | on | adjunct | Clause | then | modifier | NounPhrase | and | 6 | \
                 10 | 50",
                "Court Hussar | on | adjunct | Clause | then | adjunct | ComplementCluster | rest \
                 | 6 | 2 | 50",
                "Court Hussar | on | adjunct | Clause | then | complement | ComplementCluster | \
                 rest | 6 | 10 | 50",
                "Court Hussar | on | adjunct | Clause | then | modifier | Nominal | rest | 6 | 14 \
                 | 50",
                "Court Hussar | on | modifier | NounPhrase | and | adjunct | ComplementCluster | \
                 rest | 10 | 2 | 50",
                "Court Hussar | on | modifier | NounPhrase | and | complement | ComplementCluster \
                 | rest | 10 | 10 | 50",
                "Court Hussar | on | modifier | NounPhrase | and | modifier | Nominal | rest | 10 \
                 | 14 | 50",
                "Iroas's Blessing | controls | modifier | Nominal | or | modifier | Nominal | \
                 planeswalker | 1 | 1 | 2",
                "Lisette, Dean of the Root | until | adjunct | Clause | and | adjunct | VerbPhrase \
                 | gain | 1 | 1 | 2",
                "Lord of the Ulvenwald | control | modifier | Nominal | and | modifier | Nominal | \
                 Werewolves | 1 | 1 | 2",
                "Sage Owl | When | adjunct | Clause | then | adjunct | Clause | look | 2 | 2 | 4",
                "Sage Owl | in | adjunct | Clause | then | adjunct | VerbPhrase | put | 2 | 2 | 4",
                "Sands of Time | control | modifier | Nominal | and | modifier | Nominal | land | \
                 16 | 16 | 32",
                "Sands of Time | simultaneously | adjunct | VerbPhrase | and | adjunct | \
                 VerbPhrase | untaps | 16 | 16 | 32",
                "Sands of Time | tapped | modifier | Nominal | and | modifier | Nominal | artifact \
                 | 16 | 16 | 32",
                "Sands of Time | untapped | modifier | Nominal | and | modifier | Nominal | \
                 artifact | 16 | 16 | 32",
            ]
        );
    }
}

//! Forks (`sylex_syntax::ForkForest`) on real Corpus Units, against the
//! unit's enumerated Readings. From each Reading the oracle reads what it
//! answers for every word (its leaves' lexical readings, each maximal
//! projection's dependency and frame Productions) by the Reading-side rules
//! (`CompiledGrammar::reading_head_line_child`, `child_names`), checks each
//! dependency against the function and governor `CompiledGrammar::
//! constituents` locates, and builds the forks by their definition: the
//! questions with two answers, frame questions that the lexical and
//! dependency answers determine dropped, questions with one partition
//! grouped. Then:
//!
//! - the forks the forest computes equal the oracle's, and the split forest's
//!   Readings are the forest's;
//! - for every option of every fork (in the default suite, each fork's first
//!   option only on a unit of more than 1,000 Derivations), picking it leaves
//!   exactly the Readings that give it and excluding it leaves the others
//!   (compared one by one up to [`COMPARED_READINGS`], by count above), and the
//!   narrowed forest's forks equal the oracle's on those Readings.
//!
//! The pool run also reports how many Derivations are left after picking an
//! option of every fork, the first in display order each time, until none is
//! left, and for each unit whose Readings the answers do not tell apart,
//! where two of them first differ.

#[path = "support/unit_texts.rs"]
mod unit_texts;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::time::Duration;
use std::time::Instant;

use dpsi::Ident;
use rayon::prelude::*;
use sylex_english::corpus::Corpus;
use sylex_english::corpus::Face;
use sylex_lexicon::LexicalReading;
use sylex_syntax::Answer;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Dependency;
use sylex_syntax::Fork;
use sylex_syntax::ForkForest;
use sylex_syntax::ForkKind;
use sylex_syntax::ForkWord;
use sylex_syntax::Parse;
use sylex_syntax::Reading;
use sylex_vocabulary::LexicalEnvironment;

/// Real units, each for what it exercises:
/// - *Crook of Condemnation*: one dependency fork ("from" modifies "card" or
///   adjoins the Verb Phrase) behind a vertex five levels up.
/// - *Draugr Necromancer* (3,312 Derivations), *Maralen, Fae Ascendant* (5,955)
///   and *Remove Enchantments* (9,216): many forks on large forests.
/// - *Counterbore*: two attachments ("with", "as") that one vertex packs as a
///   disjunction.
/// - *Ensnared by the Mara* and *Heart of Bogardan*: a Clause coordination that
///   begins with a headless conjunct, and nested Noun Phrases at one start,
///   where a (Category, start) key gives one Derivation two answers.
/// - *Auriok Salvagers*: frame Productions that its dependency answers
///   determine.
/// - *Eye of Yawgmoth* and *The Eternal Wanderer*: two leaves in one word
///   ("creature's", "player's").
/// - *Decimate*: more than one live root.
/// - *Up the Beanstalk*: a node that packs two lexical readings of one word
///   ("cast", present and preterite) as its head.
/// - *Citadel Gate*: a frame fork the lexical and dependency forks do not
///   determine ("white" a noun or a mention of the word).
/// - *Turtles in Time*: conditional forks ("in" and "Time" have answers only
///   where "Turtles" is not the card's name).
/// - [`LINKED`]: Linked Coordinations and the one explicit coordinator outside
///   a coordination schema that the pool prints.
const SAMPLE: &[&str] = &[
    "Crook of Condemnation",
    "Draugr Necromancer",
    "Maralen, Fae Ascendant",
    "Remove Enchantments",
    "Counterbore",
    "Ensnared by the Mara",
    "Heart of Bogardan",
    "Auriok Salvagers",
    "Eye of Yawgmoth",
    "The Eternal Wanderer",
    "Decimate",
    "Up the Beanstalk",
    "Giant's Amulet",
    "Topography Tracker",
    "Burning Inquiry",
    "Volatile Stormdrake",
    "Anim Pakal, Thousandth Moon",
    "Turtle Tracks",
    "Moritte of the Frost",
    "Citadel Gate",
    "Turtles in Time",
];

/// Units whose Readings differ in what a dependent of a coordination with
/// no coordinator leaf, or of one of its conjuncts, depends on; the forks
/// separate every one of their Readings, since a Linked Coordination's head
/// line ends in its connective and a coordinator written outside a schema is
/// its Production's head-line child:
/// - *Giant's Amulet*: "If you do, create …, then attach …", the condition over
///   both conjuncts or the first;
/// - *Topography Tracker*: "If …, instead it explores, then it explores again",
///   *if* and *instead* each over both or the first;
/// - *Burning Inquiry*: "draws three cards, then discards three cards at
///   random", *at random* over both or the last;
/// - *Volatile Stormdrake*: "you get {E}{E}{E}{E}, then sacrifice that creature
///   unless you pay …", a fronted *if* and a trailing *unless*;
/// - *Anim Pakal, Thousandth Moon*: a fronted *whenever* and a trailing
///   *where*;
/// - *Turtle Tracks*: a floated *each* over the coordination or its first
///   conjunct;
/// - *Moritte of the Frost*: "… and, if it's a creature, it enters with …", a
///   `DelimitedFrontConjunct`, whose `coordinator` the grammar writes.
const LINKED: &[&str] = &[
    "Giant's Amulet",
    "Topography Tracker",
    "Burning Inquiry",
    "Volatile Stormdrake",
    "Anim Pakal, Thousandth Moon",
    "Turtle Tracks",
    "Moritte of the Frost",
];

/// What one Reading answers for each word, by the word's start and the kind
/// of question.
type Answers = BTreeMap<(usize, ForkKind), Answer>;

/// An interned answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Id(u32);

/// A question: its word's start and its kind.
type Question = (usize, ForkKind);

/// What one Reading answers, interned.
type Interned = BTreeMap<Question, Id>;

/// A fork as compared: kind and start, aliases, context, whether
/// conditional, and each option's answer, aliases' answers and count.
type Shape = (
    ForkKind,
    usize,
    Vec<(ForkKind, usize)>,
    u128,
    bool,
    Vec<(Answer, Vec<Answer>, u128)>,
);

fn shape(fork: &Fork) -> Shape {
    (
        fork.kind,
        fork.word.start,
        fork.aliases.iter().map(|(k, w)| (*k, w.start)).collect(),
        fork.context,
        fork.conditional,
        fork.options
            .iter()
            .map(|o| (o.answer.clone(), o.aliases.clone(), o.derivations))
            .collect(),
    )
}

/// Reads a Reading's answers, checking each dependency against the
/// Constituents `constituents` locates.
struct Oracle<'a> {
    grammar: &'a CompiledGrammar,
    words: BTreeMap<usize, ForkWord>,
    /// The forest's leaves: start, end and lexical reading.
    leaves: Vec<(usize, usize, LexicalReading)>,
}

/// One Reading's walk: the next leaf's earliest start, the preorder index
/// of the next node, and each node's located dependency to check.
struct Walk {
    at: usize,
    node: usize,
    answers: Answers,
    /// (preorder index of a maximal projection, its function, its mother's
    /// preorder index).
    projections: Vec<(usize, Ident, usize)>,
}

impl Walk {
    fn answer(&mut self, start: usize, kind: ForkKind, answer: Answer) -> Result<(), String> {
        match self.answers.insert((start, kind), answer) {
            None => Ok(()),
            Some(old) => Err(format!("two {kind:?} answers at char {start}, one {old:?}")),
        }
    }
}

impl Oracle<'_> {
    fn answers(
        &self,
        reading: &Reading,
        lexicon: &sylex_lexicon::Lexicon,
    ) -> Result<Answers, String> {
        let mut walk = Walk {
            at: 0,
            node: 0,
            answers: Answers::new(),
            projections: Vec::new(),
        };
        let (word, frames) = self.walk(reading, &mut walk)?;
        if let Some(word) = word {
            walk.answer(
                word,
                ForkKind::Dependency,
                Answer::Dependency(Dependency::Root),
            )?;
            walk.answer(word, ForkKind::Frame, Answer::Frame(frames))?;
        }
        let (_, constituents) = self
            .grammar
            .constituents(reading, lexicon)
            .map_err(|e| e.to_string())?;
        for (index, function, mother) in walk.projections {
            let located = &constituents[index];
            if located.function != Some(function) || located.parent != Some(mother) {
                return Err(format!(
                    "Constituent {index} bears {function} in {mother}, located as {:?} in {:?}",
                    located.function, located.parent
                ));
            }
        }
        Ok(walk.answers)
    }

    /// The head-line word and frame Productions of a Reading.
    fn walk(
        &self,
        reading: &Reading,
        walk: &mut Walk,
    ) -> Result<(Option<usize>, Vec<Ident>), String> {
        let node = match reading {
            Reading::Leaf(lexical) => {
                let (start, end) = self
                    .leaves
                    .iter()
                    .filter(|(s, _, r)| *s >= walk.at && r == lexical)
                    .map(|(s, e, _)| (*s, *e))
                    .min()
                    .ok_or_else(|| format!("no leaf {lexical:?} after char {}", walk.at))?;
                walk.at = end;
                walk.answer(
                    start,
                    ForkKind::Lexical,
                    Answer::Lexical {
                        reading: lexical.clone(),
                    },
                )?;
                return Ok((Some(start), Vec::new()));
            }
            Reading::Node(node) => node,
        };
        let index = walk.node;
        walk.node += 1;
        let line = self.grammar.reading_head_line_child(node);
        let names = self.grammar.child_names(node);
        let mut children = Vec::new();
        for child in &node.children {
            let first = walk.node;
            children.push((first, self.walk(child, walk)?));
        }
        let (word, mut frames) = line.map_or((None, Vec::new()), |k| children[k].1.clone());
        let production = self
            .grammar
            .production_index(node.production)
            .ok_or("an unknown Production")?;
        if self.grammar.is_frame_production(production) {
            frames.push(node.production);
        }
        for (k, (first, (child_word, child_frames))) in children.into_iter().enumerate() {
            let Some(child_word) = child_word else { continue };
            if Some(k) == line {
                continue;
            }
            if matches!(node.children[k], Reading::Node(_)) {
                walk.projections.push((first, names[k], index));
            }
            let dependency = Dependency::Dependent {
                governor: word.map(|w| self.words[&w].clone()),
                function: names[k],
                governor_category: node.category,
            };
            walk.answer(
                child_word,
                ForkKind::Dependency,
                Answer::Dependency(dependency),
            )?;
            walk.answer(child_word, ForkKind::Frame, Answer::Frame(child_frames))?;
        }
        Ok((word, frames))
    }
}

/// The forks of a set of Readings (by index into `answers`), by
/// definition.
fn oracle_forks(answers: &[Interned], within: &BTreeSet<usize>, show: &[Answer]) -> Vec<Shape> {
    let mut questions: BTreeMap<(usize, ForkKind), BTreeMap<Id, BTreeSet<usize>>> = BTreeMap::new();
    for &d in within {
        for (key, answer) in &answers[d] {
            questions
                .entry(*key)
                .or_default()
                .entry(*answer)
                .or_default()
                .insert(d);
        }
    }
    questions.retain(|_, options| options.len() >= 2);
    // The lexical and dependency answers of each Reading.
    let meet = |d: usize| -> Vec<(&Question, &Id)> {
        answers[d]
            .iter()
            .filter(|((_, kind), _)| *kind != ForkKind::Frame)
            .collect()
    };
    questions.retain(|(_, kind), options| {
        if *kind != ForkKind::Frame {
            return true;
        }
        let mut cells: BTreeMap<Vec<(&Question, &Id)>, &Id> = BTreeMap::new();
        for (answer, readings) in options.iter() {
            for &d in readings {
                if cells
                    .insert(meet(d), answer)
                    .is_some_and(|other| other != answer)
                {
                    return true;
                }
            }
        }
        false
    });
    let total = within.len() as u128;
    let partition = |options: &BTreeMap<Id, BTreeSet<usize>>| -> BTreeSet<BTreeSet<usize>> {
        options.values().cloned().collect()
    };
    let mut groups: Vec<(Question, Vec<Question>)> = Vec::new();
    for key in questions.keys() {
        let mine = partition(&questions[key]);
        match groups
            .iter_mut()
            .find(|(primary, _)| partition(&questions[primary]) == mine)
        {
            Some((_, aliases)) => aliases.push(*key),
            None => groups.push((*key, Vec::new())),
        }
    }
    let mut shapes: Vec<Shape> = groups
        .into_iter()
        .map(|(primary, aliases)| {
            let options = &questions[&primary];
            let context: u128 = options.values().map(|r| r.len() as u128).sum();
            let mut rows: Vec<(Answer, Vec<Answer>, u128)> = options
                .iter()
                .map(|(answer, readings)| {
                    let alias_answers = aliases
                        .iter()
                        .map(|alias| {
                            questions[alias]
                                .iter()
                                .find(|(_, r)| *r == readings)
                                .expect("an alias has the same partition")
                                .0
                        })
                        .map(|a| show[a.0 as usize].clone())
                        .collect();
                    (
                        show[answer.0 as usize].clone(),
                        alias_answers,
                        readings.len() as u128,
                    )
                })
                .collect();
            rows.sort();
            (
                primary.1,
                primary.0,
                aliases
                    .iter()
                    .map(|(start, kind)| (*kind, *start))
                    .collect(),
                context,
                context != total,
                rows,
            )
        })
        .collect();
    shapes.sort_by_key(|shape| (shape.1, shape.0));
    shapes
}

/// What one unit's check measured.
#[derive(Default, Clone)]
struct Measure {
    derivations: u128,
    roots: usize,
    forks: BTreeMap<ForkKind, usize>,
    conditional: usize,
    fork_count: usize,
    /// Readings left after picking an option of every fork.
    left: u128,
    /// Distinct lexical-and-dependency, and all, answer sets.
    lexdep_classes: usize,
    all_classes: usize,
    /// For a unit the answers do not separate: where two of its Readings
    /// with the same answers first differ.
    unseparated: Option<String>,
    nodes: (usize, usize),
    items: (usize, usize),
    forks_time: Duration,
}

fn check(
    grammar: &CompiledGrammar,
    env: &LexicalEnvironment,
    face: &Face,
    text: &str,
    every_option_up_to: u128,
) -> Result<Measure, String> {
    let name = |why: String| format!("{} | {text}: {why}", face.name);
    let analyzed = env.lexicon.analyze_source_scoped(text, None, &face.scope());
    let parse: Parse<'_> = grammar.parse_analyzed(&analyzed);
    let started = Instant::now();
    let forest = parse.fork_forest().map_err(|e| name(e.to_string()))?;
    let forks = forest.forks();
    let forks_time = started.elapsed();
    let total = forest.total();
    let readings: Vec<Reading> = forest
        .readings(u128::MAX)
        .expect("every unit of the pool can be enumerated");
    if readings.len() as u128 != total {
        return Err(name(format!(
            "{} Readings, {total} Derivations",
            readings.len()
        )));
    }
    let expected_readings: BTreeSet<Reading> = parse.readings().collect();
    let found_readings: BTreeSet<Reading> = readings.iter().cloned().collect();
    if found_readings != expected_readings {
        return Err(name(
            "the split forest's Readings are not the forest's".to_owned(),
        ));
    }
    let leaves = (0..parse.forest.metrics().leaves)
        .map(|l| {
            let span = parse.forest.leaf_span(l);
            (span.start, span.end, parse.forest.leaf_reading(l).clone())
        })
        .collect();
    let oracle = Oracle {
        grammar,
        words: forest
            .words()
            .iter()
            .map(|w| (w.start, w.clone()))
            .collect(),
        leaves,
    };
    let answers: Vec<Answers> = readings
        .iter()
        .map(|r| oracle.answers(r, &env.lexicon))
        .collect::<Result<_, _>>()
        .map_err(name)?;
    let mut table: BTreeMap<Answer, u32> = BTreeMap::new();
    for a in &answers {
        for answer in a.values() {
            let next = u32::try_from(table.len()).unwrap();
            table.entry(answer.clone()).or_insert(next);
        }
    }
    let mut show = vec![None; table.len()];
    for (answer, &id) in &table {
        show[id as usize] = Some(answer.clone());
    }
    let show: Vec<Answer> = show.into_iter().map(Option::unwrap).collect();
    let interned: Vec<Interned> = answers
        .iter()
        .map(|a| a.iter().map(|(k, v)| (*k, Id(table[v]))).collect())
        .collect();
    let every: BTreeSet<usize> = (0..readings.len()).collect();
    let got: Vec<Shape> = forks.iter().map(shape).collect();
    let want = oracle_forks(&interned, &every, &show);
    if got != want {
        return Err(name(format!("forks\n  forest {got:?}\n  oracle {want:?}")));
    }
    for fork in &forks {
        let mut sum = 0;
        for option in &fork.options {
            sum += option.derivations;
        }
        if sum != fork.context {
            return Err(name(format!(
                "{:?} {} options sum to {sum}",
                fork.kind, fork.word.surface
            )));
        }
    }
    {
        let options = if total <= every_option_up_to { usize::MAX } else { 1 };
        for fork in &forks {
            for option in fork.options.iter().take(options) {
                let realize: BTreeSet<usize> = every
                    .iter()
                    .copied()
                    .filter(|&d| {
                        answers[d].get(&(fork.word.start, fork.kind)) == Some(&option.answer)
                    })
                    .collect();
                let rest: BTreeSet<usize> = every.difference(&realize).copied().collect();
                let picked = forest
                    .pick(fork.kind, &fork.word, &option.answer)
                    .map_err(|e| name(e.to_string()))?;
                compare(&picked, &readings, &interned, &show, &realize).map_err(|e| {
                    name(format!("pick {:?} {}: {e}", fork.kind, fork.word.surface))
                })?;
                let excluded = forest
                    .exclude(fork.kind, &fork.word, &option.answer)
                    .map_err(|e| name(e.to_string()))?;
                compare(&excluded, &readings, &interned, &show, &rest).map_err(|e| {
                    name(format!(
                        "exclude {:?} {}: {e}",
                        fork.kind, fork.word.surface
                    ))
                })?;
            }
        }
    }
    let mut narrowed = forest.clone();
    while let Some(fork) = narrowed.forks().into_iter().next() {
        narrowed = narrowed
            .pick(fork.kind, &fork.word, &fork.options[0].answer)
            .map_err(|e| name(e.to_string()))?;
    }
    let classes = |keep: &dyn Fn(ForkKind) -> bool| {
        answers
            .iter()
            .map(|a| a.iter().filter(|((_, k), _)| keep(*k)).collect::<Vec<_>>())
            .collect::<BTreeSet<_>>()
            .len()
    };
    let counts = parse.counts().map_err(|e| name(e.to_string()))?;
    let live = (
        (0..parse.forest.node_count())
            .filter(|&n| counts.node(n) > 0)
            .count(),
        (0..parse.forest.item_count())
            .filter(|&i| counts.item(i) > 0)
            .count(),
    );
    let mut measure = Measure {
        derivations: total,
        roots: parse.forest.roots().len(),
        conditional: forks.iter().filter(|f| f.conditional).count(),
        fork_count: forks.len(),
        left: narrowed.total(),
        lexdep_classes: classes(&|k| k != ForkKind::Frame),
        all_classes: classes(&|_| true),
        unseparated: unseparated(&readings, &interned),
        nodes: (live.0, forest.node_count()),
        items: (live.1, forest.item_count()),
        forks_time,
        ..Measure::default()
    };
    for fork in &forks {
        *measure.forks.entry(fork.kind).or_default() += 1;
    }
    Ok(measure)
}

/// A narrowed forest's Readings are compared one by one up to this many;
/// a larger one is compared by its count and its forks, since materializing
/// and comparing thousands of Readings per option costs seconds.
const COMPARED_READINGS: u128 = 500;

/// Where the first two Readings with the same answers first differ: the
/// Productions of the first nodes, in preorder, whose Productions or
/// children differ.
fn unseparated(readings: &[Reading], answers: &[Interned]) -> Option<String> {
    let mut seen: BTreeMap<&Interned, usize> = BTreeMap::new();
    for (d, a) in answers.iter().enumerate() {
        if let Some(&other) = seen.get(a) {
            return Some(first_difference(&readings[other], &readings[d]));
        }
        seen.insert(a, d);
    }
    None
}

fn first_difference(a: &Reading, b: &Reading) -> String {
    match (a, b) {
        (Reading::Node(x), Reading::Node(y))
            if x.production == y.production && x.children.len() == y.children.len() =>
        {
            x.children
                .iter()
                .zip(&y.children)
                .find(|(p, q)| p != q)
                .map_or_else(
                    || format!("{} features or gap", x.production),
                    |(p, q)| first_difference(p, q),
                )
        }
        (Reading::Node(x), Reading::Node(y)) => format!("{} vs {}", x.production, y.production),
        _ => format!("{a:?} vs {b:?}"),
    }
}

/// Checks a narrowed forest against the Readings it should keep: their
/// number, the Readings themselves when few, and the forks the oracle finds
/// on them.
fn compare(
    narrowed: &ForkForest<'_>,
    readings: &[Reading],
    answers: &[Interned],
    show: &[Answer],
    keep: &BTreeSet<usize>,
) -> Result<(), String> {
    if narrowed.total() != keep.len() as u128 {
        return Err(format!(
            "{} Derivations, {} wanted",
            narrowed.total(),
            keep.len()
        ));
    }
    if let Some(found) = narrowed.readings(COMPARED_READINGS) {
        let found: BTreeSet<Reading> = found.into_iter().collect();
        let wanted: BTreeSet<Reading> = keep.iter().map(|&d| readings[d].clone()).collect();
        if found != wanted {
            return Err(format!("{} Readings, {} wanted", found.len(), wanted.len()));
        }
    }
    let got: Vec<Shape> = narrowed.forks().iter().map(shape).collect();
    let want = oracle_forks(answers, keep, show);
    if got != want {
        return Err(format!("forks\n  forest {got:?}\n  oracle {want:?}"));
    }
    Ok(())
}

/// Checks every text of the faces `keep` admits, in parallel, narrowing by
/// every option of every fork on units of at most `every_option_up_to`
/// Derivations and by each fork's first option on larger ones.
fn check_corpus(
    keep: impl Fn(&Face) -> bool + Sync,
    every_option_up_to: u128,
) -> (BTreeSet<String>, Vec<(String, String, Measure)>) {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let corpus = Corpus::load(keep).unwrap();
    let texts = unit_texts::texts(&corpus, &env);
    let pool = rayon::ThreadPoolBuilder::new()
        .stack_size(256 << 20)
        .build()
        .unwrap();
    let results: Vec<Result<(String, String, Measure), String>> = pool.install(|| {
        texts
            .par_iter()
            .map(|(face, text)| {
                check(&grammar, &env, face, text, every_option_up_to)
                    .map(|m| (face.name.clone(), text.clone(), m))
            })
            .collect()
    });
    let failures: Vec<&str> = results
        .iter()
        .filter_map(|r| r.as_ref().err().map(String::as_str))
        .collect();
    assert!(
        failures.is_empty(),
        "{} failures:\n{}",
        failures.len(),
        failures.join("\n")
    );
    let faces = corpus.faces.iter().map(|face| face.name.clone()).collect();
    (faces, results.into_iter().map(Result::unwrap).collect())
}

#[test]
fn forks_agree_with_enumerated_readings_on_real_units() {
    let (faces, measures) = check_corpus(|face| SAMPLE.contains(&face.name.as_str()), 1000);
    assert_eq!(faces.len(), SAMPLE.len(), "{faces:?}");
    assert!(
        measures
            .iter()
            .any(|(face, _, m)| face == "Decimate" && m.roots > 1),
        "Decimate has more than one live root"
    );
    assert!(
        measures
            .iter()
            .any(|(f, _, m)| f == "Citadel Gate" && m.forks.contains_key(&ForkKind::Frame)),
        "Citadel Gate has a Frame fork"
    );
    assert!(
        measures
            .iter()
            .any(|(f, _, m)| f == "Turtles in Time" && m.conditional > 0),
        "Turtles in Time has a conditional fork"
    );
    for (face, text, m) in &measures {
        if LINKED.contains(&face.as_str()) {
            assert_eq!(
                m.all_classes as u128, m.derivations,
                "{face} | {text}: the answers do not separate its Readings: {:?}",
                m.unseparated
            );
        }
    }
}

#[test]
#[ignore = "parses every Corpus Unit and checks its forks and every narrowing against its \
            enumerated Readings; run with `cargo test --release -p sylex_english --test \
            forest_forks -- --ignored --nocapture`"]
fn forks_agree_with_enumerated_readings_on_the_pool() {
    let (_, measures) = check_corpus(|_| true, u128::MAX);
    report(&measures);
}

/// The pool's numbers, to stderr.
fn report(measures: &[(String, String, Measure)]) {
    let ambiguous: Vec<&(String, String, Measure)> = measures
        .iter()
        .filter(|(_, _, m)| m.derivations > 1)
        .collect();
    let mut by_kind: BTreeMap<ForkKind, usize> = BTreeMap::new();
    let mut per_unit: BTreeMap<usize, usize> = BTreeMap::new();
    let mut left: BTreeMap<u128, usize> = BTreeMap::new();
    let (mut conditional, mut lexdep, mut frames, mut neither) = (0, 0, 0, 0);
    let mut time = Duration::ZERO;
    let (mut nodes, mut items) = ((0, 0), (0, 0));
    for (_, _, m) in measures {
        time += m.forks_time;
        nodes = (nodes.0 + m.nodes.0, nodes.1 + m.nodes.1);
        items = (items.0 + m.items.0, items.1 + m.items.1);
    }
    for (_, _, m) in &ambiguous {
        for (kind, n) in &m.forks {
            *by_kind.entry(*kind).or_default() += n;
        }
        conditional += m.conditional;
        *per_unit.entry(m.fork_count).or_default() += 1;
        *left.entry(m.left).or_default() += 1;
        let n = usize::try_from(m.derivations).unwrap();
        if m.lexdep_classes == n {
            lexdep += 1;
        } else if m.all_classes == n {
            frames += 1;
        } else {
            neither += 1;
        }
    }
    eprintln!("texts {}; ambiguous {}", measures.len(), ambiguous.len());
    eprintln!("forks by kind {by_kind:?}; conditional {conditional}");
    eprintln!("forks per ambiguous text {per_unit:?}");
    eprintln!("Derivations left after picking every fork {left:?}");
    eprintln!(
        "separated by lexical and dependency {lexdep}; needing frame {frames}; by none {neither}"
    );
    eprintln!("nodes {nodes:?}, items {items:?} (live in the forest, split)");
    eprintln!("fork forest and forks, summed over threads: {time:?}");
    for (face, text, m) in &ambiguous {
        let n = usize::try_from(m.derivations).unwrap();
        if m.all_classes != n || m.left != 1 {
            eprintln!(
                "UNSEPARATED {face} | {text} | derivations {} classes {} left {} | {}",
                m.derivations,
                m.all_classes,
                m.left,
                m.unseparated.as_deref().unwrap_or("-")
            );
        }
    }
}

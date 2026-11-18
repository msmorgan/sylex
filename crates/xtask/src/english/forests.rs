//! `cargo xtask english forests`: every Corpus Unit's Packed Forest, with
//! its exact Derivation counts, in one SQLite database.
//!
//! The stored tables keep the forest as the chart packs and binarizes it
//! (`docs/design/english.md` §4): a node's alternative is a (Production,
//! completed item) pair, and an item's families are (previous item, child)
//! pairs. Only the part some Derivation uses is stored, with the unit's
//! forks (`ForkForest::forks`, written by `forks.rs`). The schema, with
//! each table's, index's, and view's meaning, is [`SCHEMA`], [`INDEXES`], and
//! [`VIEWS`].

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::mpsc;
use std::time::Instant;

use anyhow::Context;
use anyhow::Result;
use clap::Args;
use rusqlite::Connection;
use rusqlite::params;
use sylex_english::corpus::Corpus;
use sylex_english::corpus::UnitKind;
use sylex_lexicon::LexemeId;
use sylex_lexicon::LexicalReading;
use sylex_syntax::ChildRef;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Coordinator;
use sylex_syntax::Embedding;
use sylex_syntax::Gap;
use sylex_syntax::HeadWord;
use sylex_vocabulary::LexicalEnvironment;

mod batch;
mod context;
mod forks;
mod pipeline;
mod schema;

use batch::BatchInsert;
use batch::Val;
use schema::INDEXES;
use schema::SCHEMA;
use schema::VIEWS;

#[derive(Debug, Args)]
#[command(
    after_long_help = "The tables and views are described in the database's own schema \
    (`sqlite3 FILE .schema`); example queries with their output on the pool are in \
    crates/english/README.md, \"The forest database\"."
)]
pub(super) struct ForestsArgs {
    /// Where to write the database; an existing file is replaced.
    #[arg(long, default_value = "data/derived/english-forests.sqlite")]
    output: PathBuf,
    /// Only faces of cards with this name, a face's or its whole card's
    /// (repeatable).
    #[arg(long = "card")]
    cards: Vec<String>,
}

pub(super) fn run(args: &ForestsArgs, output: &mut dyn Write) -> Result<()> {
    let started = Instant::now();
    let env = LexicalEnvironment::load().context("loading the lexical environment")?;
    let grammar = sylex_english::bind(&env.lexicon, &env.frames)?;
    let keep = |face: &sylex_english::corpus::Face| {
        args.cards.is_empty()
            || args
                .cards
                .iter()
                .any(|c| *c == face.card || *c == face.name)
    };
    let corpus = Corpus::load(keep).context("loading the census corpus")?;
    anyhow::ensure!(
        !corpus.units.is_empty(),
        "no unit of the census corpus is on a card so named"
    );
    let summary = write(&args.output, &corpus, &env, &grammar)?;
    writeln!(output, "{summary}")?;
    writeln!(
        output,
        "wrote {} in {:.1}s",
        args.output.display(),
        started.elapsed().as_secs_f64()
    )?;
    Ok(())
}

/// The rows of one unit, as the forest has them.
struct UnitRows {
    derivations: u128,
    parsed: String,
    embeddings: Vec<Embedding>,
    nodes: Vec<NodeRow>,
    items: Vec<ItemRow>,
    leaves: Vec<LeafRow>,
    forks: forks::ForkRows,
}

struct NodeRow {
    node: usize,
    category: usize,
    span: std::ops::Range<usize>,
    surface: String,
    gap: Option<String>,
    features: String,
    coordinator: Option<Coordinator>,
    root: bool,
    settled: bool,
    derivations: u128,
    alternatives: Vec<AlternativeRow>,
}

struct AlternativeRow {
    production: usize,
    item: usize,
    derivations: u128,
    /// (head leaf, its word, the alternative's own Derivations with it, the
    /// unit's).
    heads: Vec<(Option<usize>, Option<HeadWord>, u128, u128)>,
    /// (index into `UnitRows::embeddings`, outside ways, the unit's
    /// Derivations).
    embeddings: Vec<(usize, u128, u128)>,
    conjuncts: Vec<(usize, u128)>,
}

struct ItemRow {
    item: usize,
    production: usize,
    dot: usize,
    span: std::ops::Range<usize>,
    derivations: u128,
    families: Vec<(Option<usize>, ChildRef, u128)>,
}

struct LeafRow {
    leaf: usize,
    span: std::ops::Range<usize>,
    surface: String,
    reading: LexicalReading,
}

/// Counts of what one write stored.
#[derive(Debug, Default)]
pub(crate) struct Summary {
    units: usize,
    rows: BTreeMap<&'static str, i64>,
    largest: Option<(u64, String)>,
    most_derivations: Option<(u128, String)>,
}

impl std::fmt::Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "units: {}", self.units)?;
        for (table, rows) in &self.rows {
            writeln!(f, "{table}: {rows} rows")?;
        }
        if let Some((rows, unit)) = &self.largest {
            writeln!(f, "largest unit: {rows} forest rows, {unit}")?;
        }
        if let Some((n, unit)) = &self.most_derivations {
            write!(f, "most Derivations: {n}, {unit}")?;
        }
        Ok(())
    }
}

/// How many units past the next one to write may be parsed ahead of it,
/// which bounds the parsed units held in memory at once while absorbing
/// head-of-line stalls when a complex unit takes hundreds of milliseconds to
/// parse and other worker threads continue finishing short units ahead of it.
const WINDOW: usize = 1024;

/// How many written units may wait to be freed off the writer thread.
const FREEING: usize = 256;

/// The connection settings of the writer. The `.sqlite.partial` scratch file
/// is written in one transaction by a single connection and renamed into
/// place only after every check passes:
/// - 32 KiB pages reduce B-tree splits across multi-million-row append tables;
/// - journaling, fsync, and shared locks are unneeded on an unshared scratch
///   file;
/// - `temp_store = MEMORY`, `threads = 8` (`SQLITE_MAX_WORKER_THREADS`), and a
///   1 GiB page cache (`cache_size = -1048576`, negative = KiB) keep dirty
///   B-tree pages and post-load `CREATE INDEX` sort runs in RAM instead of
///   spilling under SQLite's 2 MiB default cache.
const PRAGMAS: &str = "PRAGMA page_size = 32768;
     PRAGMA journal_mode = OFF;
     PRAGMA synchronous = OFF;
     PRAGMA locking_mode = EXCLUSIVE;
     PRAGMA temp_store = MEMORY;
     PRAGMA threads = 8;
     PRAGMA cache_size = -1048576;";

/// Writes the database for `corpus` to `path`, replacing it.
pub(crate) fn write(
    path: &Path,
    corpus: &Corpus,
    env: &LexicalEnvironment,
    grammar: &CompiledGrammar,
) -> Result<Summary> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let partial = path.with_extension("sqlite.partial");
    if partial.exists() {
        std::fs::remove_file(&partial)?;
    }
    let mut db = Connection::open(&partial)?;
    db.execute_batch(PRAGMAS)?;
    db.execute_batch(SCHEMA)?;
    db.execute_batch(VIEWS)?;
    let cleaves = sylex_english::census::cleave_texts(corpus, &env.lexicon);
    let reader = context::Reader::new(grammar, &env.lexicon);
    let faces = FaceParses::new(corpus, &cleaves, env, grammar);
    // `lexemes` and `embeddings` are written after the unit rows that reference
    // them so each table grows in key order in one pass;
    // `pragma_foreign_key_check` verifies every foreign key once the
    // transaction commits.
    db.execute_batch("PRAGMA foreign_keys = OFF;")?;
    // Units are written in order, so each `(unit, …)` table grows at its
    // end and Embeddings are numbered as they are first met in unit order.
    let summary = pipeline::run(
        corpus.units.len(),
        WINDOW,
        |unit| describe(corpus, unit),
        |unit| {
            let parsed =
                sylex_english::census::line_text(&corpus.units[unit], cleaves[unit].as_ref());
            unit_rows(parsed, &faces.take(unit), grammar, &reader)
        },
        |units| {
            let tx = db.transaction()?;
            let mut state = WriterState {
                lexemes: HashSet::new(),
                embeddings: HashMap::new(),
                unit_embedding_ids: Vec::new(),
                summary: Summary::default(),
            };
            write_grammar(&tx, grammar, &mut state.summary)?;
            let mut inserts = Inserts::prepare(&tx)?;
            // A unit's rows are freed on a thread of their own once written,
            // which keeps the writer, the run's bottleneck, from freeing them.
            // The channel is bounded, so at most `FREEING` written units (and
            // the one being freed) wait to be freed; the freeing thread waits
            // only for units, so a full channel holds the writer only until
            // it frees one.
            std::thread::scope(|scope| -> Result<()> {
                let (to_free, freeing) = mpsc::sync_channel::<UnitRows>(FREEING);
                let freer = scope.spawn(move || freeing.into_iter().for_each(drop));
                let written = (|| -> Result<()> {
                    while let Some((unit, rows)) = units.next()? {
                        write_unit(&mut inserts, &mut state, corpus, unit, &rows).map_err(
                            |error| {
                                let unit = row_unit(&error).unwrap_or(unit);
                                error.context(describe(corpus, unit))
                            },
                        )?;
                        to_free
                            .send(rows)
                            .map_err(|_| anyhow::anyhow!("the thread freeing rows stopped"))?;
                    }
                    Ok(())
                })();
                drop(to_free);
                let joined = freer.join().map_err(|panic| {
                    anyhow::anyhow!(
                        "the thread freeing rows panicked: {}",
                        pipeline::message(panic.as_ref())
                    )
                });
                written?;
                joined?;
                inserts
                    .finish(&mut state.summary.rows)
                    .map_err(|error| match row_unit(&error) {
                        Some(unit) => error.context(describe(corpus, unit)),
                        None => error,
                    })
            })?;
            drop(inserts);
            write_embeddings(
                &tx,
                &state.embeddings,
                &reader,
                &mut state.lexemes,
                &mut state.summary,
            )?;
            write_lexemes(&tx, &state.lexemes, env, &mut state.summary)?;
            tx.execute_batch(INDEXES)?;
            tx.commit()?;
            Ok(state.summary)
        },
    )?;
    let dangling: i64 = db.query_row("SELECT count(*) FROM pragma_foreign_key_check", [], |r| {
        r.get(0)
    })?;
    anyhow::ensure!(dangling == 0, "{dangling} rows reference no row");
    drop(db);
    std::fs::rename(&partial, path)?;
    Ok(summary)
}

/// The unit of the row `error` says an `INSERT` failed on. Rows are buffered
/// (`BatchInsert`), so it may be an earlier unit than the one being written
/// when the failure surfaced. Every table written unit by unit has the unit
/// as its first column.
fn row_unit(error: &anyhow::Error) -> Option<usize> {
    let unit = error.downcast_ref::<batch::RowError>()?.first()?;
    Some(usize::try_from(unit).expect("a unit column holds a unit index"))
}

fn describe(corpus: &Corpus, unit: usize) -> String {
    let u = &corpus.units[unit];
    let face = &corpus.faces[u.face];
    let place = match u.kind {
        UnitKind::TypeLine => "type line".to_owned(),
        UnitKind::Line => format!("line {}", u.index + 1),
    };
    format!("{} {place}: {:?}", face.name, u.text)
}

/// The parse of each unit, read face by face as the census reads it
/// (`sylex_english::face::read_units`): a face's lines are parsed together
/// when the first of its units is asked for, once each, and each unit's
/// parse is taken once.
struct FaceParses<'a, 'g> {
    corpus: &'a Corpus,
    cleaves: &'a [Option<sylex_english::cleave::CleaveText>],
    env: &'a LexicalEnvironment,
    grammar: &'g CompiledGrammar,
    ranges: Vec<std::ops::Range<usize>>,
    faces: Vec<OnceLock<Vec<Mutex<Option<sylex_syntax::Parse<'g>>>>>>,
}

impl<'a, 'g> FaceParses<'a, 'g> {
    fn new(
        corpus: &'a Corpus,
        cleaves: &'a [Option<sylex_english::cleave::CleaveText>],
        env: &'a LexicalEnvironment,
        grammar: &'g CompiledGrammar,
    ) -> Self {
        Self {
            corpus,
            cleaves,
            env,
            grammar,
            ranges: corpus.face_units(),
            faces: corpus.faces.iter().map(|_| OnceLock::new()).collect(),
        }
    }

    /// The parse of `unit`.
    ///
    /// # Panics
    /// When it was taken before.
    fn take(&self, unit: usize) -> sylex_syntax::Parse<'g> {
        let face = self.corpus.units[unit].face;
        let range = &self.ranges[face];
        let parses = self.faces[face].get_or_init(|| {
            sylex_english::face::read_units(
                &self.corpus.faces[face],
                &self.corpus.units[range.clone()],
                &self.cleaves[range.clone()],
                &self.env.lexicon,
                self.grammar,
            )
            .into_iter()
            .map(|parse| Mutex::new(Some(parse)))
            .collect()
        });
        parses[unit - range.start]
            .lock()
            .expect("no thread panics holding a unit's parse")
            .take()
            .expect("a unit's parse is taken once")
    }
}

/// Gathers the rows of the part of a unit's forest some Derivation uses:
/// `parse` is its parse, as the census reads it, and `parsed` the text it
/// was read as.
fn unit_rows(
    parsed: &str,
    parse: &sylex_syntax::Parse<'_>,
    grammar: &CompiledGrammar,
    reader: &context::Reader<'_>,
) -> Result<UnitRows> {
    let parsed = parsed.to_owned();
    let counts = parse.counts()?;
    let heads = parse.alternative_heads()?;
    let conjuncts = parse.alternative_conjuncts()?;
    let embeddings = parse.alternative_embeddings(context::DEPTH, |step| reader.wrapper(step))?;
    exclusive(parse, &counts)?;
    let forest = parse.forest();
    let tokens = parse.tokens();
    // A Constituent's span begins where it was predicted, before the space
    // that precedes its first word.
    let surface = |span: &std::ops::Range<usize>| {
        tokens[span.clone()]
            .iter()
            .collect::<String>()
            .trim_start()
            .to_owned()
    };
    let roots: BTreeSet<usize> = forest.roots().iter().copied().collect();
    let mut unit_embeddings: Vec<Embedding> = Vec::new();
    let mut unit_embedding_index: HashMap<Embedding, usize> = HashMap::new();
    let mut nodes = Vec::new();
    for node in (0..forest.node_count()).filter(|&n| counts.node(n) > 0) {
        let span = forest.node_span(node);
        let coordinator = parse.node_coordinator(node);
        let features: BTreeMap<&str, &str> = parse
            .node_features(node)
            .into_iter()
            .map(|(f, v)| (f.as_str(), v.as_str()))
            .collect();
        let alternatives = forest
            .node_alternatives(node)
            .enumerate()
            .map(|(index, (production, item))| {
                let outside = counts.node_outside(node);
                let inside = counts.item_inside(item);
                let word = |leaf: Option<usize>| match forest.leaf_reading(leaf?) {
                    LexicalReading::Word(word) => Some((word.lexeme, word.form)),
                    LexicalReading::OwnName { .. }
                    | LexicalReading::IntroducedName { .. }
                    | LexicalReading::FlavorWord { .. }
                    | LexicalReading::SetName { .. }
                    | LexicalReading::AnchorWord { .. }
                    | LexicalReading::VoteOption { .. }
                    | LexicalReading::Numeral { .. } => None,
                };
                Ok(AlternativeRow {
                    production,
                    item,
                    derivations: counts.node_family(node, index),
                    heads: heads[node][index]
                        .iter()
                        .map(|(&leaf, &own)| Ok((leaf, word(leaf), own, product(own, outside)?)))
                        .collect::<Result<_>>()?,
                    embeddings: embeddings[node][index]
                        .iter()
                        .map(|(embedding, &ways)| {
                            let local = if let Some(&id) = unit_embedding_index.get(embedding) {
                                id
                            } else {
                                let id = unit_embeddings.len();
                                unit_embeddings.push(embedding.clone());
                                unit_embedding_index.insert(embedding.clone(), id);
                                id
                            };
                            Ok((local, ways, product(ways, inside)?))
                        })
                        .collect::<Result<_>>()?,
                    conjuncts: match coordinator {
                        Some(_) if !grammar.production_is_tail(production) => conjuncts[node]
                            [index]
                            .iter()
                            .map(|(&n, &own)| Ok((n, product(own, outside)?)))
                            .collect::<Result<_>>()?,
                        Some(_) | None => Vec::new(),
                    },
                })
            })
            .collect::<Result<_>>()?;
        nodes.push(NodeRow {
            node,
            category: forest.node_category(node),
            surface: surface(&span),
            span,
            gap: match parse.node_gap(node) {
                Gap::None => None,
                gap => Some(format!("{gap:?}")),
            },
            features: serde_json::to_string(&features)?,
            coordinator,
            root: roots.contains(&node),
            settled: counts.settled(node),
            derivations: counts.node(node),
            alternatives,
        });
    }
    let mut items = Vec::new();
    let mut used_leaves = BTreeSet::new();
    for item in (0..forest.item_count()).filter(|&i| counts.item(i) > 0) {
        let families: Vec<_> = forest
            .item_families(item)
            .enumerate()
            .map(|(index, (previous, child))| {
                if let ChildRef::Leaf(leaf) = child {
                    used_leaves.insert(leaf);
                }
                (previous, child, counts.item_family(item, index))
            })
            .collect();
        items.push(ItemRow {
            item,
            production: forest.item_production(item),
            dot: forest.item_dot(item),
            span: forest.item_span(item),
            derivations: counts.item(item),
            families,
        });
    }
    let leaves = used_leaves
        .into_iter()
        .map(|leaf| {
            let span = forest.leaf_span(leaf);
            LeafRow {
                leaf,
                surface: surface(&span),
                span,
                reading: forest.leaf_reading(leaf).clone(),
            }
        })
        .collect::<Vec<_>>();
    let forks = forks::rows(parse, &leaves)?;
    Ok(UnitRows {
        derivations: counts.total(),
        parsed,
        embeddings: unit_embeddings,
        nodes,
        items,
        leaves,
        forks,
    })
}

/// The product of two counts, which is a count of the unit's Derivations.
fn product(a: u128, b: u128) -> Result<u128> {
    a.checked_mul(b)
        .with_context(|| format!("a Derivation count overflows u128: {a} * {b}"))
}

/// Checks the two facts that make sums across nodes exact (the
/// `verb_phrases` view states them): no Derivation holds two alternatives of
/// one Production over one span, nor two verb phrase alternatives
/// (frame-headed, and no interleaved prefix) on one head leaf. Two such
/// alternatives in one Derivation would be nested, the inner one under the
/// outer one's item through children of the same span, or through `head`
/// children; any path down the forest is part of some Derivation.
fn exclusive(parse: &sylex_syntax::Parse<'_>, counts: &sylex_syntax::Counts<'_>) -> Result<()> {
    let forest = parse.forest();
    let grammar = parse.grammar();
    // A verb phrase: a frame-headed Production other than an interleaved
    // prefix, which is the head of its `/interleaved` Production and part of
    // that one verb phrase.
    let framed = |production: usize| {
        grammar.production_frame(production).is_some()
            && !grammar.production_is_interleaved_prefix(production)
    };
    for node in (0..forest.node_count()).filter(|&n| counts.node(n) > 0) {
        let span = forest.node_span(node);
        for (production, item) in forest.node_alternatives(node) {
            let below = descendants(parse, item, &|child, _| forest.node_span(child) == span);
            for inner in below {
                if forest
                    .node_alternatives(inner)
                    .any(|(p, _)| p == production)
                {
                    anyhow::bail!(
                        "nodes {node} and {inner} share a span and Production `{}` and can share \
                         a Derivation",
                        grammar.production_id(production)
                    );
                }
            }
            if framed(production) {
                let below = descendants(parse, item, &|_, function| function == "head");
                for inner in below {
                    if let Some((p, _)) = forest.node_alternatives(inner).find(|&(p, _)| framed(p))
                    {
                        anyhow::bail!(
                            "frame-headed `{}` (node {node}) and `{}` (node {inner}) share a head \
                             leaf and can share a Derivation",
                            grammar.production_id(production),
                            grammar.production_id(p)
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

/// The nodes reached from a completed item through children `follow` admits
/// (by node and function), and on through their alternatives' items.
fn descendants(
    parse: &sylex_syntax::Parse<'_>,
    item: usize,
    follow: &dyn Fn(usize, &str) -> bool,
) -> BTreeSet<usize> {
    let forest = parse.forest();
    let grammar = parse.grammar();
    let mut items = vec![item];
    let mut seen_items = BTreeSet::new();
    let mut nodes = BTreeSet::new();
    while let Some(item) = items.pop() {
        if !seen_items.insert(item) {
            continue;
        }
        let function = grammar.production_functions(forest.item_production(item))
            [parse.item_child(item)]
        .as_str();
        for (previous, child) in forest.item_families(item) {
            items.extend(previous);
            if let ChildRef::Node(child) = child
                && follow(child, function)
                && nodes.insert(child)
            {
                items.extend(forest.node_items(child));
            }
        }
    }
    nodes
}

/// A count as SQLite's 64-bit integer.
fn int(n: u128) -> Result<i64> {
    i64::try_from(n).with_context(|| format!("count {n} exceeds SQLite's 64-bit integers"))
}

fn index(n: usize) -> i64 {
    i64::try_from(n).expect("an index fits in i64")
}

fn write_grammar(
    tx: &rusqlite::Transaction<'_>,
    grammar: &CompiledGrammar,
    summary: &mut Summary,
) -> Result<()> {
    let mut category = tx.prepare("INSERT INTO categories VALUES (?1, ?2)")?;
    for (index, name) in grammar.category_names().iter().enumerate() {
        category.execute(params![self::index(index), name.as_str()])?;
    }
    summary
        .rows
        .insert("categories", index(grammar.category_names().len()));
    let mut production =
        tx.prepare("INSERT INTO productions VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")?;
    for p in 0..grammar.production_count() {
        let functions: Vec<String> = grammar
            .production_functions(p)
            .iter()
            .map(ToString::to_string)
            .collect();
        let frame = grammar.production_frame(p);
        production.execute(params![
            index(p),
            grammar.production_id(p).as_str(),
            index(grammar.production_category(p)),
            serde_json::to_string(&functions)?,
            frame.as_ref().map(|(_, label, _)| label.as_str()),
            frame.as_ref().map(|(_, _, variant)| *variant),
            grammar.production_is_tail(p),
            grammar.production_is_interleaved_prefix(p),
        ])?;
    }
    summary
        .rows
        .insert("productions", index(grammar.production_count()));
    Ok(())
}

/// The per-unit `INSERT`s, prepared once for the transaction.
struct Inserts<'t> {
    unit: BatchInsert<'t, 10, 64>,
    node: BatchInsert<'t, 13, 64>,
    alternative: BatchInsert<'t, 6, 128>,
    head: BatchInsert<'t, 8, 128>,
    embedding: BatchInsert<'t, 6, 128>,
    conjunct: BatchInsert<'t, 5, 64>,
    item: BatchInsert<'t, 7, 128>,
    family: BatchInsert<'t, 7, 128>,
    leaf: BatchInsert<'t, 9, 96>,
    forks: forks::Inserts<'t>,
}

impl<'t> Inserts<'t> {
    fn prepare(tx: &'t Connection) -> Result<Self> {
        Ok(Self {
            unit: BatchInsert::prepare(tx, "units")?,
            node: BatchInsert::prepare(tx, "nodes")?,
            alternative: BatchInsert::prepare(tx, "alternatives")?,
            head: BatchInsert::prepare(tx, "heads")?,
            embedding: BatchInsert::prepare(tx, "alternative_embeddings")?,
            conjunct: BatchInsert::prepare(tx, "conjuncts")?,
            item: BatchInsert::prepare(tx, "items")?,
            family: BatchInsert::prepare(tx, "families")?,
            leaf: BatchInsert::prepare(tx, "leaves")?,
            forks: forks::Inserts::prepare(tx)?,
        })
    }

    /// Flushes every table's remainder and records its row count in `rows`.
    fn finish(&mut self, rows: &mut BTreeMap<&'static str, i64>) -> Result<()> {
        rows.extend([
            self.unit.finish()?,
            self.node.finish()?,
            self.alternative.finish()?,
            self.head.finish()?,
            self.embedding.finish()?,
            self.conjunct.finish()?,
            self.item.finish()?,
            self.family.finish()?,
            self.leaf.finish()?,
        ]);
        self.forks.finish(rows)
    }
}

struct WriterState {
    lexemes: HashSet<LexemeId>,
    embeddings: HashMap<Embedding, i64>,
    unit_embedding_ids: Vec<i64>,
    summary: Summary,
}

fn write_unit(
    inserts: &mut Inserts<'_>,
    state: &mut WriterState,
    corpus: &Corpus,
    unit: usize,
    rows: &UnitRows,
) -> Result<()> {
    let u = &corpus.units[unit];
    let face = &corpus.faces[u.face];
    let (oracle_id, discriminator) = face
        .identity
        .split_once('#')
        .with_context(|| format!("face identity `{}` has no `#`", face.identity))?;
    let face_index: Option<i64> = match discriminator {
        "card" => None,
        other => Some(
            other
                .strip_prefix("face:")
                .and_then(|n| n.parse().ok())
                .with_context(|| format!("face identity `{}` names no face", face.identity))?,
        ),
    };
    let u_id = index(unit);
    inserts.unit.push([
        Val::Int(u_id),
        Val::Text(oracle_id),
        face_index.map_or(Val::Null, Val::Int),
        Val::Text(&face.name),
        Val::Text(&face.card),
        Val::Static(match u.kind {
            UnitKind::TypeLine => "type line",
            UnitKind::Line => "line",
        }),
        Val::Int(index(u.index)),
        Val::Text(&u.text),
        Val::Text(&rows.parsed),
        Val::Int(int(rows.derivations)?),
    ])?;
    state.unit_embedding_ids.clear();
    state.unit_embedding_ids.reserve(rows.embeddings.len());
    for embedding in &rows.embeddings {
        let id = if let Some(&id) = state.embeddings.get(embedding) {
            id
        } else {
            let id = index(state.embeddings.len());
            state.embeddings.insert(embedding.clone(), id);
            id
        };
        state.unit_embedding_ids.push(id);
    }
    write_nodes(
        inserts,
        u_id,
        &rows.nodes,
        &mut state.lexemes,
        &state.unit_embedding_ids,
    )?;
    let families = write_items(inserts, u_id, &rows.items)?;
    write_leaves(inserts, u_id, &rows.leaves, &mut state.lexemes)?;
    forks::write(&mut inserts.forks, u_id, &rows.forks)?;
    let summary = &mut state.summary;
    summary.units += 1;
    let size = (rows.nodes.len() + rows.items.len() + families) as u64;
    if summary.largest.as_ref().is_none_or(|(n, _)| size > *n) {
        summary.largest = Some((size, describe(corpus, unit)));
    }
    if summary
        .most_derivations
        .as_ref()
        .is_none_or(|(n, _)| rows.derivations > *n)
    {
        summary.most_derivations = Some((rows.derivations, describe(corpus, unit)));
    }
    Ok(())
}

fn write_nodes(
    inserts: &mut Inserts<'_>,
    u_id: i64,
    nodes: &[NodeRow],
    lexemes: &mut HashSet<LexemeId>,
    unit_embedding_ids: &[i64],
) -> Result<()> {
    for node in nodes {
        let (coordinator, link) = match node.coordinator {
            Some(Coordinator::Lexeme(id)) => {
                lexemes.insert(id);
                (Val::Static(id.as_str()), Val::Null)
            }
            Some(Coordinator::Linked(tail)) => (Val::Null, Val::Text(tail.as_str())),
            None => (Val::Null, Val::Null),
        };
        inserts.node.push([
            Val::Int(u_id),
            Val::Int(index(node.node)),
            Val::Int(index(node.category)),
            Val::Int(index(node.span.start)),
            Val::Int(index(node.span.end)),
            Val::Text(&node.surface),
            node.gap.as_deref().map_or(Val::Null, Val::Text),
            Val::Text(&node.features),
            coordinator,
            link,
            Val::Bool(node.root),
            Val::Bool(node.settled),
            Val::Int(int(node.derivations)?),
        ])?;
        for (a, alternative) in node.alternatives.iter().enumerate() {
            let key = (u_id, index(node.node), index(a));
            inserts.alternative.push([
                Val::Int(key.0),
                Val::Int(key.1),
                Val::Int(key.2),
                Val::Int(index(alternative.production)),
                Val::Int(index(alternative.item)),
                Val::Int(int(alternative.derivations)?),
            ])?;
            for (leaf, word, inside, derivations) in &alternative.heads {
                if let Some((lexeme, _)) = word {
                    lexemes.insert(*lexeme);
                }
                inserts.head.push([
                    Val::Int(key.0),
                    Val::Int(key.1),
                    Val::Int(key.2),
                    leaf.map_or(Val::Null, |l| Val::Int(index(l))),
                    word.map_or(Val::Null, |(lexeme, _)| Val::Static(lexeme.as_str())),
                    word.as_ref()
                        .map_or(Val::Null, |(_, form)| Val::Debug(form)),
                    Val::Int(int(*inside)?),
                    Val::Int(int(*derivations)?),
                ])?;
            }
            for &(local_emb, outside, derivations) in &alternative.embeddings {
                inserts.embedding.push([
                    Val::Int(key.0),
                    Val::Int(key.1),
                    Val::Int(key.2),
                    Val::Int(unit_embedding_ids[local_emb]),
                    Val::Int(int(outside)?),
                    Val::Int(int(derivations)?),
                ])?;
            }
            for (conjuncts, derivations) in &alternative.conjuncts {
                inserts.conjunct.push([
                    Val::Int(key.0),
                    Val::Int(key.1),
                    Val::Int(key.2),
                    Val::Int(index(*conjuncts)),
                    Val::Int(int(*derivations)?),
                ])?;
            }
        }
    }
    Ok(())
}

/// Writes a unit's items and their families; returns the families written.
fn write_items(inserts: &mut Inserts<'_>, u_id: i64, items: &[ItemRow]) -> Result<usize> {
    let mut families = 0usize;
    for item in items {
        inserts.item.push([
            Val::Int(u_id),
            Val::Int(index(item.item)),
            Val::Int(index(item.production)),
            Val::Int(index(item.dot)),
            Val::Int(index(item.span.start)),
            Val::Int(index(item.span.end)),
            Val::Int(int(item.derivations)?),
        ])?;
        for (f, (previous, child, derivations)) in item.families.iter().enumerate() {
            let (node, leaf) = match child {
                ChildRef::Node(n) => (Val::Int(index(*n)), Val::Null),
                ChildRef::Leaf(l) => (Val::Null, Val::Int(index(*l))),
            };
            inserts.family.push([
                Val::Int(u_id),
                Val::Int(index(item.item)),
                Val::Int(index(f)),
                previous.map_or(Val::Null, |p| Val::Int(index(p))),
                node,
                leaf,
                Val::Int(int(*derivations)?),
            ])?;
            families += 1;
        }
    }
    Ok(families)
}

fn write_leaves(
    inserts: &mut Inserts<'_>,
    u_id: i64,
    leaves: &[LeafRow],
    lexemes: &mut HashSet<LexemeId>,
) -> Result<()> {
    for leaf in leaves {
        let (kind, lexeme, form, value) = match &leaf.reading {
            LexicalReading::Word(word) => {
                lexemes.insert(word.lexeme);
                (
                    Val::Static("word"),
                    Val::Static(word.lexeme.as_str()),
                    Val::Debug(&word.form),
                    Val::Null,
                )
            }
            LexicalReading::OwnName { form, .. } => (
                Val::Static("own name"),
                Val::Null,
                Val::Debug(form),
                Val::Null,
            ),
            LexicalReading::IntroducedName { .. } => (
                Val::Static("introduced name"),
                Val::Null,
                Val::Null,
                Val::Null,
            ),
            LexicalReading::FlavorWord { .. } => {
                (Val::Static("flavor word"), Val::Null, Val::Null, Val::Null)
            }
            LexicalReading::SetName { .. } => {
                (Val::Static("set name"), Val::Null, Val::Null, Val::Null)
            }
            LexicalReading::AnchorWord { .. } => {
                (Val::Static("anchor word"), Val::Null, Val::Null, Val::Null)
            }
            LexicalReading::VoteOption { .. } => {
                (Val::Static("vote option"), Val::Null, Val::Null, Val::Null)
            }
            LexicalReading::Numeral { value, .. } => (
                Val::Static("numeral"),
                Val::Null,
                Val::Null,
                Val::Int(i64::from(*value)),
            ),
        };
        inserts.leaf.push([
            Val::Int(u_id),
            Val::Int(index(leaf.leaf)),
            Val::Int(index(leaf.span.start)),
            Val::Int(index(leaf.span.end)),
            Val::Text(&leaf.surface),
            kind,
            lexeme,
            form,
            value,
        ])?;
    }
    Ok(())
}

fn write_embeddings(
    tx: &rusqlite::Transaction<'_>,
    embeddings: &HashMap<Embedding, i64>,
    reader: &context::Reader<'_>,
    lexemes: &mut HashSet<LexemeId>,
    summary: &mut Summary,
) -> Result<()> {
    let mut embedding_insert: BatchInsert<'_, 4, 128> = BatchInsert::prepare(tx, "embeddings")?;
    let mut step_insert: BatchInsert<'_, 9, 96> = BatchInsert::prepare(tx, "embedding_steps")?;
    // In id order, so each table grows at its end.
    let mut in_order: Vec<(&Embedding, i64)> = embeddings.iter().map(|(e, &id)| (e, id)).collect();
    in_order.sort_unstable_by_key(|&(_, id)| id);
    for (embedding, id) in in_order {
        embedding_insert.push([
            Val::Int(id),
            Val::Int(index(embedding.steps.len())),
            Val::Bool(embedding.truncated),
            Val::Static(reader.context(embedding).name()),
        ])?;
        for (n, step) in embedding.steps.iter().enumerate() {
            if let Some((lexeme, _)) = step.governor_head {
                lexemes.insert(lexeme);
            }
            step_insert.push([
                Val::Int(id),
                Val::Int(index(n + 1)),
                Val::Int(index(step.projection)),
                Val::Int(index(step.child)),
                Val::Static(step.function.as_str()),
                Val::Int(index(step.governor)),
                step.governor_head
                    .map_or(Val::Null, |(lexeme, _)| Val::Static(lexeme.as_str())),
                step.governor_head
                    .as_ref()
                    .map_or(Val::Null, |(_, form)| Val::Debug(form)),
                Val::Bool(reader.wrapper(step)),
            ])?;
        }
    }
    summary
        .rows
        .extend([embedding_insert.finish()?, step_insert.finish()?]);
    Ok(())
}

fn write_lexemes(
    tx: &rusqlite::Transaction<'_>,
    lexemes: &HashSet<LexemeId>,
    env: &LexicalEnvironment,
    summary: &mut Summary,
) -> Result<()> {
    let mut sorted: Vec<LexemeId> = lexemes.iter().copied().collect();
    sorted.sort_unstable_by_key(LexemeId::as_str);
    let mut insert = tx.prepare("INSERT INTO lexemes VALUES (?1, ?2, ?3)")?;
    for id in sorted {
        let lexeme = env
            .lexicon
            .lexemes()
            .get(&id)
            .with_context(|| format!("lexeme `{}` is not in the lexicon", id.as_str()))?;
        insert.execute(params![
            id.as_str(),
            lexeme.lemma,
            format!("{:?}", lexeme.category)
        ])?;
    }
    summary.rows.insert("lexemes", index(lexemes.len()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    /// Each item's families: (previous item, child node).
    type Families = HashMap<i64, Vec<(Option<i64>, Option<i64>)>>;

    /// An item's Derivations, from the stored rows.
    fn inside(
        i: i64,
        families: &Families,
        alternatives: &HashMap<i64, Vec<i64>>,
        memo: &mut HashMap<i64, i64>,
    ) -> i64 {
        if let Some(&n) = memo.get(&i) {
            return n;
        }
        let n = families[&i]
            .iter()
            .map(|&(previous, child)| {
                previous.map_or(1, |p| inside(p, families, alternatives, memo))
                    * child.map_or(1, |c| {
                        alternatives[&c]
                            .iter()
                            .map(|&a| inside(a, families, alternatives, memo))
                            .sum()
                    })
            })
            .sum();
        memo.insert(i, n);
        n
    }

    /// Writes the database for the faces of the named cards.
    pub(super) fn database(cards: &[&str]) -> (tempfile::TempDir, PathBuf, Corpus) {
        let env = LexicalEnvironment::load().unwrap();
        let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
        let corpus = Corpus::load(|face| cards.contains(&face.card.as_str())).unwrap();
        assert!(!corpus.units.is_empty(), "no unit of {cards:?}");
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("forests.sqlite");
        write(&path, &corpus, &env, &grammar).unwrap();
        (dir, path, corpus)
    }

    /// The parse of each unit of `corpus`, as the census reads it.
    pub(super) fn parses<'g>(
        corpus: &Corpus,
        env: &LexicalEnvironment,
        grammar: &'g CompiledGrammar,
    ) -> Vec<sylex_syntax::Parse<'g>> {
        let cleaves = sylex_english::census::cleave_texts(corpus, &env.lexicon);
        corpus
            .face_units()
            .into_iter()
            .enumerate()
            .flat_map(|(face, range)| {
                sylex_english::face::read_units(
                    &corpus.faces[face],
                    &corpus.units[range.clone()],
                    &cleaves[range],
                    &env.lexicon,
                    grammar,
                )
            })
            .collect()
    }

    fn column(db: &Connection, sql: &str) -> Vec<i64> {
        let mut statement = db.prepare(sql).unwrap();
        statement
            .query_map([], |row| row.get(0))
            .unwrap()
            .map(Result::unwrap)
            .collect()
    }

    /// A unit's Derivations counted again from the stored rows alone: each
    /// root node's alternatives' items, each item's families of previous
    /// item and child.
    fn recount(db: &Connection, unit: i64) -> i64 {
        let mut alternatives: HashMap<i64, Vec<i64>> = HashMap::new();
        let mut statement = db
            .prepare("SELECT node, item FROM alternatives WHERE unit = ?1")
            .unwrap();
        for row in statement
            .query_map([unit], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
        {
            let (node, item): (i64, i64) = row.unwrap();
            alternatives.entry(node).or_default().push(item);
        }
        let mut families = Families::new();
        let mut statement = db
            .prepare("SELECT item, previous, child_node FROM families WHERE unit = ?1")
            .unwrap();
        for row in statement
            .query_map([unit], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
            .unwrap()
        {
            let (item, previous, child) = row.unwrap();
            families.entry(item).or_default().push((previous, child));
        }
        let roots = column(
            db,
            &format!("SELECT node FROM nodes WHERE unit = {unit} AND root"),
        );
        let mut memo = HashMap::new();
        roots
            .iter()
            .flat_map(|root| &alternatives[root])
            .map(|&a| inside(a, &families, &alternatives, &mut memo))
            .sum()
    }

    /// Every table's rows, ordered, as text.
    fn dump(db: &Connection) -> Vec<String> {
        let mut statement = db
            .prepare("SELECT name FROM sqlite_schema WHERE type = 'table' ORDER BY name")
            .unwrap();
        let names: Vec<String> = statement
            .query_map([], |r| r.get(0))
            .unwrap()
            .map(Result::unwrap)
            .collect();
        let mut rows = Vec::new();
        for name in names {
            let width = db
                .prepare(&format!("SELECT * FROM {name}"))
                .unwrap()
                .column_count();
            let order: Vec<String> = (1..=width).map(|i| i.to_string()).collect();
            let mut statement = db
                .prepare(&format!(
                    "SELECT * FROM {name} ORDER BY {}",
                    order.join(", ")
                ))
                .unwrap();
            let mut query = statement.query([]).unwrap();
            while let Some(row) = query.next().unwrap() {
                let values: Vec<String> = (0..width)
                    .map(|i| format!("{:?}", row.get::<_, rusqlite::types::Value>(i).unwrap()))
                    .collect();
                rows.push(format!("{name}: {}", values.join("|")));
            }
        }
        rows
    }

    /// *Crook of Condemnation*'s three units, *Delver of Secrets*'s
    /// double-faced card, and *Jace, Cunning Castaway*, whose text shortens
    /// its own name: the stored totals equal `derivation_count`, the stored
    /// rows recount to them, and each node's, alternative's and item's
    /// stored count is the sum it must be.
    #[test]
    fn the_stored_rows_reproduce_each_unit_s_forest_and_its_counts() {
        let cards = [
            "Crook of Condemnation",
            "Delver of Secrets // Insectile Aberration",
            "Jace, Cunning Castaway",
        ];
        let (_dir, path, corpus) = database(&cards);
        let db = Connection::open(&path).unwrap();
        let env = LexicalEnvironment::load().unwrap();
        let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
        let totals = column(&db, "SELECT derivations FROM units ORDER BY unit");
        assert_eq!(totals.len(), corpus.units.len());
        let parses = parses(&corpus, &env, &grammar);
        for (unit, u) in corpus.units.iter().enumerate() {
            let expected = parses[unit].derivation_count();
            assert_eq!(
                u128::try_from(totals[unit]).unwrap(),
                expected,
                "{}",
                u.text
            );
            assert_eq!(recount(&db, index(unit)), totals[unit], "{}", u.text);
        }
        assert!(totals.iter().any(|&n| n > 1), "{totals:?}");
        for (what, sql) in [
            (
                "a node's alternatives",
                "SELECT count(*) FROM nodes n WHERE derivations != (SELECT sum(derivations) \
                 FROM alternatives a WHERE a.unit = n.unit AND a.node = n.node)",
            ),
            (
                "an alternative's head leaves",
                "SELECT count(*) FROM alternatives a WHERE derivations != (SELECT \
                 sum(derivations) FROM heads h WHERE (h.unit, h.node, h.alternative) = \
                 (a.unit, a.node, a.alternative))",
            ),
            (
                "an alternative's Embeddings",
                "SELECT count(*) FROM alternatives a WHERE derivations != (SELECT \
                 sum(derivations) FROM alternative_embeddings e WHERE (e.unit, e.node, \
                 e.alternative) = (a.unit, a.node, a.alternative))",
            ),
            (
                "a coordination alternative's conjunct counts",
                "SELECT count(*) FROM alternatives a JOIN nodes n USING (unit, node) JOIN \
                 productions p USING (production) WHERE (n.coordinator IS NOT NULL OR n.link IS \
                 NOT NULL) AND NOT p.tail AND a.derivations != (SELECT sum(derivations) FROM \
                 conjuncts j WHERE (j.unit, j.node, j.alternative) = (a.unit, a.node, \
                 a.alternative))",
            ),
            (
                "an item's families",
                "SELECT count(*) FROM items i WHERE derivations != (SELECT sum(derivations) \
                 FROM families f WHERE f.unit = i.unit AND f.item = i.item)",
            ),
            (
                "the unit's roots",
                "SELECT count(*) FROM units u WHERE derivations != (SELECT \
                 coalesce(sum(derivations), 0) FROM nodes n WHERE n.unit = u.unit AND root)",
            ),
            (
                "a settled node",
                "SELECT count(*) FROM nodes n JOIN units u USING (unit) WHERE settled != \
                 (n.derivations = u.derivations)",
            ),
        ] {
            assert_eq!(column(&db, sql), [0], "{what}");
        }
    }

    /// *Crook of Condemnation*'s *Exile target card from a graveyard* has two
    /// Derivations: *from a graveyard* modifies *card* or *exile*. Each verb
    /// phrase of *exile* is in one, an effect clause.
    #[test]
    fn crook_of_condemnation_s_verb_phrases_split_its_attachment() {
        let (_dir, path, _) = database(&["Crook of Condemnation"]);
        let db = Connection::open(&path).unwrap();
        let mut statement = db
            .prepare(
                "SELECT v.surface, v.lemma, v.frame, v.voice, v.context, v.derivations, \
                 u.derivations FROM verb_phrases v JOIN units u USING (unit) WHERE u.line = 0 \
                 AND u.kind = 'line' ORDER BY v.surface",
            )
            .unwrap();
        let rows: Vec<String> = statement
            .query_map([], |r| {
                Ok((0..7)
                    .map(|i| format!("{:?}", r.get::<_, rusqlite::types::Value>(i).unwrap()))
                    .collect::<Vec<_>>()
                    .join(" "))
            })
            .unwrap()
            .map(Result::unwrap)
            .collect();
        assert_eq!(rows.len(), 2, "{rows:#?}");
        for (row, surface) in rows
            .iter()
            .zip(["Exile target card", "Exile target card from a graveyard"])
        {
            assert!(
                row.starts_with(&format!(
                    "Text(\"{surface}\") Text(\"exile\") Text(\"Monotransitive(NP)\") \
                     Text(\"Active\") Text(\"effect clause\") Integer(1) Integer(2)"
                )),
                "{row}"
            );
        }
    }

    /// *Acolyte of Bahamut*'s *The first Dragon spell you cast each turn*
    /// has two Derivations, *cast* Present in one and Preterite in the
    /// other: two nodes over one span, each in one Derivation. Summed by
    /// span and Production, *cast* is in both.
    #[test]
    fn nodes_sharing_a_span_and_production_sum_to_every_derivation() {
        let (_dir, path, _) = database(&["Acolyte of Bahamut"]);
        let db = Connection::open(&path).unwrap();
        let per_node = column(
            &db,
            "SELECT sum(v.derivations) FROM verb_phrases v JOIN units u USING (unit) WHERE \
             u.line = 0 AND u.kind = 'line' AND v.lemma = 'cast' AND \
             v.context = 'relative clause' GROUP BY v.node",
        );
        assert_eq!(per_node, [1, 1]);
        let spans = column(
            &db,
            "SELECT s.every FROM verb_phrase_spans s JOIN units u USING (unit) WHERE u.line = 0 \
             AND u.kind = 'line' AND s.lemma = 'cast' AND s.context = 'relative clause' AND \
             s.derivations = u.derivations",
        );
        assert_eq!(spans, [1]);
        let tokens = column(
            &db,
            "SELECT t.every FROM verb_phrase_tokens t JOIN units u USING (unit) WHERE u.line = 0 \
             AND u.kind = 'line' AND t.lemma = 'cast' AND t.context = 'relative clause'",
        );
        assert_eq!(tokens, [1]);
    }

    /// Each context on a real unit: (card, verb, the verb phrase's text,
    /// its context). A passive under *would* (*Glacial Chasm*), conjuncts
    /// under a deontic (*Vigean Hydropon*), relative clauses of four
    /// Productions, and a participial postmodifier of each Production that
    /// builds one.
    #[test]
    fn each_context_is_read_on_real_units() {
        const CASES: &[(&str, &str, &str, &str)] = &[
            ("Field Trip", "learn", "Learn", "effect clause"),
            ("Ao, the Dawn Sky", "die", "dies", "trigger clause"),
            ("Possessed Skaab", "die", "die", "would clause"),
            ("Glacial Chasm", "deal", "dealt to you", "would clause"),
            ("Absorbing Man and Titania", "deal", "deal", "would clause"),
            ("Vigean Hydropon", "attack", "attack", "deontic"),
            ("Vigean Hydropon", "block", "block", "deontic"),
            ("Circling Vultures", "cast", "cast an instant", "deontic"),
            ("Dawn Charm", "target", "targets you", "relative clause"),
            ("Tourach's Gate", "control", "control", "relative clause"),
            (
                "Absorbing Man and Titania",
                "will",
                "would deal",
                "relative clause",
            ),
            (
                "Circling Vultures",
                "can",
                "could cast an instant",
                "relative clause",
            ),
            (
                "Jungle Patrol",
                "name",
                "named Wood",
                "participial postmodifier",
            ),
            (
                "Boarded Window",
                "attack",
                "attacking you",
                "participial postmodifier",
            ),
            (
                "Enduring Renewal",
                "reveal",
                "revealed",
                "participial postmodifier",
            ),
            (
                "Torpor Orb",
                "enter",
                "entering",
                "participial postmodifier",
            ),
            ("Graven Lore", "spend", "spent", "participial postmodifier"),
            (
                "Gorex, the Tombshell",
                "exile",
                "exiled",
                "participial postmodifier",
            ),
            (
                "Shadow of the Grave",
                "discard",
                "discarded",
                "participial postmodifier",
            ),
            // `GerundParticipialConjunct`: *target creature [blocking] or
            // [being blocked by this creature]*.
            (
                "Sworn Defender",
                "be",
                "being blocked by this creature",
                "participial postmodifier",
            ),
            ("Road of Return", "choose", "Choose one", "other"),
        ];
        let cards: BTreeSet<&str> = CASES.iter().map(|case| case.0).collect();
        let (_dir, path, _) = database(&cards.into_iter().collect::<Vec<_>>());
        let db = Connection::open(&path).unwrap();
        for &(card, lemma, surface, context) in CASES {
            let found: Vec<String> = db
                .prepare(
                    "SELECT DISTINCT s.context FROM verb_phrase_spans s JOIN units u USING \
                     (unit) WHERE u.face_name = ?1 AND s.lemma = ?2 AND s.surface = ?3",
                )
                .unwrap()
                .query_map([card, lemma, surface], |r| r.get(0))
                .unwrap()
                .map(Result::unwrap)
                .collect();
            assert!(
                found.iter().any(|c| c == context),
                "{card}: {surface:?} is {found:?}, not {context}"
            );
        }
        // The passive under *would* is a passive, the would its step above
        // the passive auxiliary's Complement.
        let passive = column(
            &db,
            "SELECT count(*) FROM verb_phrases v JOIN units u USING (unit) WHERE u.face_name = \
             'Glacial Chasm' AND v.lemma = 'deal' AND v.variant LIKE 'passive%' AND v.context = \
             'would clause'",
        );
        assert!(passive[0] > 0);
        // A conjunct under *can't* stands under its coordination first.
        let conjunct = column(
            &db,
            "SELECT count(*) FROM verb_phrases v JOIN units u USING (unit) JOIN \
             embedding_steps s ON s.embedding = v.embedding AND s.step = 1 JOIN productions g ON \
             g.production = s.governor WHERE u.face_name = 'Vigean Hydropon' AND v.lemma = \
             'attack' AND s.wrapper AND g.name LIKE 'Coordination:%'",
        );
        assert!(conjunct[0] > 0);
    }

    #[test]
    fn writing_twice_gives_the_same_content() {
        let cards = [
            "Crook of Condemnation",
            "Delver of Secrets // Insectile Aberration",
        ];
        let (_a, first, _) = database(&cards);
        let (_b, second, _) = database(&cards);
        let first = dump(&Connection::open(&first).unwrap());
        assert!(first.len() > 100, "{}", first.len());
        assert_eq!(first, dump(&Connection::open(&second).unwrap()));
    }
}

//! `cargo xtask english forests`: every Corpus Unit's Packed Forest, with
//! its exact Derivation counts, in one SQLite database.
//!
//! The stored tables keep the forest as the chart packs and binarizes it
//! (`docs/decisions/english.md` §4): a node's alternative is a (Production,
//! completed item) pair, and an item's families are (previous item, child)
//! pairs. Only the part some Derivation uses is stored. The schema, with
//! each table's and view's meaning, is [`SCHEMA`] and [`VIEWS`].

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Context;
use anyhow::Result;
use clap::Args;
use rayon::prelude::*;
use rusqlite::Connection;
use rusqlite::params;
use sylex_english::corpus::Corpus;
use sylex_english::corpus::UnitKind;
use sylex_lexicon::LexemeId;
use sylex_lexicon::LexicalReading;
use sylex_syntax::ChildRef;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Embedding;
use sylex_syntax::Gap;
use sylex_syntax::HeadWord;
use sylex_vocabulary::LexicalEnvironment;

mod context;
mod schema;

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
    nodes: Vec<NodeRow>,
    items: Vec<ItemRow>,
    leaves: Vec<LeafRow>,
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

/// What a coordination node's Summary names as its coordinator.
#[derive(Clone, Copy)]
enum Coordinator {
    Lexeme(LexemeId),
    /// A Linked Coordination's link: the Category of its linked conjunct.
    Link(LexemeId),
}

struct AlternativeRow {
    production: usize,
    item: usize,
    derivations: u128,
    /// (head leaf, its word, the alternative's own Derivations with it, the
    /// unit's).
    heads: Vec<(Option<usize>, Option<HeadWord>, u128, u128)>,
    embeddings: Vec<(Embedding, u128, u128)>,
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

/// How many units are parsed in parallel before their rows are written.
const BATCH: usize = 256;

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
    db.execute_batch("PRAGMA journal_mode = OFF; PRAGMA synchronous = OFF;")?;
    db.execute_batch(SCHEMA)?;
    db.execute_batch(VIEWS)?;
    let cleaves = sylex_english::census::cleave_texts(corpus, &env.lexicon);
    let reader = context::Reader::new(grammar, &env.lexicon);
    let mut lexemes = BTreeSet::new();
    let mut embeddings: BTreeMap<Embedding, i64> = BTreeMap::new();
    let mut summary = Summary::default();
    // Lexemes and Embeddings are written last, so references are checked
    // once at the end (`foreign_key_check`), not on each insert.
    db.execute_batch("PRAGMA foreign_keys = OFF;")?;
    let tx = db.transaction()?;
    write_grammar(&tx, grammar)?;
    for start in (0..corpus.units.len()).step_by(BATCH) {
        let end = (start + BATCH).min(corpus.units.len());
        let rows: Vec<UnitRows> = (start..end)
            .into_par_iter()
            .map(|unit| {
                unit_rows(corpus, unit, cleaves[unit].as_ref(), env, grammar, &reader)
                    .with_context(|| describe(corpus, unit))
            })
            .collect::<Result<_>>()?;
        for (offset, rows) in rows.iter().enumerate() {
            let unit = start + offset;
            write_unit(
                &tx,
                corpus,
                unit,
                rows,
                &mut lexemes,
                &mut embeddings,
                &mut summary,
            )
            .with_context(|| describe(corpus, unit))?;
        }
    }
    write_embeddings(&tx, &embeddings, &reader, &mut lexemes)?;
    write_lexemes(&tx, &lexemes, env)?;
    tx.commit()?;
    let dangling: i64 = db.query_row("SELECT count(*) FROM pragma_foreign_key_check", [], |r| {
        r.get(0)
    })?;
    anyhow::ensure!(dangling == 0, "{dangling} rows reference no row");
    for table in [
        "units",
        "nodes",
        "alternatives",
        "items",
        "families",
        "leaves",
        "heads",
        "embeddings",
        "embedding_steps",
        "alternative_embeddings",
        "conjuncts",
        "lexemes",
        "productions",
        "categories",
    ] {
        let rows: i64 = db.query_row(&format!("SELECT count(*) FROM {table}"), [], |r| r.get(0))?;
        summary.rows.insert(table, rows);
    }
    drop(db);
    std::fs::rename(&partial, path)?;
    Ok(summary)
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

/// Parses one unit as the census does and gathers the rows of the part of
/// its forest some Derivation uses.
fn unit_rows(
    corpus: &Corpus,
    unit: usize,
    cleave: Option<&sylex_english::cleave::CleaveText>,
    env: &LexicalEnvironment,
    grammar: &CompiledGrammar,
    reader: &context::Reader<'_>,
) -> Result<UnitRows> {
    let u = &corpus.units[unit];
    let face = &corpus.faces[u.face];
    let parsed = cleave.map_or_else(|| u.text.clone(), |c| c.uncleaved.clone());
    let analyzed = env
        .lexicon
        .analyze_source_scoped(&parsed, None, &face.scope());
    let parse = grammar.parse_analyzed(&analyzed);
    let counts = parse.counts()?;
    let heads = parse.alternative_heads()?;
    let conjuncts = parse.alternative_conjuncts()?;
    let embeddings = parse.alternative_embeddings(context::DEPTH, |step| reader.wrapper(step))?;
    exclusive(&parse, &counts)?;
    let forest = &parse.forest;
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
    let mut nodes = Vec::new();
    for node in (0..forest.node_count()).filter(|&n| counts.node(n) > 0) {
        let span = forest.node_span(node);
        let coordinator = parse
            .node_coordinator(node)
            .map(|id| {
                if env.lexicon.lexemes().contains_key(&id) {
                    Ok(Coordinator::Lexeme(id))
                } else if grammar.category_id(id.as_str()).is_some() {
                    Ok(Coordinator::Link(id))
                } else {
                    Err(anyhow::anyhow!(
                        "coordinator `{id}` is neither a lexeme nor a Category"
                    ))
                }
            })
            .transpose()?;
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
                    LexicalReading::FlavorWord { .. } | LexicalReading::Numeral { .. } => None,
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
                            Ok((embedding.clone(), ways, product(ways, inside)?))
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
        .collect();
    Ok(UnitRows {
        derivations: counts.total(),
        parsed,
        nodes,
        items,
        leaves,
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
    let forest = &parse.forest;
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
    let forest = &parse.forest;
    let grammar = parse.grammar();
    let mut items = vec![item];
    let mut seen_items = BTreeSet::new();
    let mut nodes = BTreeSet::new();
    while let Some(item) = items.pop() {
        if !seen_items.insert(item) {
            continue;
        }
        let function = grammar.production_functions(forest.item_production(item))
            [forest.item_dot(item) - 1]
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

fn write_grammar(tx: &rusqlite::Transaction<'_>, grammar: &CompiledGrammar) -> Result<()> {
    let mut category = tx.prepare("INSERT INTO categories VALUES (?1, ?2)")?;
    for (index, name) in grammar.category_names().iter().enumerate() {
        category.execute(params![self::index(index), name.as_str()])?;
    }
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
    Ok(())
}

fn write_unit(
    tx: &rusqlite::Transaction<'_>,
    corpus: &Corpus,
    unit: usize,
    rows: &UnitRows,
    lexemes: &mut BTreeSet<LexemeId>,
    embeddings: &mut BTreeMap<Embedding, i64>,
    summary: &mut Summary,
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
    tx.prepare_cached("INSERT INTO units VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)")?
        .execute(params![
            u_id,
            oracle_id,
            face_index,
            face.name,
            face.card,
            match u.kind {
                UnitKind::TypeLine => "type line",
                UnitKind::Line => "line",
            },
            index(u.index),
            u.text,
            rows.parsed,
            int(rows.derivations)?,
        ])?;
    let mut node_insert = tx.prepare_cached(
        "INSERT INTO nodes VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
    )?;
    let mut alternative_insert =
        tx.prepare_cached("INSERT INTO alternatives VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?;
    let mut head_insert =
        tx.prepare_cached("INSERT INTO heads VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")?;
    let mut embedding_insert =
        tx.prepare_cached("INSERT INTO alternative_embeddings VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?;
    let mut conjunct_insert =
        tx.prepare_cached("INSERT INTO conjuncts VALUES (?1, ?2, ?3, ?4, ?5)")?;
    for node in &rows.nodes {
        let (coordinator, link) = match node.coordinator {
            Some(Coordinator::Lexeme(id)) => {
                lexemes.insert(id);
                (Some(id.as_str()), None)
            }
            Some(Coordinator::Link(id)) => (None, Some(id.as_str())),
            None => (None, None),
        };
        node_insert.execute(params![
            u_id,
            index(node.node),
            index(node.category),
            index(node.span.start),
            index(node.span.end),
            node.surface,
            node.gap,
            node.features,
            coordinator,
            link,
            node.root,
            node.settled,
            int(node.derivations)?,
        ])?;
        for (a, alternative) in node.alternatives.iter().enumerate() {
            let key = (u_id, index(node.node), index(a));
            alternative_insert.execute(params![
                key.0,
                key.1,
                key.2,
                index(alternative.production),
                index(alternative.item),
                int(alternative.derivations)?,
            ])?;
            for (leaf, word, inside, derivations) in &alternative.heads {
                if let Some((lexeme, _)) = word {
                    lexemes.insert(*lexeme);
                }
                head_insert.execute(params![
                    key.0,
                    key.1,
                    key.2,
                    leaf.map(index),
                    word.map(|(lexeme, _)| lexeme.as_str()),
                    word.map(|(_, form)| format!("{form:?}")),
                    int(*inside)?,
                    int(*derivations)?,
                ])?;
            }
            for (embedding, outside, derivations) in &alternative.embeddings {
                let next = index(embeddings.len());
                let id = *embeddings.entry(embedding.clone()).or_insert(next);
                embedding_insert.execute(params![
                    key.0,
                    key.1,
                    key.2,
                    id,
                    int(*outside)?,
                    int(*derivations)?,
                ])?;
            }
            for (conjuncts, derivations) in &alternative.conjuncts {
                conjunct_insert.execute(params![
                    key.0,
                    key.1,
                    key.2,
                    index(*conjuncts),
                    int(*derivations)?,
                ])?;
            }
        }
    }
    let families = write_items(tx, u_id, &rows.items)?;
    write_leaves(tx, u_id, &rows.leaves, lexemes)?;
    summary.units += 1;
    let size = (rows.nodes.len() + rows.items.len()) as u64 + families;
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

/// Writes a unit's items and their families; returns the families written.
fn write_items(tx: &rusqlite::Transaction<'_>, u_id: i64, items: &[ItemRow]) -> Result<u64> {
    let mut item_insert =
        tx.prepare_cached("INSERT INTO items VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?;
    let mut family_insert =
        tx.prepare_cached("INSERT INTO families VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?;
    let mut families = 0u64;
    for item in items {
        item_insert.execute(params![
            u_id,
            index(item.item),
            index(item.production),
            index(item.dot),
            index(item.span.start),
            index(item.span.end),
            int(item.derivations)?,
        ])?;
        for (f, (previous, child, derivations)) in item.families.iter().enumerate() {
            let (node, leaf) = match child {
                ChildRef::Node(n) => (Some(index(*n)), None),
                ChildRef::Leaf(l) => (None, Some(index(*l))),
                ChildRef::Punct => (None, None),
            };
            family_insert.execute(params![
                u_id,
                index(item.item),
                index(f),
                previous.map(index),
                node,
                leaf,
                int(*derivations)?,
            ])?;
            families += 1;
        }
    }
    Ok(families)
}

fn write_leaves(
    tx: &rusqlite::Transaction<'_>,
    u_id: i64,
    leaves: &[LeafRow],
    lexemes: &mut BTreeSet<LexemeId>,
) -> Result<()> {
    let mut leaf_insert =
        tx.prepare_cached("INSERT INTO leaves VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)")?;
    for leaf in leaves {
        let (kind, lexeme, form, value) = match &leaf.reading {
            LexicalReading::Word(word) => {
                lexemes.insert(word.lexeme);
                (
                    "word",
                    Some(word.lexeme.as_str()),
                    Some(format!("{:?}", word.form)),
                    None,
                )
            }
            LexicalReading::FlavorWord { .. } => ("flavor word", None, None, None),
            LexicalReading::Numeral { value, .. } => ("numeral", None, None, Some(*value)),
        };
        leaf_insert.execute(params![
            u_id,
            index(leaf.leaf),
            index(leaf.span.start),
            index(leaf.span.end),
            leaf.surface,
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
    embeddings: &BTreeMap<Embedding, i64>,
    reader: &context::Reader<'_>,
    lexemes: &mut BTreeSet<LexemeId>,
) -> Result<()> {
    let mut embedding_insert = tx.prepare("INSERT INTO embeddings VALUES (?1, ?2, ?3, ?4)")?;
    let mut step_insert =
        tx.prepare("INSERT INTO embedding_steps VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)")?;
    for (embedding, &id) in embeddings {
        embedding_insert.execute(params![
            id,
            index(embedding.steps.len()),
            embedding.truncated,
            reader.context(embedding).name(),
        ])?;
        for (n, step) in embedding.steps.iter().enumerate() {
            if let Some((lexeme, _)) = step.governor_head {
                lexemes.insert(lexeme);
            }
            step_insert.execute(params![
                id,
                index(n + 1),
                index(step.projection),
                index(step.child),
                step.function.as_str(),
                index(step.governor),
                step.governor_head.map(|(lexeme, _)| lexeme.as_str()),
                step.governor_head.map(|(_, form)| format!("{form:?}")),
                reader.wrapper(step),
            ])?;
        }
    }
    Ok(())
}

fn write_lexemes(
    tx: &rusqlite::Transaction<'_>,
    lexemes: &BTreeSet<LexemeId>,
    env: &LexicalEnvironment,
) -> Result<()> {
    let mut insert = tx.prepare("INSERT INTO lexemes VALUES (?1, ?2, ?3)")?;
    for id in lexemes {
        let lexeme = env
            .lexicon
            .lexemes()
            .get(id)
            .with_context(|| format!("lexeme `{id}` is not in the lexicon"))?;
        insert.execute(params![
            id.as_str(),
            lexeme.lemma,
            format!("{:?}", lexeme.category)
        ])?;
    }
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
    fn database(cards: &[&str]) -> (tempfile::TempDir, PathBuf, Corpus) {
        let env = LexicalEnvironment::load().unwrap();
        let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
        let corpus = Corpus::load(|face| cards.contains(&face.card.as_str())).unwrap();
        assert!(!corpus.units.is_empty(), "no unit of {cards:?}");
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("forests.sqlite");
        write(&path, &corpus, &env, &grammar).unwrap();
        (dir, path, corpus)
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
            let expected = grammar.parse_analyzed(&analyzed).derivation_count();
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

    /// *Kykar, Zephyr Awakener*'s *Whenever you cast a noncreature spell*
    /// has two Derivations, *cast* Present in one and Preterite in the
    /// other: two nodes over one span, each in one Derivation. Summed by
    /// span and Production, *cast* is in both.
    #[test]
    fn nodes_sharing_a_span_and_production_sum_to_every_derivation() {
        let (_dir, path, _) = database(&["Kykar, Zephyr Awakener"]);
        let db = Connection::open(&path).unwrap();
        let per_node = column(
            &db,
            "SELECT sum(v.derivations) FROM verb_phrases v JOIN units u USING (unit) WHERE \
             u.line = 1 AND u.kind = 'line' AND v.lemma = 'cast' GROUP BY v.node",
        );
        assert_eq!(per_node, [1, 1]);
        let spans = column(
            &db,
            "SELECT s.every FROM verb_phrase_spans s JOIN units u USING (unit) WHERE u.line = 1 \
             AND u.kind = 'line' AND s.lemma = 'cast' AND s.context = 'trigger clause' AND \
             s.derivations = u.derivations",
        );
        assert_eq!(spans, [1]);
        let tokens = column(
            &db,
            "SELECT t.every FROM verb_phrase_tokens t JOIN units u USING (unit) WHERE u.line = 1 \
             AND u.kind = 'line' AND t.lemma = 'cast'",
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
            (
                "Roil's Retribution",
                "block",
                "blocking creatures",
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

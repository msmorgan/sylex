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

/// The stored tables. Every `derivations` column is the exact number of the
/// unit's Derivations that use the row's node, alternative, item, family,
/// head or Embedding (`sylex_syntax::Counts`).
pub(crate) const SCHEMA: &str = r"
-- The grammar's Categories and Productions. `functions` is a JSON array of
-- each child's name, punctuation included (an empty name); `frame` and
-- `variant` are a frame-headed Production's Selected Frame and what its id
-- adds after the frame's name: its Valence Alternation and derived shape,
-- `/`-separated (`passive`, `passive+by`, `adjectival`, `gap0`,
-- `shifted-object`, `clusters`, `prefix1`, `interleaved2`, ...; empty for the
-- frame's own phrase). `tail`: it builds a serial or
-- polysyndetic tail of a coordination, not a whole one.
CREATE TABLE categories (category INTEGER PRIMARY KEY, name TEXT NOT NULL);
CREATE TABLE productions (
    production INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    category INTEGER NOT NULL REFERENCES categories,
    functions TEXT NOT NULL,
    frame TEXT,
    variant TEXT,
    tail INTEGER NOT NULL
);
-- Every lexeme a stored row names, with its lemma and part of speech.
CREATE TABLE lexemes (lexeme TEXT PRIMARY KEY, lemma TEXT NOT NULL, part_of_speech TEXT NOT NULL);
-- One row per Corpus Unit. `oracle_id` joins mtgjson's
-- `cardIdentifiers.scryfallOracleId`; `face` is the Scryfall `card_faces`
-- index, NULL for a one-face card. `parsed` is the text parsed: the unit's
-- text, or a cleave line's Uncleaved Text. `derivations` is its total.
CREATE TABLE units (
    unit INTEGER PRIMARY KEY,
    oracle_id TEXT NOT NULL,
    face INTEGER,
    face_name TEXT NOT NULL,
    card_name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('type line', 'line')),
    line INTEGER NOT NULL,
    text TEXT NOT NULL,
    parsed TEXT NOT NULL,
    derivations INTEGER NOT NULL
);
-- Completed Constituents, by the forest's node index. `start` and `end` are
-- char offsets into `parsed` in logical order (a period written inside a
-- closing quotation mark follows it), `start` where the Constituent was
-- predicted, before any space that precedes its first word; `surface` is
-- that text without the space. `features` is
-- a JSON object of bound features; `gap` NULL is no Gap. A coordination has
-- its `coordinator` lexeme, or, for a Linked Coordination (which has none),
-- the `link` Category its linked conjunct is built as. `settled`: used by
-- every Derivation. `root`: spans the whole unit as the grammar's root.
CREATE TABLE nodes (
    unit INTEGER NOT NULL REFERENCES units,
    node INTEGER NOT NULL,
    category INTEGER NOT NULL REFERENCES categories,
    start INTEGER NOT NULL,
    end INTEGER NOT NULL,
    surface TEXT NOT NULL,
    gap TEXT,
    features TEXT NOT NULL,
    coordinator TEXT REFERENCES lexemes,
    link TEXT,
    root INTEGER NOT NULL,
    settled INTEGER NOT NULL,
    derivations INTEGER NOT NULL,
    PRIMARY KEY (unit, node)
) WITHOUT ROWID;
-- A node's alternatives: each a Production with the completed item that is
-- its children, in the forest's order.
CREATE TABLE alternatives (
    unit INTEGER NOT NULL,
    node INTEGER NOT NULL,
    alternative INTEGER NOT NULL,
    production INTEGER NOT NULL REFERENCES productions,
    item INTEGER NOT NULL,
    derivations INTEGER NOT NULL,
    PRIMARY KEY (unit, node, alternative)
) WITHOUT ROWID;
-- Partial Productions: `dot` children found, spanning `start`..`end`.
CREATE TABLE items (
    unit INTEGER NOT NULL,
    item INTEGER NOT NULL,
    production INTEGER NOT NULL REFERENCES productions,
    dot INTEGER NOT NULL,
    start INTEGER NOT NULL,
    end INTEGER NOT NULL,
    derivations INTEGER NOT NULL,
    PRIMARY KEY (unit, item)
) WITHOUT ROWID;
-- An item's families: its previous item (NULL for its first child) and its
-- last child, a node, a leaf, or punctuation (both NULL).
CREATE TABLE families (
    unit INTEGER NOT NULL,
    item INTEGER NOT NULL,
    family INTEGER NOT NULL,
    previous INTEGER,
    child_node INTEGER,
    child_leaf INTEGER,
    derivations INTEGER NOT NULL,
    PRIMARY KEY (unit, item, family)
) WITHOUT ROWID;
-- The lexical alternatives some family takes. `kind` is `word` (with its
-- lexeme and Word Form), `flavor word` or `numeral` (with `value`).
CREATE TABLE leaves (
    unit INTEGER NOT NULL,
    leaf INTEGER NOT NULL,
    start INTEGER NOT NULL,
    end INTEGER NOT NULL,
    surface TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('word', 'flavor word', 'numeral')),
    lexeme TEXT REFERENCES lexemes,
    form TEXT,
    value INTEGER,
    PRIMARY KEY (unit, leaf)
) WITHOUT ROWID;
-- Each alternative's head words: the leaf reached through `head` children
-- (lexeme NULL where that is no word, or there is no `head` child). One
-- alternative's items may reach several. `inside` is the alternative's own
-- Derivations with that head; `derivations` the unit's.
CREATE TABLE heads (
    unit INTEGER NOT NULL,
    node INTEGER NOT NULL,
    alternative INTEGER NOT NULL,
    lexeme TEXT REFERENCES lexemes,
    form TEXT,
    inside INTEGER NOT NULL,
    derivations INTEGER NOT NULL,
    UNIQUE (unit, node, alternative, lexeme, form)
);
-- Embeddings: the dependent functions a Constituent stands under, nearest
-- first, at most three. Going up from it through `head` children to the top
-- of its head line, step N records the Production there (`projectionN`),
-- the function that bears in its governor (`functionN`), the governor's
-- Production and its head word. `depth` is the number of steps; a root's own
-- head line has none.
CREATE TABLE embeddings (
    embedding INTEGER PRIMARY KEY,
    depth INTEGER NOT NULL,
    projection1 INTEGER REFERENCES productions, function1 TEXT,
    governor1 INTEGER REFERENCES productions, governor_lexeme1 TEXT REFERENCES lexemes,
    governor_form1 TEXT,
    projection2 INTEGER REFERENCES productions, function2 TEXT,
    governor2 INTEGER REFERENCES productions, governor_lexeme2 TEXT REFERENCES lexemes,
    governor_form2 TEXT,
    projection3 INTEGER REFERENCES productions, function3 TEXT,
    governor3 INTEGER REFERENCES productions, governor_lexeme3 TEXT REFERENCES lexemes,
    governor_form3 TEXT
);
-- Each alternative's Embeddings. `outside` is the ways to surround its node
-- with that Embedding; `derivations` the unit's Derivations that use the
-- alternative under it. An alternative's head and Embedding are independent,
-- so the Derivations with a given head and Embedding are
-- `heads.inside * alternative_embeddings.outside`.
CREATE TABLE alternative_embeddings (
    unit INTEGER NOT NULL,
    node INTEGER NOT NULL,
    alternative INTEGER NOT NULL,
    embedding INTEGER NOT NULL REFERENCES embeddings,
    outside INTEGER NOT NULL,
    derivations INTEGER NOT NULL,
    PRIMARY KEY (unit, node, alternative, embedding)
) WITHOUT ROWID;
-- For each alternative of a coordination node (one with a coordinator or a
-- link) whose Production is no tail, its conjuncts: its children named
-- `first` or `last`, its idiomatic `coordinate` (*and so on*), and those of
-- the tails it continues in through `rest`.
CREATE TABLE conjuncts (
    unit INTEGER NOT NULL,
    node INTEGER NOT NULL,
    alternative INTEGER NOT NULL,
    conjuncts INTEGER NOT NULL,
    derivations INTEGER NOT NULL,
    PRIMARY KEY (unit, node, alternative, conjuncts)
) WITHOUT ROWID;
";

/// Views for the questions already known. Each keeps a `derivations`
/// column, so "in every Derivation" is `derivations = units.derivations` and
/// "in some Derivation" is any row.
pub(crate) const VIEWS: &str = r"
-- Embeddings with Production names and governor lemmas.
CREATE VIEW named_embeddings AS
SELECT e.embedding, e.depth,
    p1.name AS projection1, e.function1, g1.name AS governor1, l1.lemma AS governor_lemma1,
    e.governor_form1,
    p2.name AS projection2, e.function2, g2.name AS governor2, l2.lemma AS governor_lemma2,
    e.governor_form2,
    p3.name AS projection3, e.function3, g3.name AS governor3, l3.lemma AS governor_lemma3,
    e.governor_form3
FROM embeddings e
LEFT JOIN productions p1 ON p1.production = e.projection1
LEFT JOIN productions g1 ON g1.production = e.governor1
LEFT JOIN lexemes l1 ON l1.lexeme = e.governor_lexeme1
LEFT JOIN productions p2 ON p2.production = e.projection2
LEFT JOIN productions g2 ON g2.production = e.governor2
LEFT JOIN lexemes l2 ON l2.lexeme = e.governor_lexeme2
LEFT JOIN productions p3 ON p3.production = e.projection3
LEFT JOIN productions g3 ON g3.production = e.governor3
LEFT JOIN lexemes l3 ON l3.lexeme = e.governor_lexeme3;

-- Every verb phrase: each alternative of a frame-headed Production whose head
-- word is a verb, once per (head word, Embedding) it has, with its Selected
-- Frame, Valence Alternation variant, Voice and the Embedding it stands
-- under. `derivations` is exact: the unit's Derivations that use this
-- alternative with this head and Embedding. Summing it over rows of one node
-- counts that node's Derivations with those facts; never sum it across
-- different nodes of one unit as if they were exclusive Derivations, since
-- one Derivation uses many nodes.
--
-- The Embedding tells apart where a verb phrase stands (`english.fug`):
-- - an effect clause: step 1 `function1 = 'clause'` under `Declarative`, or
--   its Clause the `head` of an adjunct Production under it (then step 2);
-- - a trigger or condition clause: a `FiniteComplementClause` projection,
--   `complement` of `Valence:PrepositionPhrase/Clause(Finite)@Predicate`
--   whose head is *when*, *whenever*, *if*, ...;
-- - a replacement's *would* clause: a `BareInfinitival` projection,
--   `complement` of `Valence:VerbPhrase/Catenative(BareInf)` headed by
--   `will` in its Preterite form, with step 2 the *if* clause above;
-- - a deontic: the same with a modal head such as *can't*, *must* or *may*;
-- - a relative clause: function `clause` of a `RelativeClause` Production
--   such as `SubjectRelative`;
-- - a participial postmodifier: a `ParticipialPostmodifier` projection,
--   `modifier` of a Nominal.
CREATE VIEW verb_phrases AS
SELECT a.unit, a.node, a.alternative, n.surface, c.name AS category, p.name AS production,
    h.lexeme AS verb, l.lemma, h.form, p.frame, p.variant,
    json_extract(n.features, '$.Voice') AS voice, n.settled,
    e.embedding, e.depth,
    e.projection1, e.function1, e.governor1, e.governor_lemma1, e.governor_form1,
    e.projection2, e.function2, e.governor2, e.governor_lemma2, e.governor_form2,
    e.projection3, e.function3, e.governor3, e.governor_lemma3, e.governor_form3,
    h.inside * ae.outside AS derivations
FROM alternatives a
JOIN productions p ON p.production = a.production AND p.frame IS NOT NULL
JOIN nodes n ON n.unit = a.unit AND n.node = a.node
JOIN categories c ON c.category = n.category
JOIN heads h ON h.unit = a.unit AND h.node = a.node AND h.alternative = a.alternative
JOIN lexemes l ON l.lexeme = h.lexeme AND l.part_of_speech = 'Verb'
JOIN alternative_embeddings ae
    ON ae.unit = a.unit AND ae.node = a.node AND ae.alternative = a.alternative
JOIN named_embeddings e ON e.embedding = ae.embedding;

-- Every coordination node: each alternative with its conjunct count, its
-- Category and its coordinator (NULL for a Linked Coordination, whose `link`
-- is given instead). `derivations` is the unit's Derivations that use
-- this alternative with that many conjuncts; the same caution as for
-- `verb_phrases` applies to sums across nodes.
CREATE VIEW coordinations AS
SELECT j.unit, j.node, j.alternative, n.surface, c.name AS category, p.name AS production,
    n.coordinator, l.lemma AS coordinator_lemma, n.link, j.conjuncts, n.settled,
    j.derivations
FROM conjuncts j
JOIN nodes n ON n.unit = j.unit AND n.node = j.node
JOIN categories c ON c.category = n.category
JOIN alternatives a ON a.unit = j.unit AND a.node = j.node AND a.alternative = j.alternative
JOIN productions p ON p.production = a.production
LEFT JOIN lexemes l ON l.lexeme = n.coordinator;
";

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
    heads: Vec<(Option<HeadWord>, u128, u128)>,
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
            .map(|unit| unit_rows(corpus, unit, cleaves[unit].as_ref(), env, grammar))
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
    write_embeddings(&tx, &embeddings, &mut lexemes)?;
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
) -> Result<UnitRows> {
    let u = &corpus.units[unit];
    let face = &corpus.faces[u.face];
    let parsed = cleave.map_or_else(|| u.text.clone(), |c| c.uncleaved.clone());
    let analyzed = env
        .lexicon
        .analyze_source_scoped(&parsed, None, &face.scope());
    let parse = grammar.parse_analyzed(&analyzed);
    let counts = parse.counts().with_context(|| describe(corpus, unit))?;
    let heads = parse.alternative_heads()?;
    let conjuncts = parse.alternative_conjuncts()?;
    let embeddings = parse.alternative_embeddings()?;
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
                AlternativeRow {
                    production,
                    item,
                    derivations: counts.node_family(node, index),
                    heads: heads[node][index]
                        .iter()
                        .map(|(&head, &own)| (head, own, own * outside))
                        .collect(),
                    embeddings: embeddings[node][index]
                        .iter()
                        .map(|(embedding, &ways)| (embedding.clone(), ways, ways * inside))
                        .collect(),
                    conjuncts: match coordinator {
                        Some(_) if !grammar.production_is_tail(production) => conjuncts[node]
                            [index]
                            .iter()
                            .map(|(&n, &own)| (n, own * outside))
                            .collect(),
                        Some(_) | None => Vec::new(),
                    },
                }
            })
            .collect();
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
        tx.prepare("INSERT INTO productions VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?;
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
        tx.prepare_cached("INSERT INTO heads VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?;
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
            for (head, inside, derivations) in &alternative.heads {
                if let Some((lexeme, _)) = head {
                    lexemes.insert(*lexeme);
                }
                head_insert.execute(params![
                    key.0,
                    key.1,
                    key.2,
                    head.map(|(lexeme, _)| lexeme.as_str()),
                    head.map(|(_, form)| format!("{form:?}")),
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
    lexemes: &mut BTreeSet<LexemeId>,
) -> Result<()> {
    let mut insert = tx.prepare(
        "INSERT INTO embeddings VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, \
         ?14, ?15, ?16, ?17)",
    )?;
    for (embedding, &id) in embeddings {
        let mut values: Vec<rusqlite::types::Value> =
            vec![id.into(), index(embedding.len()).into()];
        for depth in 0..sylex_syntax::EMBEDDING_DEPTH {
            match embedding.get(depth) {
                Some(step) => {
                    if let Some((lexeme, _)) = step.governor_head {
                        lexemes.insert(lexeme);
                    }
                    values.extend([
                        index(step.projection).into(),
                        step.function.as_str().to_owned().into(),
                        index(step.governor).into(),
                        step.governor_head
                            .map(|(lexeme, _)| lexeme.as_str().to_owned())
                            .into(),
                        step.governor_head
                            .map(|(_, form)| format!("{form:?}"))
                            .into(),
                    ]);
                }
                None => values.extend((0..5).map(|_| rusqlite::types::Value::Null)),
            }
        }
        insert.execute(rusqlite::params_from_iter(values))?;
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
                "an alternative's heads",
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
    /// phrase of *exile* is in one, as the `clause` of a `Declarative`.
    #[test]
    fn crook_of_condemnation_s_verb_phrases_split_its_attachment() {
        let (_dir, path, _) = database(&["Crook of Condemnation"]);
        let db = Connection::open(&path).unwrap();
        let mut statement = db
            .prepare(
                "SELECT v.surface, v.lemma, v.frame, v.voice, v.projection1, v.function1, \
                 v.governor1, v.derivations, u.derivations FROM verb_phrases v JOIN units u \
                 USING (unit) WHERE u.line = 0 AND u.kind = 'line' ORDER BY v.surface",
            )
            .unwrap();
        let rows: Vec<String> = statement
            .query_map([], |r| {
                Ok((0..9)
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
                     Text(\"Active\") Text(\"Imperative\") Text(\"clause\") \
                     Text(\"Declarative\") Integer(1) Integer(2)"
                )),
                "{row}"
            );
        }
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

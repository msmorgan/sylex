//! The database's tables and views, each described in its SQL.

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
-- `interleaved_prefix`: it builds the part of a frame's phrase before an
-- interleaved dependent (`/prefixN`), which only its `/interleaved`
-- Production takes, as its head; it is no whole phrase.
CREATE TABLE categories (category INTEGER PRIMARY KEY, name TEXT NOT NULL);
CREATE TABLE productions (
    production INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    category INTEGER NOT NULL REFERENCES categories,
    functions TEXT NOT NULL,
    frame TEXT,
    variant TEXT,
    tail INTEGER NOT NULL,
    interleaved_prefix INTEGER NOT NULL
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
-- Each alternative's head leaves: the leaf reached through `head` children,
-- with its lexeme and Word Form when it is a word (`leaf` NULL where a
-- `head` child is punctuation, or there is none). One alternative's items
-- may reach several (*cast* as Present or Preterite). `inside` is the
-- alternative's own Derivations with that head; `derivations` the unit's.
CREATE TABLE heads (
    unit INTEGER NOT NULL,
    node INTEGER NOT NULL,
    alternative INTEGER NOT NULL,
    leaf INTEGER,
    lexeme TEXT REFERENCES lexemes,
    form TEXT,
    inside INTEGER NOT NULL,
    derivations INTEGER NOT NULL,
    UNIQUE (unit, node, alternative, leaf)
);
-- Embeddings: the dependent functions a Constituent stands under, nearest
-- first. Going up from it through `head` children to the top of its head
-- line, a step records the Production there (`projection`), its position
-- and function in its governor, the governor's Production and its head
-- word. A wrapper step (`wrapper`: an Auxiliary Complement, a part of a
-- coordination, do-support's Complement, an adjectival passive under *be*)
-- is kept but not counted; the steps stop at the third that is no wrapper,
-- and `truncated` says steps above were cut. `context` is the context a verb
-- phrase under the Embedding stands in, read from its first step that is no
-- wrapper: `effect clause`, `trigger clause`, `would clause`, `deontic`,
-- `relative clause`, `participial postmodifier` or `other`
-- (`crates/xtask/src/english/forests/context.rs` states each). A root's own
-- head line has no step.
CREATE TABLE embeddings (
    embedding INTEGER PRIMARY KEY,
    steps INTEGER NOT NULL,
    truncated INTEGER NOT NULL,
    context TEXT NOT NULL
);
CREATE TABLE embedding_steps (
    embedding INTEGER NOT NULL REFERENCES embeddings,
    step INTEGER NOT NULL,
    projection INTEGER NOT NULL REFERENCES productions,
    child INTEGER NOT NULL,
    function TEXT NOT NULL,
    governor INTEGER NOT NULL REFERENCES productions,
    governor_lexeme TEXT REFERENCES lexemes,
    governor_form TEXT,
    wrapper INTEGER NOT NULL,
    PRIMARY KEY (embedding, step)
) WITHOUT ROWID;
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
/// column; how to sum it is said at `verb_phrases`.
pub(crate) const VIEWS: &str = r"
-- Each Embedding's steps with Production names and governor lemmas.
CREATE VIEW named_embedding_steps AS
SELECT s.embedding, s.step, p.name AS projection, s.function, g.name AS governor,
    l.lemma AS governor_lemma, s.governor_form, s.wrapper
FROM embedding_steps s
JOIN productions p ON p.production = s.projection
JOIN productions g ON g.production = s.governor
LEFT JOIN lexemes l ON l.lexeme = s.governor_lexeme;

-- Every verb phrase: each alternative of a frame-headed Production whose head
-- is a verb (an interleaved prefix is part of its `/interleaved` one), once per (head, Embedding) it has, with its Selected Frame,
-- variant, Voice, context and Embedding (`embedding_steps`), and its head
-- token (`head_start`, the head leaf's start). `derivations` is exact: the
-- unit's Derivations that use this alternative with this head and Embedding.
--
-- A fact holds in some Derivation where it has a row, and in every
-- Derivation where the Derivations that have it equal `units.derivations`.
-- Those are a sum of rows, exact only over rows no two of which one
-- Derivation can hold:
-- - rows of one node: its alternatives, heads and Embeddings partition its
--   Derivations;
-- - rows of nodes that share a span and a Production: the chart packs a
--   Constituent by its bound features too, so one verb phrase over one span
--   is often several nodes (*cast* Present in one, Preterite in another),
--   and two of them could share one Derivation only nested over the same
--   span, which the writer checks never happens (`verb_phrase_spans` sums
--   them);
-- - rows sharing a head token: no Derivation holds two of these verb phrase
--   alternatives on one head leaf, which the writer checks
--   (`verb_phrase_tokens` sums them). The one case of two frame-headed
--   alternatives on a head leaf is an interleaved prefix under its
--   `/interleaved` Production, which this view leaves out.
-- Never sum rows of different spans otherwise: one Derivation uses many
-- nodes.
CREATE VIEW verb_phrases AS
SELECT a.unit, a.node, a.alternative, n.start, n.end, n.surface, c.name AS category,
    p.name AS production, h.leaf AS head_leaf, hl.start AS head_start, h.lexeme AS verb, l.lemma,
    h.form, p.frame, p.variant, json_extract(n.features, '$.Voice') AS voice, n.settled,
    e.embedding, e.context, e.truncated,
    h.inside * ae.outside AS derivations
FROM alternatives a
JOIN productions p
    ON p.production = a.production AND p.frame IS NOT NULL AND NOT p.interleaved_prefix
JOIN nodes n ON n.unit = a.unit AND n.node = a.node
JOIN categories c ON c.category = n.category
JOIN heads h ON h.unit = a.unit AND h.node = a.node AND h.alternative = a.alternative
JOIN leaves hl ON hl.unit = h.unit AND hl.leaf = h.leaf
JOIN lexemes l ON l.lexeme = h.lexeme AND l.part_of_speech = 'Verb'
JOIN alternative_embeddings ae
    ON ae.unit = a.unit AND ae.node = a.node AND ae.alternative = a.alternative
JOIN embeddings e ON e.embedding = ae.embedding;

-- Verb phrases summed over the nodes that share a span and a Production
-- (see `verb_phrases`), by verb, frame, variant, Voice and context. `every`:
-- every Derivation of the unit has such a verb phrase over this span.
CREATE VIEW verb_phrase_spans AS
SELECT v.unit, v.start, v.end, v.surface, v.production, v.lemma, v.frame, v.variant,
    v.voice, v.context, sum(v.derivations) AS derivations,
    sum(v.derivations) = u.derivations AS every
FROM verb_phrases v JOIN units u USING (unit)
GROUP BY v.unit, v.start, v.end, v.production, v.lemma, v.frame, v.variant, v.voice, v.context;

-- Verb phrases summed over the frame-headed alternatives that share a head
-- token (see `verb_phrases`), by verb, frame, variant, Voice and context.
-- `every`: every Derivation of the unit has such a verb phrase headed by this
-- token, whatever its span.
CREATE VIEW verb_phrase_tokens AS
SELECT v.unit, v.head_start, v.lemma, v.frame, v.variant, v.voice, v.context,
    sum(v.derivations) AS derivations, sum(v.derivations) = u.derivations AS every
FROM verb_phrases v JOIN units u USING (unit)
GROUP BY v.unit, v.head_start, v.lemma, v.frame, v.variant, v.voice, v.context;

-- Every coordination node: each alternative with its conjunct count, its
-- Category and its coordinator (NULL for a Linked Coordination, whose `link`
-- is given instead). `derivations` is the unit's Derivations that use
-- this alternative with that many conjuncts; sum it as `verb_phrases` says.
CREATE VIEW coordinations AS
SELECT j.unit, j.node, j.alternative, n.start, n.end, n.surface, c.name AS category,
    p.name AS production, n.coordinator, l.lemma AS coordinator_lemma, n.link, j.conjuncts,
    n.settled, j.derivations
FROM conjuncts j
JOIN nodes n ON n.unit = j.unit AND n.node = j.node
JOIN categories c ON c.category = n.category
JOIN alternatives a ON a.unit = j.unit AND a.node = j.node AND a.alternative = j.alternative
JOIN productions p ON p.production = a.production
LEFT JOIN lexemes l ON l.lexeme = n.coordinator;
";

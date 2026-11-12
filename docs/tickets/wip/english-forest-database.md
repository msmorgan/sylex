---
needs: [english-forest-counts-and-forks]
---

# The pool's Packed Forests in one SQLite database

## Why

A question across the whole pool ("which verbs take this frame in the passive") needs a parse
of every unit, about 50 s in the dev profile for `english strata`, and then code written for that
one question. With every unit's Packed Forest and its exact counts
(`english-forest-counts-and-forks`) in one database, such a question is a SQL query, and its
answer can be joined to mtgjson's `cards` and `cardLegalities`.
The forest stays packed (`docs/decisions/english.md` §4): there is never a row per Reading.

## Design

- A command writes one SQLite file for the census corpus. Tables:
  - units: the face's Oracle id and face name (which join to `cardIdentifiers.scryfallOracleId`
    and `cards` in `data/mtgjson/AllPrintings.sqlite`, and from there to `cardLegalities`), the
    unit's kind and index, its text and its Derivation count;
  - nodes: Category, span, surface, Gap, features, exact count and the settled flag;
  - alternatives and edges: each node alternative with its Production, head lemma and form and
    Selected Frame (which belong to the alternative, not the node), and the edges with their
    children, exact counts and the alternative each belongs to.
- A named view of every verb phrase with its verb, Selected Frame and Voice and the node it
  stands under.
- The command has no query features: questions are SQL.
- The file lives under `data/derived/`, which is already untracked (`/data/` is in
  `.gitignore`).

## Open questions

- How edges are stored. The forest is binarized: a node's alternative is a (Production, item)
  pair and each item a chain of (previous item, child) families. Storing the item chains is
  exact and stays packed; flattening each alternative to its list of children multiplies where
  an item's prefixes branch.
- Whether a unit needs a cap, and what happens to it: answered by the measurement below.

## Done when

- Before the schema is fixed, the landing reports for the pool: total nodes and edges,
  the largest unit by each, the file size and the run time; and says, from those numbers,
  whether any unit needs a cap.
- The database is written for every unit of the census corpus, and a test checks on real units
  that each unit's stored counts equal what `english-forest-counts-and-forks` computes.
- The named view is exercised by a query in the landing, with its row count.
- The landing reports how many census faces have no Vintage `Legal` or `Restricted` printing in
  mtgjson by the join (the census corpus is chosen by Scryfall's legality, `OracleCard::supported`).
- `jj --no-pager status` shows no database file, and `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Not in scope

- A row per Reading, and any query interface beyond SQL views.
- Forks and the views over them: `english-forest-database-forks`.
- The per-unit dump of a sliced forest: an open question of `english-forestwalk-narrowing`.

## Pointers

- `crates/syntax/src/chart.rs` (`PackedForest`, `Node`, `Item`), `reading.rs`.
- `crates/english/src/census.rs`, `corpus.rs`, `crates/upstream/src/scryfall.rs`
  (`OracleCard::supported`).
- `crates/english/grammar/english.fug` (`feature Voice`, the `VerbPhrase` Categories).
- `data/mtgjson/AllPrintings.sqlite`: `cards`, `cardIdentifiers`, `cardLegalities`.

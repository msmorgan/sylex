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

- `cargo xtask english forests` writes one SQLite file, `data/derived/english-forests.sqlite`
  (`/data/` is gitignored), for every unit of the census corpus, read through
  `sylex_upstream::scryfall::oracle_cards`. It has no query features: questions are SQL.
  `crates/english/README.md` gives example queries with their output on the pool.
- The SQLite dependency is `rusqlite` with its bundled SQLite.
- Units carry the keys that join to mtgjson: the face's Scryfall Oracle id (to
  `cardIdentifiers.scryfallOracleId`), the face name and card name as the corpus has them, the
  unit kind and index, the text parsed and its Derivation total. An mtgjson `uuid` names a
  printing, not a face, so it is no unit key.
- The stored tables keep the forest exactly as packed and binarized: a node's alternative is a
  (Production, completed item) pair, and an item's families are (previous item, child) pairs.
  Every node, alternative, item and family carries its exact Derivation count from `Counts`,
  and every node its settled flag. Only the part of the forest some Derivation uses is stored.
  Nothing is flattened into per-alternative child lists in a stored table, since that multiplies
  where an item's prefixes branch.
- Production, Selected Frame (with its Valence Alternation variant), head lexeme, lemma and form
  belong to a node's alternatives, not the node. One alternative may reach more than one head
  word through its items, so heads are rows of their own with exact counts.
- Each alternative's Embedding, the chain of dependent functions it stands under, is stored with
  exact counts per alternative, so a view over it can ask "in every Derivation" (count equals the
  unit's total) and "in some Derivation" (count above zero) alike.
- Views: every verb phrase alternative with its verb, Selected Frame, Voice and alternation
  variant, form and Embedding; and every coordination node with its Category, coordinator and
  conjunct count. A view that flattens says in its comment what it shows and what it must not be
  used to count.
- No unit is capped.

## Done when

- The landing reports for the pool: total units, nodes, items, families and rows per table, the
  largest unit, the file size and the run time.
- The database is written for every unit of the census corpus, and tests on real units check
  that each unit's stored total equals `derivation_count`, that a node's alternatives' counts
  sum to the node's, and that the stored rows reproduce the forest.
- Writing the database twice gives the same content.
- The views are exercised by example queries in the landing, with their output.
- The landing reports how many census faces have no Vintage `Legal` or `Restricted` printing in
  mtgjson by the join (the census corpus is chosen by Scryfall's legality, `OracleCard::supported`).
- `jj --no-pager status` shows no database file, and `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Not in scope

- A row per Reading, and any query interface beyond SQL views.
- Forks and the views over them: `english-forest-database-forks`.
- The per-unit dump of a sliced forest: an open question of `english-forestwalk-narrowing`.

## Pointers

- `crates/syntax/src/chart.rs` (`PackedForest`, `Node`, `Item`), `counts.rs`, `reading.rs`.
- `crates/english/src/census.rs`, `corpus.rs`, `crates/upstream/src/scryfall.rs`
  (`OracleCard::supported`).
- `crates/english/grammar/english.fug` (`feature Voice`, the `VerbPhrase` Categories).
- `data/mtgjson/AllPrintings.sqlite`: `cards`, `cardIdentifiers`, `cardLegalities`.

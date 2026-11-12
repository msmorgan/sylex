---
needs: [syntax-lexeme-and-coordinator-ids]
---

# Duplicated chart storage, character-indexed tables and repeated forest walks in `sylex_syntax`

## Why

Chart construction and post-chart forest analysis in `sylex_syntax` duplicate keys, allocate per
character inside words, bypass their own chunked hasher, and re-walk the same immutable forest
across passes. In `CompiledGrammar::run`, `add()` probes `item_index.get(&key)` and then hashes the
key a second time on `item_index.insert(key, i)`, storing a full copy of `ItemKey` in `HashMap`
buckets alongside `forest.items` (`node_index` does the same for `Summary` alongside
`forest.nodes`), and `[FeatureValue; N]` arrays in `State` and `Summary` derive element-by-element `write_u8`
hashing instead of hashing a 64-bit word per eight values, and `sets`, `item_index` and `waiting`
pre-allocate entries for every character offset `0..=n` only to skip every offset inside a token. After parsing, `Parse::children_first` advances through families with
quadratic `.nth(index)` calls per DFS step, `Graph::order` allocates a throwaway `Vec<V>` of
successors per vertex, and `Parse`'s forest queries (`counts`, `alternative_heads`,
`alternative_conjuncts`, `alternative_embeddings`, `fork_graph::split`) recompute `children_first`
up to five times and `head_inside` twice on the same `Parse`.

## Design

- `sylex_syntax` adds `hashbrown` as a direct dependency and deduplicates `item_index` and
  `node_index` with `hashbrown::HashTable<usize>` indexing into `forest.items` and `forest.nodes`,
  probing once per lookup or insert and storing each `Item` and `Node` only once in `PackedForest`.
- `FeatureValue` implements `Hash::hash_slice` in safe code by packing each 8-element chunk into a
  `u64` (`write_u64`), hashing `[FeatureValue; N]` arrays a word at a time with no `unsafe` or new
  cast crate.
- `CompiledGrammar::run` allocates `waiting` and `item_index` entries only at visited boundary
  positions rather than for every character offset `0..=n` in the input string.
- `Parse::children_first` and `Graph::order` advance through families and successors with a
  `(cursor, phase)` index frame on the DFS stack, removing `.nth(index)` re-scans and per-vertex
  `Vec` allocations.
- `Parse` memoizes `children_first` and `head_inside` with `OnceLock` (keeping `Parse` `Sync`) so
  `counts`, `alternative_heads`, `alternative_conjuncts`, `alternative_embeddings` and
  `fork_graph::split` share one traversal of the immutable forest instead of recomputing them per
  call.

## Done when

- On the full pool, `cargo xtask english forests` produces table-by-table identical content to
  before the change, and `cargo xtask english strata --check` writes an identical `strata.json`
  report.
- `item_index` and `node_index` store indices into `PackedForest` with no duplicate `ItemKey` or
  `Summary` bucket keys and one hash per miss; `[FeatureValue; N]` hashes a word at a time in safe
  code; `CompiledGrammar::run` allocates no `waiting` or `item_index` entries at unvisited interior
  character offsets; neither `Parse::children_first` nor `Graph::order` uses `.nth(index)` or
  per-vertex `Vec` allocation; and repeated forest queries on one `Parse` share `children_first` and
  `head_inside`.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_syntax`, `sylex_english`
  and `xtask`.

## Not in scope

- Dense `Lexeme` and coordinator ids (`syntax-lexeme-and-coordinator-ids`) and bind-time enum,
  child-role, `ChildSlot` and `normal_form` resolution (`syntax-bind-time-types`).
- Changing the grammar (`english.fug`), admitted Readings or `Summary` Packing equivalence.

## Pointers

- `crates/syntax/src/chart.rs` (`CompiledGrammar::run`, `ChartMap`, `ItemKey`, `Item`, `Node`,
  `PackedForest`), `crates/syntax/src/hash.rs` (`ChartHasher`).
- `crates/syntax/src/step.rs` (`FeatureValue`, `State`, `Summary`).
- `crates/syntax/src/counts.rs` (`Parse::children_first`, `Parse::counts`),
  `crates/syntax/src/alternatives.rs` (`alternative_heads`, `alternative_conjuncts`,
  `alternative_embeddings`, `head_inside`).
- `crates/syntax/src/fork_graph.rs` (`Graph::order`, `split`), `crates/syntax/src/forks.rs`.

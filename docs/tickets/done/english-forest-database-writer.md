---
needs: [english-forest-database-pipeline]
---

# The writer thread of `cargo xtask english forests`

## Why

A full-pool run was bound by its one writer thread: of an 85 s run the writer was busy about
75 s. About 7 s of that was freeing each unit's rows after writing them and about 6 s looking
Embeddings up in a `BTreeMap`.

## Design

- A unit's rows are freed on a thread of their own once written.
- Embeddings are interned in a hash map and `embeddings` and `embedding_steps` are written in
  id order.
- Nothing further is done to the writer. With a writer that discards every unit the run takes
  57 s on the 12-core, 24-thread machine it was measured on, and with the two steps above it
  takes about 74 s, so parsing bounds the run within some 17 s. The other steps tried each
  bought little for its code: multi-row `INSERT` statements about 3 s of wall time for about
  180 lines, building the bound values on the parsing threads about 2 s on top of that,
  collecting Lexemes on the parsing threads and creating `head_lines_by_word` after the load
  under a second each. Opening the connection without its mutex is what `rusqlite` already
  does.

## Done when

- Every table's content equals what the command wrote before the change, compared on the full
  pool by the hash of the database's `.dump`.
- `cargo test -p xtask` and `cargo clippy -p xtask --tests -- -D warnings` pass.

## Not in scope

- More than one writer, or writing tables to separate databases.
- The parsing side, which now bounds the run.

## Pointers

- `crates/xtask/src/english/forests.rs` (`write`, `write_unit`, `write_embeddings`),
  `crates/xtask/src/english/forests/pipeline.rs`.

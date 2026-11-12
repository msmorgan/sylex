---
needs: [english-forest-database-pipeline]
---

# The writer thread of `cargo xtask english forests`

## Why

A full-pool run is bound by its one writer thread: in a profile of an 81 s run the writer was
busy about 72 s, while each of the 24 parsing threads was busy about 45 s. Of the writer's time,
about 60% is SQLite executing one `INSERT` per row (B-tree inserts and index seeks, the bytecode
interpreter, and opening and closing a statement's cursors for every row), 10% binding
parameters, 10% freeing each unit's rows after writing them, 8% looking Embeddings up in a
`BTreeMap`, and 2% the `BTreeSet` of Lexemes. File writes are 3% and formatting Word Forms under
1%, so a larger page cache and cheaper formatting buy nothing.

## Design

- Each step below is measured on the full pool with `perf` (a frame-pointer build, the writer
  thread's samples) and kept only if it shortens the writer's time; a step that does not is left
  out, and the landing says so with its measurement.
- A unit's rows are freed off the writer thread once written.
- Embeddings are interned in a hash map with a vector in id order, and `embeddings` and
  `embedding_steps` are written in id order.
- Lexemes are collected without an ordered-set probe per row on the writer thread.
- `head_lines_by_word` is created after the rows are loaded, inside the transaction.
- The connection is opened without SQLite's connection mutex, which one thread owning the
  connection does not need.
- The tables that hold most rows are written with multi-row `INSERT` statements, the statement
  for a chunk built once from the table's column count, not written out by hand per table.

## Done when

- Every table's content equals what the command wrote before the change, compared table by
  table on the full pool, and is identical across repeated runs. The schema and views are
  unchanged.
- The landing reports, before and after, the run's wall-clock time and the writer thread's busy
  time with its split (SQLite execution, binding, freeing, Embedding lookup), and for each
  Design step the writer time it saved or that it was left out.
- `cargo test -p xtask` and `cargo clippy -p xtask --tests -- -D warnings` pass.

## Not in scope

- More than one writer, or writing tables to separate databases.
- The parsing side: with the writer free, parsing bounds the run at roughly half its present
  time on 24 cores.

## Pointers

- `crates/xtask/src/english/forests.rs` (`write`, `write_unit`, `write_embeddings`, `Inserts`),
  `crates/xtask/src/english/forests/forks.rs` (`Inserts`, `write`),
  `crates/xtask/src/english/forests/pipeline.rs`, `crates/xtask/src/english/forests/schema.rs`.

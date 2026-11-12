---
needs: [english-forest-database, english-forest-database-forks]
---

# Pipeline and dev-profile optimization for `cargo xtask english forests`

## Why

`cargo xtask english forests` runs under the `dev` profile without `opt-level` overrides on
`libsqlite3-sys` or `rusqlite`, and alternates in lockstep between parsing 256-unit batches on
Rayon and inserting rows on one thread while the worker pool sits idle. Full-pool generation
spends most of its wall time single-threaded inside unoptimized SQLite writes and batch straggler
barriers.

## Design

- Add `opt-level = 2` overrides in the workspace root `Cargo.toml` for `libsqlite3-sys`,
  `rusqlite` and `dpsi` under `[profile.dev.package.*]`.
- Replace the stop-the-world `BATCH` loop in `crates/xtask/src/english/forests.rs` with parallel
  workers streaming `(unit, UnitRows)` over an MPSC channel to a dedicated SQLite writer thread
  inside `std::thread::scope`, keeping the single outer SQLite transaction.
- Dispatch units in monotonic order under a bounded in-flight window (`rayon::scope_fifo` with
  bounded permits, or `rayon::broadcast` over an `AtomicUsize` counter gated by the writer's
  cursor) rather than bisecting `0..units.len()` against a bounded channel, so workers cannot fill
  the window with high-numbered units and deadlock while the writer awaits earlier ones.
- Buffer out-of-order completions on the writer thread in a `BTreeMap<usize, UnitRows>` keyed by
  unit index and flush consecutive units from `0` upward, preserving sequential rightmost B-tree
  appends on `(unit, …)` `WITHOUT ROWID` tables and deterministic `embeddings` integer IDs.
- Prepare all per-unit `INSERT` statements once for the transaction (across `forests.rs` and
  `forests/forks.rs`) instead of calling `prepare_cached` per unit, and look up existing keys in
  `embeddings` before cloning `Embedding` on insert.

## Done when

- `cargo xtask english forests` writes identical table content across repeated runs, and its output
  on a multi-card subset matches the schema and invariants tested in
  `crates/xtask/src/english/forests.rs`.
- The landing reports wall-clock time, user CPU time, average CPU utilization and peak RSS on the
  full pool before and after the change.
- `jj --no-pager status` shows no database file, and `cargo test -p xtask` and
  `cargo clippy -p xtask --tests -- -D warnings` pass.

## Not in scope

- Sharing a concurrency abstraction with `sylex_english::census::run`, whose small per-unit
  results fit in a single `par_iter().collect()`.
- Changing the database schema or views, or altering the forest passes in `sylex_syntax`.

## Pointers

- `Cargo.toml` (`[profile.dev.package.*]`), `.cargo/config.toml` (`xtask` alias).
- `crates/xtask/src/english/forests.rs`, `crates/xtask/src/english/forests/forks.rs`,
  `crates/xtask/src/english/forests/schema.rs`.

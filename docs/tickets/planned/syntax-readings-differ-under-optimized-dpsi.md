---
needs: []
---

# A unit's Readings differ when `dpsi` is built optimized

## Why

With `[profile.dev.package.dpsi] opt-level = 2` in the root `Cargo.toml`,
`cargo test -p sylex_english --test forest_forks` fails every run on *Maralen, Fae Ascendant*'s
third line and *Draugr Necromancer*'s second: "the split forest's Readings are not the
forest's", the set `ForkForest::readings` gives against the set `Parse::readings` gives, with
equal counts. Without the override it passes every run. No output of the parser may depend on
how a dependency is optimized, so something compares or orders by a fact that optimization
changes. `dpsi::Ident` is equal by address and ordered by text, and every constructor interns,
which reads as sound; the cause is not found. The override is removed, so the dev profile hides
the defect and a release build may show it.

## Done when

- The cause is named: which comparison, ordering or hash gives a different result under the
  optimized `dpsi`, and in which crate.
- The cause is fixed, and `cargo test -p sylex_english --test forest_forks` passes with the
  `dpsi` override restored and under `--release`.
- `cargo test` passes for each changed crate, in the dev profile and with `--release`.

## Pointers

- `crates/english/tests/forest_forks.rs` (`check`, the comparison of `forest.readings` with
  `parse.readings`); `crates/syntax/src/fork_graph.rs` (`Graph::trees`);
  `crates/syntax/src/reading.rs` (`Reading`, its `Ord`).
- `dpsi` 0.1.1 (`Ident`: `PartialEq` by address, `Ord` and `Hash` by text).
- Both failing units have a *from among* phrase.

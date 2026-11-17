---
needs: []
---

# No test witnesses a conditional fork

## Why

A fork is conditional when its word has an answer in only some of a unit's Derivations
(`crates/syntax/src/forks.rs`). The fork tests over real units (`crates/english/tests/forest_forks.rs`,
`crates/english/tests/settled_spine.rs`, `the_stored_forks_equal_the_fork_forest_s` in
`crates/xtask/src/english/forests/forks.rs`) found one only in *Turtles in Time*'s "Exile Turtles in
Time.", whose second Reading read the card's name as words with capitals where none go. That Reading
is gone, no unit in scope has a conditional fork, and the conditional path of the fork forest, its
display and its `forks.conditional` column are untested.

## Done when

- A test asserts a conditional fork's context, options and display, on a real unit or on a minimal
  repro derived from a named real card, and the forest database test checks its stored
  `conditional` column.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Pointers

`crates/syntax/src/forks.rs` (`Fork::conditional`), `crates/syntax/src/fork_graph.rs`,
`crates/english/tests/forest_forks.rs`, `crates/xtask/src/english/forests/forks.rs`.

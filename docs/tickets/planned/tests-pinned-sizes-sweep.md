---
needs: []
---

# Tests and comments pin sizes that move with the grammar

## Why

Tests and their comments state the Derivation count of named real units, and one asserts it:
`assert_eq!(forest.total(), 3472)` for *Remove Enchantments* in
`a_large_forest_narrows_by_the_surveys_option_counts`. A unit's forest size changes with any
grammar change that touches it, so the assertion fails on a change that broke nothing, and the
comments are wrong until someone notices. What those tests are for is a large forest: that it
narrows without enumeration, that its forks are counted exactly, that a judgment on it is
evaluated like any other. None of them is about the number.

The known instances: that assertion, and the sizes of *Draugr Necromancer* and *Remove
Enchantments* in the comments of `crates/english/tests/{forest_alternatives,forest_counts,
forest_forks,judgments,settled_spine}.rs`, `crates/xtask/src/english/forestwalk/tests.rs` and
`crates/xtask/src/english/forests/forks.rs`.

## Done when

- No test asserts by equality a size of a real unit's parse (its Derivations, chart items,
  forest nodes, forks or Segments) unless that size is what the test is about, and its doc
  comment says so. A test that only needs a large or an ambiguous forest asserts that.
- No comment under `crates/` states a real unit's Derivation count.
- The sweep covers every equality assertion on a count or size in `crates/*/tests` and in the
  test modules under `crates/xtask/src`. The landing lists each one it examined, with why its
  number is what the test is about, or how it was rewritten, or why it was retired.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Not in scope

- A count fixed by a test's own grammar, as the Catalan number of attachments in
  `crates/syntax/tests/engine.rs`: no change to the English grammar moves it.
- A judgment entry's `readings N` assertion.
- The example query output in `crates/english/README.md`.

## Pointers

`crates/english/tests/`, `crates/xtask/src/english/forestwalk/tests.rs`,
`crates/xtask/src/english/forests/forks.rs`, `crates/syntax/tests/`.

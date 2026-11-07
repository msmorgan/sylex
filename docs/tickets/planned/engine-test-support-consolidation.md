---
needs: []
---
# Share the engine integration-test setup helpers

## Why

Each file in `crates/engine/tests/` carries its own copy of the same fixture setup:
`force_onto_battlefield` is defined in nine files, `force_into_play` and `find_in_hand` in several,
and `card()`, `plugin()` and `two_player_with()` recur. The copies have drifted into different
operations under one name: `layers.rs` searches the hand and then the library, `combat.rs`,
`stack.rs`, `activate.rs` and `enumerate.rs` require a hand card, `planeswalker_jace.rs` takes any
owned card from hand or library, and `pip_payment.rs` and `replace_registry.rs` move an existing
object id. A single permissive helper would let a test pass with the wrong fixture, so the shared
helpers must keep these operations apart.

## Done when

- A shared support module under `crates/engine/tests/` holds the behaviorally identical setup, with
  one distinctly named helper per distinct operation (hand only, hand or library, an existing
  object id), and no helper that falls back from one search to another unless that fallback is
  its stated operation.
- Every duplicated caller uses the shared helpers, and each call site's fixture search is
  unchanged.
- Any helper left local to one test file has a one-line comment saying why it is not shared.
- Every integration test that existed before still exists under the same name and passes.

## Not in scope

The crate-internal unit-test fixtures in `crates/engine/src/test_support.rs`.

## Pointers

- `rg -n 'fn (force_onto_battlefield|force_into_play|find_in_hand|find_owned|card|plugin|two_player_with)\(' crates/engine/tests`

---
needs: []
---

# A Production states its cases once

## Why

Where two Productions have the same mother and children and differ only in equations that hold
for one value of a child's feature, the grammar writes both out, and the copies drift:
`ComparativeQuantityCountDetermined` and `ComparativeCountCluster` lacked an exclusion their
mass twins had, which admitted *more than you do creatures*. About eleven such pairs stand in
`crates/english/grammar/english.fug` (count against mass, light against heavy).

## Design

- An authored Production may carry `cases`, keyed on one child feature's value, each adding its
  own equations: `cases: [Mass -> [eqs], Count -> [eqs]]`. The compiler emits one Production per
  case, named `Name/Value`, as premodification already expands by form. Nothing in
  `crates/syntax/src/step.rs` changes, and the forest is the same apart from Production names.
- A case list that leaves a value of its feature unstated is a load error.
- Only pairs that are one construction are merged. A pair that builds two constructions
  (distributive against plain agreement, distributive against joint reference) stays two
  Productions.

## Done when

- `.fug` reads `cases`, with load errors for an unknown value, a value stated twice and a value
  left out, each tested.
- The pairs that are one construction are written as one Production with cases; the landing
  lists each pair merged and each close pair left apart with the reason.
- Where the two members of a pair both admitted one input today (a noun that is both count and
  mass), the landing says so with the units, and whether the second is a wanted Reading.
- The forest is unchanged apart from Production names, and `cargo xtask english strata --check`,
  `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_syntax`,
  `sylex_english` and `xtask`.

## Pointers

- `crates/syntax/src/fug.rs`, `grammar.rs`, `compile.rs` (the premodification expansion).
- `crates/english/grammar/english.fug`: `ComparativeCountCluster` / `ComparativeMassCluster`,
  `ComparativeQuantityCountDetermined` / `ComparativeQuantityMassDetermined`,
  `MaximumDetermination` / `MaximumMeasuredMass`, `AnyPartitive` / `AnyMassPartitive`,
  `NumericCountPremodifier` / `NumericMassPremodifier`.

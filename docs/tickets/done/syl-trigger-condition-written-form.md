---
needs: [cleanup-step6-spine]
---

# Refuse a trigger written in a form its Template already writes

## Why

One printed text has one written form (`docs/design/spelling.md` §4): a one-condition trigger
is the Ability `when`, `whenever` or `at`, and `triggered` is written only over a disjunction of
trigger conditions. `cargo xtask dsl check` resolves the other forms too:
`triggered(when(e), action = a)` beside `when(e, a)`, an `or` at `TriggerCondition` (or the
Template `and` over it) with fewer than two conditions, and one nested directly in another, so
nothing keeps the corpus on the one form.

## Design

- Every conjunction and disjunction constructor (`And`, `AnyOf`, `Compound`, `Conjunction`,
  `Coordination` and `Or`), and every Template that forwards a `List` parameter into one, takes two
  or more items; `cargo xtask dsl check` refuses an empty list and names the single-item form for a
  list of one.
- Direct nesting of `and` or `or` is refused at `TriggerCondition` only, where both words mean
  disjunction [CR#603.1b,603.2].

## Done when

- `cargo xtask dsl check` refuses, each with a refusal that names the written form to use:
  `triggered(…)` whose condition is a bare `when`, `whenever` or `at` (outside that `Ability`
  Template's own body); a conjunction or disjunction constructor (or Template over one) with fewer
  than two items; and an `and` or `or` at `TriggerCondition` that directly contains another `and`
  or `or` at `TriggerCondition`.
- Every declaration and card under `plugins/` passes (`everyObject` deleted; *Shelkin Brownie* and
  *Tolaria* unwrapped from `compound([tapSymbol])` to `tapSymbol`).
- Negative tests in `crates/xtask/tests/suite/dsl_check.rs` pin each refusal.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask` and
  `cargo clippy -p xtask --tests -- -D warnings` pass.

## Pointers

`docs/design/spelling.md` §4, `crates/xtask/src/dsl_check.rs`,
`plugins/builtin/primitives/ability/triggered.syl`, `plugins/builtin/primitives/triggerCondition/`,
`plugins/builtin/templates/triggerCondition/`.

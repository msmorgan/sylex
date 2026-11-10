---
needs: [cleanup-step6-spine]
---

# Refuse a trigger written in a form its Template already writes

## Why

One printed text has one written form (`docs/decisions/spelling.md` §4): a one-condition trigger
is the Ability `when`, `whenever` or `at`, and `triggered` is written only over a disjunction of
trigger conditions. `cargo xtask dsl check` resolves the other forms too:
`triggered(when(e), action = a)` beside `when(e, a)`, an `and` at `TriggerCondition` (or the
Template `or` over it) with fewer than two conditions, and one nested directly in another, so
nothing keeps the corpus on the one form.

## Open questions

- The `or` at `GameEvent` and at `ProducedMana` accept an empty or one-arm list too. Should the
  fewer-than-two refusal be general, over every disjunction and conjunction constructor, or stay
  at `TriggerCondition`?

## Done when

- `cargo xtask dsl check` refuses, each with a refusal that names the written form to use:
  `triggered(…)` whose condition is a bare `when`, `whenever` or `at`; an `and` or `or` at
  `TriggerCondition` with fewer than two conditions; and one that directly contains another
  `and` or `or` at `TriggerCondition`.
- Negative tests in `crates/xtask/tests/suite/dsl_check.rs` pin each refusal.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask` and
  `cargo clippy -p xtask --tests -- -D warnings` pass.

## Pointers

`docs/decisions/spelling.md` §4, `crates/xtask/src/dsl_check.rs`,
`plugins/builtin/primitives/ability/triggered.syl`, `plugins/builtin/primitives/triggerCondition/`,
`plugins/builtin/templates/triggerCondition/`.

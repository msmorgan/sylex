---
needs: [syl-primitive-constructors]
---

# `theRingTemptsYou` without the reserved scope

## Why

`plugins/builtin/definitions/keyword_action/theRingTemptsYou.syl` writes `Action.ownScope` around
its own deed, so that the Ring emblem can count the times the Ring has tempted you by the deed's
own steps. `.syl` may not write a scope (`docs/decisions/syl.md` §4); the file does so through a
temporary primitive declared in it (`ownScope`) and the validator's path-gated Ring exception,
which is the only reason the file passes `cargo xtask dsl check`.

## Done when

- `theRingTemptsYou.syl` writes no scope and declares no primitive.
- The Ring emblem still counts the times the Ring has tempted you.
- `cargo xtask dsl check` has no Ring path exception (`RING_EXCEPTION` in
  `crates/xtask/src/dsl_check/schema.rs`, and `Signature::ring_only` in
  `crates/xtask/src/dsl_check.rs`), so a primitive building a reserved constructor is refused in
  every file, the Ring's included, with a negative test.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/definitions/keyword_action/theRingTemptsYou.syl`,
`crates/xtask/src/dsl_check/schema.rs` (`RING_EXCEPTION`, `EXPANSION_CONSTRUCTORS`),
`crates/xtask/src/dsl_check.rs` (`ring_only`), `docs/decisions/semantics.md` §2.3 and §3,
`docs/decisions/syl.md` §4.

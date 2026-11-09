---
needs: [syl-primitive-constructors, syl-explicit-deeds]
---

# `theRingTemptsYou` without the reserved scope

## Why

`plugins/builtin/definitions/keyword_action/theRingTemptsYou.syl` writes `Action.ownScope` around
its own deed, so that the Ring emblem can count the times the Ring has tempted you by the deed's
own steps. `.syl` may not write a scope (`docs/decisions/syl.md` §4); the file does so through a
temporary primitive declared in it (`ownScope`) and the validator's path-gated Ring exception,
which is the only reason the file passes `cargo xtask dsl check`. The body also writes the same
count of temptations (`eventCount(ownScope(theRingTemptsYou, …), actor(you), thisGame)`) three
times, once per emblem ability that reads it [CR#701.54c]; only the threshold differs.

## Done when

- `theRingTemptsYou.syl` writes no scope and declares no primitive.
- The count of temptations is written once, as a parameterless `Amount` Template that the three
  emblem abilities call. It is a closed expression with no parameter, so a Template expands it
  the same way at each use and no local binding is needed.
- The Ring emblem still counts the times the Ring has tempted you. No check can confirm this until
  the Rust resolution and expansion layer exists; the landing shows how the rewritten body counts
  them, for the owner to review.
- `cargo xtask dsl check` has no Ring path exception (`RING_EXCEPTION` in
  `crates/xtask/src/dsl_check/schema.rs`, and `Signature::ring_only` in
  `crates/xtask/src/dsl_check.rs`), so a primitive building a reserved constructor is refused in
  every file, the Ring's included, with a negative test.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/definitions/keyword_action/theRingTemptsYou.syl`,
`crates/xtask/src/dsl_check/schema.rs` (`RING_EXCEPTION`, `EXPANSION_CONSTRUCTORS`),
`crates/xtask/src/dsl_check.rs` (`ring_only`), `docs/decisions/semantics.md` §2.3 and §3,
`docs/decisions/syl.md` §4.

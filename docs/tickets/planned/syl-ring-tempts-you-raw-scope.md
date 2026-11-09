---
needs: [syl-primitive-constructors]
---

# `theRingTemptsYou` without the reserved scope

## Why

`plugins/builtin/definitions/keyword_action/theRingTemptsYou.syl` writes `Action.ownScope` around
its own deed, so that the Ring emblem can count the times the Ring has tempted you by the deed's
own steps. `.syl` may not write a scope (`docs/decisions/syl.md` §4); the file does so through a
temporary primitive declared in it (`ownScope`) and `sylGen`'s path-gated `legacyScope`
exception, which is the only reason the file loads.

## Done when

- `theRingTemptsYou.syl` writes no scope and declares no primitive.
- The temporary `ownScope` primitive and the `legacyScope` exception (`lean/SylGen/Main.lean`, and
  its uses in `Emit.lean` and `Resolve.lean`) are deleted, so a primitive building a reserved
  constructor is refused in every file, the Ring's included, with a negative test.
- The validator's mirror is deleted: `cargo xtask dsl check` has no Ring path exception
  (`RING_EXCEPTION` in `crates/xtask/src/dsl_check/schema.rs`, and `Signature::ring_only` in
  `crates/xtask/src/dsl_check.rs`), and its negative test refuses the Ring's
  file too.
- The Ring emblem's count reads the same as before: `lake exe dumpValues compare` shows no
  changed card or builtin row besides `theRingTemptsYou`'s own and the deleted `ownScope`'s.
- `cargo xtask lean check` and `cargo test -p sylex_syl` pass.

## Pointers

`plugins/builtin/definitions/keyword_action/theRingTemptsYou.syl`, `lean/SylGen/Main.lean`
(`legacyScope`), `lean/SylGen/Environment.lean` (`sourceConstructorRefusal`),
`docs/decisions/semantics.md` §2.3 and §3, `docs/decisions/syl.md` §4.

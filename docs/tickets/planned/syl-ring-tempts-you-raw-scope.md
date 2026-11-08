---
needs: [syl-primitive-constructors]
---

# `theRingTemptsYou` without the reserved scope and Deed

## Why

`plugins/builtin/definitions/keyword_action/theRingTemptsYou.syl` writes `Action.ownScope` and
`Deed.action`, which `.syl` may not write (`docs/decisions/syl.md` §4), so that the Ring emblem
can count the times the Ring has tempted you by the deed's own steps. It does so through three
temporary primitives declared in that file (`ownScope`, `action`, `noPermanent`) and `sylGen`'s
path-gated `legacyScope` exception, which is the only reason the file loads.

## Done when

- `theRingTemptsYou.syl` writes no scope and no raw Deed, and declares no primitive.
- The three temporary primitives and the `legacyScope` exception (`lean/SylGen/Main.lean`, and
  its uses in `Emit.lean` and `Resolve.lean`) are deleted, so a primitive building a reserved
  constructor is refused in every file, the Ring's included, with a negative test.
- The Ring emblem's count reads the same as before: `lake exe dumpValues compare` shows no
  changed card or builtin row besides `theRingTemptsYou`'s own and the deleted primitives'.
- `cargo xtask lean check` and `cargo test -p sylex_syl` pass.

## Pointers

`plugins/builtin/definitions/keyword_action/theRingTemptsYou.syl`, `lean/SylGen/Main.lean`
(`legacyScope`), `lean/SylGen/Environment.lean` (`sourceConstructorRefusal`),
`docs/decisions/semantics.md` §2.3 and §3, `docs/decisions/syl.md` §4.

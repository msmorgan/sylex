---
needs: [syl-single-face-card-call]
---

# Whether Template bodies become opaque

## Why

A Template expands transparently (`docs/decisions/syl.md` §4): its arguments are substituted into
its body, so each use of a parameter is checked only after expansion. A Definition's body is
opaque by comparison. If Template bodies were opaque too, every argument would be resolved once
at the call site and each use checked against its own slot.

Written against the start of step 6 (`syl-single-face-card-call`).

## Open questions

- Whether Template bodies become opaque, every argument resolved once at the call site and each
  use checked against its own slot.

## Done when

- The answer is in `docs/decisions/syl.md` §4. If Templates become opaque, `cargo xtask dsl check`
  resolves each argument once at its call site and checks each use against its slot, with a
  negative test, and every Template in `plugins/` passes it.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`docs/decisions/syl.md` §1 and §4, `docs/decisions/semantics.md` §3,
`crates/xtask/src/dsl_check.rs`.

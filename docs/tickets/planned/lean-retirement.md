---
needs: [discourse-corpus-agreement, idris-retirement]
---
# Delete `lean/` and `sylGen`

Standard constraints apply. Settled (owner, 2026-11-02): Lean is deleted in full, and no Lean
mirror of the DSL is kept. A mirror would keep the DSL bound to Lean: its keywords (the source of
the trailing-underscore names) and an AST that records no written order. `idris/` went first.

- Delete `lean/` (the checker, proofs, `sylGen`, the generated-module plumbing) and
  `lean-discourse-export`'s exporter and snapshot with it. The snapshot is not maintained: its
  expected outputs are frozen at deletion, and the first AST change afterwards would break fixtures
  nothing can regenerate. After this, the spec catalog's Rust tests and the canon corpus are the
  only record of what must and must not check.
- Delete `cargo xtask lean` and every other Lean-only xtask command, the Lean CI job, the Lean
  cache provisioning in `scripts/provision-workspace`, and Lean-only settings, skills and hooks
  references the repo carries.
- Before deleting, port the Lean AST's doc comments onto the Rust types in
  `crates/deckmaste_semantics_v3/src/ast` (their only doc line points into `lean/Semantics/`), with
  every CR citation audited. The generator `semantics-rust-crate` describes runs only while
  `lean/` exists. `cr-coverage.lock` is not lowered: every rule cited only in `.lean` files must be
  cited again where it now lives.
- Update `CLAUDE.md` (crate fates, Bearings, Tests for a landing), `docs/decisions/semantics.md`,
  `docs/decisions/syl.md` and the memory index entries that name Lean as current.
- The `lean-v3-*` and `plugins-v3-*` tickets that need this one are then done in `.syl` and Rust
  only.

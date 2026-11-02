---
needs: []
---
**Emit `crates/deckmaste_plugin/tables/entailments.ron` without Idris.** The
engine reads it at build time (`deckmaste_engine/src/entail.rs`,
`include_str!`), and today `idris/scripts/emit-tables` regenerates it from
`idris/src/EmitTables.idr`; the CI `idris` job diffs the committed file against
that output. It is the one runtime artifact the Idris tree still produces.
Move the emission to a Rust derivation in xtask (output byte-identical to the
committed file on first landing). Not Lean: its checker is retired after Idris, and its DSL mirror may follow
(`semantics-rust-rewrite`, owner 2026-11-02). CI diffs the committed file
against the new emitter; `idris/scripts/emit-tables` and `EmitTables.idr` are
deleted in the same landing. Related: `idris-emittables-drift-control`
becomes moot and is closed by this ticket.

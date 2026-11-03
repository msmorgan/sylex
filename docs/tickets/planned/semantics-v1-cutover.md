---
needs: [semantics-v3-parity]
---
**Retire v1 scaffolding and move the `.syl` plugins to their canonical home (updated 2026-10-14
per [`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md)).**
Delete the `v1` set (`deckmaste_semantics`, `deckmaste_lowering`,
`deckmaste_legacy_render`, `deckmaste_plugin` with `deck.rs` and
`provenance.rs` rehomed, `plugins/builtin`, and `plugins/canon`; `idris/` is already gone
with `idris-retirement`;
the `deckmaste_spelling` splice machinery and v1 `deckmaste_english` are
already retired, and `deckmaste_spelling` is now a stub crate;
`deckmaste_semantics_v2`,
`plugins_v2/canon/`, and `plugins_v2/builtin/` are retired in
`semantics-v3-dsl-and-plugins`, and the pre-v4 `deckmaste_lexical_source` was
retired in `english-v4-cutover-v3`), use the `deckmaste_semantics_v3`, `deckmaste_discourse`, and
`deckmaste_lowering_v3` crates from `semantics-rust-rewrite` (dropping the `_v3` suffixes), move
`plugins_v3/` to `plugins/`, drop the symlink, and re-point
`deckmaste_migrations`. Coordinate with `idris-retirement` and
`english-v4-final-suffix-cutover` (whose remaining scope moves
`catalogs/*.ron` into `plugins/` metadata and deletes `deckmaste_features`;
the `_v4` suffixes are already dropped).
Standard constraints apply.

The Idris card gate is retired earlier, by `idris-retirement` (owner, 2026-11-02, replacing the
2026-09-06 ruling that retired it here).

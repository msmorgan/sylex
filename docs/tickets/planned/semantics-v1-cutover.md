---
needs: [semantics-v3-parity]
---
**Retire v1 scaffolding (the `.syl` plugins already live in `plugins/`; updated 2026-10-14
per [`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md)).**
Delete the `v1` set (`sylex_semantics_v1`, `sylex_lowering_v1`,
`sylex_render_v1`, `sylex_plugin_v1` with `deck.rs` and
`provenance.rs` rehomed, `v1/plugins/builtin`, and `v1/plugins/canon`; `idris/` is already gone
with `idris-retirement`;
the `deckmaste_spelling` splice machinery and v1 `deckmaste_english` are
already retired, and the `spelling` stub crate is deleted (`sylex_spelling` is planned for when
spelling starts);
`deckmaste_semantics_v2`,
`plugins_v2/canon/`, and `plugins_v2/builtin/` are retired in
`semantics-v3-dsl-and-plugins`, and the pre-v4 `deckmaste_lexical_source` was
retired in `english-v4-cutover-v3`), use the `deckmaste_semantics_v3`, `deckmaste_discourse`, and
`deckmaste_lowering_v3` crates from `semantics-rust-rewrite` (dropping the `_v3` suffixes), and re-point
`sylex_pipeline_v1`. Coordinate with `idris-retirement` and
`english-v4-final-suffix-cutover` (whose remaining scope moves
`catalogs/*.ron` into `plugins/` metadata and deletes `sylex_features_v1`;
the `_v4` suffixes are already dropped).
Standard constraints apply.

The Idris card gate is retired earlier, by `idris-retirement` (owner, 2026-11-02, replacing the
2026-09-06 ruling that retired it here).

---
needs: [engine-ability-origin-refs]
---

# Delete `v1/` once the new line reaches parity

**Why.** `v1/` is the old pipeline: its own semantics, lowering, render, plugin loader, pipeline,
features and RON macro crates, and its RON plugins. The engine, the TUI, the matchups and `xtask`
still load cards through it, and engine tests and examples load `v1/plugins/builtin` and
`v1/plugins/canon` directly. It stays only until the `.syl` semantics and its lowering to
`sylex_core` can feed the engine; then it is dead weight that every build compiles.

**Done when.**
- Every crate outside `v1/` gets its cards from the `.syl` pipeline and its lowering to core, and
  no `Cargo.toml` outside `v1/` names a v1 crate.
- What is still used from `sylex_plugin_v1` is gone from it: `deck.rs` is rehomed in a current
  crate or replaced, and `provenance.rs` is replaced by lowering-time refs
  (`engine-ability-origin-refs`), not rehomed.
- `v1/` is deleted, with its workspace members, its `[profile.dev.package.sylex_pipeline_v1]`
  entry, its `xtask` commands and the CI steps that generate and build `v1/plugins/wizards`.
- No file outside `v1/` names a `v1/` path or a v1 crate: the engine's `build.rs`, the tests and
  examples that load v1 plugins, the doc comments in `crates/core` that point at v1 RON macros,
  and the comments in `crates/card/Cargo.toml`.
- The demo and test decks the engine, TUI and matchups use still load.

**Pointers.** `v1/`, the workspace `Cargo.toml`, `crates/{card,engine,matchups,tui,xtask}/Cargo.toml`,
`crates/engine/build.rs`, `.github/workflows/ci.yml`, `docs/decisions/architecture.md`.

---
needs: [v1-retirement]
---
# Delete `ParamShape` and `KeywordDecl` from core

**Why.** `sylex_core::ParamShape` and `sylex_core::KeywordDecl` (`crates/core/src/keyword.rs`,
re-exported from `crates/core/src/lib.rs`) are read by nothing in the current line. Their only
consumers are v1: `sylex_plugin_v1`'s keyword registry (`Plugin.keywords` and `keyword_shape` in
`v1/crates/plugin/src/plugin.rs`, pinned by `v1/crates/plugin/tests/builtin.rs`) and
`sylex_lowering_v1`'s `Lower` impls (`v1/crates/lowering/src/keyword.rs`). Once v1 is deleted,
they are orphans.

**Done when.**
- `ParamShape`, `KeywordDecl`, their re-exports and their round-trip test are gone from
  `sylex_core`, and nothing outside `docs/` names either type.
- `cargo test -p sylex_core` and `cargo clippy -p sylex_core --tests -- -D warnings` pass.

**Pointers.** `crates/core/src/keyword.rs`, `crates/core/src/lib.rs`.

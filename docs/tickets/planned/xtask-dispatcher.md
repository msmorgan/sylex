---
needs: []
---

# Split xtask into per-tool binaries behind a dispatcher

**Why.** `xtask` is one crate that depends on every library in the workspace, including the v1 line,
the English grammar and the catalogs. Running any subcommand, even `cite check`, compiles and links
all of them. Each tool should pay only for what it uses.

**Done when.**
- `cargo xtask <tool> …` works as before for every subcommand, through a thin `xtask` dispatcher
  that runs the tool's own binary.
- Each tool, or group of tools that share dependencies, is its own crate under `crates/xtask/`, with
  only the dependencies it uses. Shared helpers live in a small common crate.
- The dispatcher itself has no `sylex_*` or v1 dependencies.
- `cargo tree -p` on the `cite` tool's crate shows no v1, English, vocabulary or catalogs crate. The
  landing reports which crates each tool depends on.
- Tools that exist only for v1 are grouped so that deleting `v1/` deletes them with it.
- CI and `cargo-xtask` invocations elsewhere in the repo still work. `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for every new crate.

**Not in scope.** Deleting or rewriting any tool. Changing a tool's command-line interface.

**Pointers.** `crates/xtask/Cargo.toml`, `crates/xtask/src/bin/cargo-xtask.rs`,
`crates/xtask/src/`, `.cargo/config.toml`, `.github/workflows/ci.yml`.

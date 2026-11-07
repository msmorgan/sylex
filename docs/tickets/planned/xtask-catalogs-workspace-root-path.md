---
needs: []
---

# The catalog output check finds the workspace root at run time

**Why.** Before writing a catalog, `crates/catalogs/src/io.rs` refuses an output directory that is
the workspace root or one of its ancestors. It finds the root by canonicalizing
`env!("CARGO_MANIFEST_DIR")/../..`, a path baked in at compile time. This repo runs several jj
workspaces under `.workspaces/` and shares a prewarmed `target/` between them, so a binary built in
one tree often runs from another whose baked path does not exist; the canonicalize fails and
callers turn it into a panic. That is how `catalogs::tests` cases in `cargo test -p xtask --lib`
fail in some workspaces and pass in others.

**Done when.**
- The workspace root is resolved at run time from the tree the command runs in (for example from
  the current directory's ancestry or a root the caller passes), not from a compile-time path.
- When no root can be resolved, the check fails with an error that says so; it never silently
  skips the ancestor check.
- The check still refuses an output directory that is the workspace root or an ancestor of it,
  with a test.
- `cargo test -p xtask --lib` passes in a freshly provisioned workspace, in the default checkout
  and in a workspace whose `target/` came from another checkout.

**Pointers.** `crates/catalogs/src/io.rs`, `crates/xtask/src/catalogs.rs`.

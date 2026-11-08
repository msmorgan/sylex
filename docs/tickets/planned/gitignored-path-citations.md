---
needs: []
---
# Tracked code never cites a gitignored path

## Why

A comment or message in tracked code that cites a gitignored path (`docs/memory/`,
`docs/evidence/`, `docs/cleanup/`, `.superpowers/`) or a nonexistent one (`docs/superpowers/`)
dangles in a fresh checkout. One such citation remains in code that is not being deleted:
`v1/crates/macro_ron/src/frames.rs` cites a `docs/superpowers/research/…` file for corpus
counts the comment already states. Nothing stops a new one from landing.

## Done when

- The `frames.rs` comment keeps its fact and drops the dangling path.
- A standing check (an `xtask` lint run in CI) fails when a tracked file under `crates/`,
  `plugins/`, `v1/` or `scripts/` cites a gitignored path. Code that names such a path as data
  (the scanner's exclusion list in `crates/xtask/src/coverage.rs`, `scripts/provision-workspace`
  sharing the ignored directories) is not a citation, and the check tells the two apart without a
  per-site allowlist.
- The check passes on the tree.

## Not in scope

- `lean/`, which cites `docs/evidence/` and is being deleted (`lean-retirement`).
- Tickets and other docs citing shared local docs.

## Pointers

- `rg -n 'docs/(superpowers|memory|evidence|cleanup)|\.superpowers' crates plugins v1 scripts`
- `.gitignore`, `.github/workflows/ci.yml`, `crates/xtask/src/lib.rs`

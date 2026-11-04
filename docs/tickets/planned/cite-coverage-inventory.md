---
needs: []
---

# Citation coverage is an inventory, not a floor

**Why.** `cargo xtask cite coverage --check` fails CI when the set of cited CR rules shrinks below
the floor in `cr-coverage.lock`. That is a threshold: deleting `lean/` and rewriting semantics
will shrink it legitimately, and a floor pushes agents to add citations just to clear it. The
count also includes markdown, where a citation is a reference, not coverage.

**Done when.**
- Coverage counts only implementation sources: Rust in `crates/` (code and tests) and `.syl` in
  `plugins/`. Markdown, and other non-implementation sources, are left out of coverage only;
  `cite check` and `cite audit` still read markdown.
- `cr-coverage.lock` records the current covered set, and `cite bless` rewrites it.
- The CI step "cr coverage ratchet" (`.github/workflows/ci.yml`) is replaced by a check that the lock
  matches the sources, i.e. that bless was run. It never fails because the set shrank.
- Lost rules appear as removed lines in the lock's diff.
- `cargo test -p xtask` and `cargo clippy -p xtask --tests -- -D warnings` pass. The landing reports
  the covered-set size before and after the markdown exclusion.

**Not in scope.** Restoring citations the exclusion drops. Changes to `cite check` or `cite audit`.

**Pointers.** `crates/xtask/src/coverage.rs`, `crates/xtask/src/cite.rs`, `cite-config.json`,
`cr-coverage.lock`, `.github/workflows/ci.yml`.

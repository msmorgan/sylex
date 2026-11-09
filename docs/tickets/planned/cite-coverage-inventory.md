---
needs: []
---

# Citation coverage is an inventory, not a floor

## Why

`cargo xtask cite coverage --check` fails CI when the set of rules cited at the `Bound` tier
shrinks below the set in `cr-coverage.lock`. That is a threshold: deleting `v1/` will shrink it
legitimately, and a floor pushes agents to add citations just to clear it. `Bound` is every
scanned file outside `docs/` that is not Markdown, so v1 `.ron` macros and the `v1/` crates count
as coverage today. Markdown is already only `Mentioned` and stays out of
the lock.

## Done when

- Coverage counts only implementation sources: Rust in `crates/` (code and tests) and `.syl` in
  `plugins/`. `cite check` and `cite audit` keep reading every source they read now.
- `cr-coverage.lock` records the current covered set, and `cargo xtask cite bless` rewrites it
  along with `cr-citations.lock`.
- The CI step "cr coverage ratchet" (`.github/workflows/ci.yml`) is replaced by a check that the
  lock matches the sources, i.e. that bless was run. It never fails because the set shrank, and
  lost rules show as removed lines in the lock's diff.
- The landing reports the covered-set size before and after the narrowing, and lists the rules it drops so
  steps 7 and 8 of the cleanup plan can restore them.

## Not in scope

- Restoring citations the narrowing drops.

## Pointers

- `crates/xtask/src/coverage.rs` (`tier_for_path`, `scan_files`, `check_ratchet`, `run`),
  `crates/xtask/src/cite.rs` (`command_bless`)
- `cite-config.json`, `cr-coverage.lock`, `.github/workflows/ci.yml`

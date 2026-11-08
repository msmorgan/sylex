---
needs: []
---

# Split `trigger.rs` into a `trigger/` module

## Why

`crates/engine/src/trigger.rs` is 6.7k lines, the largest engine file: about 1,620 lines
of `impl GameState` trigger code and 5,100 of embedded tests. Code and tests cover several
distinct subsystems in one file.

## Done when

- `trigger.rs` is a `trigger/` directory module. Its code is cut along method-cluster seams, each
  submodule with its own `impl GameState` block (no traits, no forwarding shims); paths used from
  outside as `crate::trigger::…` still resolve, through `pub(crate) use` re-exports where needed.
- Every test that can go through the engine's public API lives in `crates/engine/tests/`; a test
  that needs private access stays in the `mod tests` of the submodule that owns what it exercises,
  with no per-submodule `tests.rs` file. A submodule whose remaining inline tests are still large,
  or one part of which needs exhaustive testing of its own, is split further.
- The number of `#[test]` functions across `src/` and `tests/` is the same before and after, and
  the sorted multiset of comment lines matches, so no authored comment is dropped.
- `cargo +nightly fmt --check` passes.

## Pointers

`crates/engine/src/trigger.rs`, `crates/engine/tests/`. Known pitfalls from moving
tests out of a large `mod tests`: the module had `use` lines interleaved between test functions,
so collect every module-level `use` first and treat them as the header of each destination;
section-banner comments between test clusters are easy to orphan; and
`cargo fix --lib --tests --allow-dirty --allow-no-vcs` prunes the unused imports (jj workspaces
have no `.git`).

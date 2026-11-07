---
needs: []
---

# Split `step/mod.rs` along its method clusters

**Why.** `crates/engine/src/step/mod.rs` is 3,405 lines: about 2,380 of `impl GameState` step
methods and about 1,020 of tests. The sibling files in `step/` hold only the `EventApply` impls,
so the step methods themselves were never split.

**Done when.**
- The `impl GameState` step methods live in `step/` submodules cut along their concept clusters,
  and `crate::step::…` paths used outside the module still resolve.
- Every test that can go through the engine's public API lives in `crates/engine/tests/`; a test
  that needs private access stays in the `mod tests` of the submodule that owns what it exercises.
  No submodule gains a `tests.rs` file or becomes a directory module just to hold its tests.
- The number of `#[test]` functions across `src/` and `tests/` is the same before and after, and
  no authored comment is lost.

**Pointers.** `crates/engine/src/step/`, `crates/engine/tests/`, `split-trigger-rs` (the sibling
split, with known pitfalls).

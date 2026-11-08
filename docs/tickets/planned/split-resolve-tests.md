---
needs: []
---

# Move the `resolve/` test suites out of their logic files

## Why

`crates/engine/src/resolve/action.rs` (6.4k lines) and `resolve/effect.rs` (7.3k) are
large because of their embedded `mod tests`, not their logic: `action.rs`'s logic ends near line
1,030 and `effect.rs`'s near 1,890. The logic is reasonably sized, but a reader of either file has
to scroll past thousands of test lines. `player_action.rs` (tests from line ~920 of 3.9k) and
`count.rs` (~710 of 2.7k) have the same shape.

## Done when

- Every test in `action.rs`, `effect.rs`, `player_action.rs` and `count.rs` that can go through
  the engine's public API lives in `crates/engine/tests/`; a test that needs private access stays
  in its module's `mod tests`.
- No module becomes a directory module or gains a `tests.rs` file just to hold its tests. A module
  whose remaining inline tests are still large, or one part of which needs exhaustive testing of
  its own, is split along its concept boundaries.
- Inline tests share helpers through the existing `#[cfg(test)]` `test_support` and
  `resolve/fixtures.rs` modules, not a new fixtures module.
- Test names and CR-citation text are unchanged, the number of `#[test]` functions across
  `src/` and `tests/` is the same before and after, and no authored comment is lost (the sorted
  multiset of comment lines matches).

## Pointers

`crates/engine/src/resolve/`, `crates/engine/src/test_support.rs`,
`crates/engine/tests/`, `split-trigger-rs` (the same kind of move, with known pitfalls).

---
needs: []
---

# The xtask forest tests follow the current parses

## Why

Two `xtask` tests fail on the default line, because they pin derivation counts and fork options
of real units and the parses have since changed:

- `english::forests::forks::tests::wide_and_narrow_attachments_over_a_coordination` expects
  Sands of Time's wide and narrow attachments over 100 Derivations, and the parse now has 50.
- `english::forestwalk::tests::settled_siblings_fold_as_a_large_forest_narrows_and_under_limit`
  (Remove Enchantments, line 1) still narrows to 2 Derivations under its picks and exclusion,
  which differ in whether *your opponents control* modifies *Auras*#4 or *creatures*#2, but its
  settled spine no longer ends in `head: Nominal "other Auras attached to attacking creatures
  your opponents control": 2 alternatives`.

## Done when

- For each unit, the change in the parse is identified and judged correct or a defect. A
  correct change is pinned in the test, and a defect is fixed in the grammar.
- `cargo test -p xtask` passes.

## Pointers

- `crates/xtask/src/english/forests/forks.rs` (its tests module),
  `crates/xtask/src/english/forestwalk/tests.rs`.
- `cargo xtask english forestwalk --card 'Sands of Time'`,
  `cargo xtask english forestwalk --card 'Remove Enchantments' --line 1`.

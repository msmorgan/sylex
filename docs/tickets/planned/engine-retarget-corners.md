---
needs: [engine-copy-spells]
---
# Retargeting a committed stack entry: becomes-the-target and Flagbearer

## Why

The `Retarget` handler (`crates/engine/src/decide/pending/cast.rs`), which chooses new targets for
a committed stack entry [CR#115.7d], writes `entry.targets = chosen` and emits nothing. A target
newly chosen this way is targeted by that spell: Deflection's ruling (2004-10-04) says "the new
target is considered to be targeted by the deflected spell. This will trigger any effects which
trigger on being targeted." So a changed slot must fire becomes-the-target abilities as it would
at announcement [CR#601.2c], and an unchanged slot must not.

The engine already ignores Flagbearer when targets change: `Must(Target)` rows are checked only
on the announcement path (`ChooseTargets` in the same file). Coalition Honor Guard's ruling
(2016-06-08) confirms this: "If a spell or ability's targets are changed, or if a copy of a spell
or ability is put onto the stack and has new targets chosen, it doesn't have to target a
Flagbearer." Nothing pins that behavior.

## Done when

- The handler compares old and new targets slot by slot and emits `BecameTarget` (one per
  distinct object, as at announcement) only for objects that a changed slot now targets.
- Tests: Ward or Angelic Cub fires when a spell's target is changed to it, and does not fire for
  a slot left unchanged.
- A regression test: retargeting with a Coalition Honor Guard on the battlefield may choose a
  non-Flagbearer target.

## Not in scope

- Copies. A copy is a new spell put onto the stack with its targets [CR#707.10,707.10c], so every
  target of a copy becomes a target whether or not it changed (Dormant Gomazoa ruling,
  2010-06-15). The copy path must not use this diff; copy targeting is
  `engine-spell-copy-stack-lifecycle`. The Flagbearer test for a copy with new targets waits for
  that copy path: a non-`AsIs` `CopySpell` returns no work today.

## Pointers

- `crates/engine/src/decide/pending/cast.rs` (`Retarget`, `ChooseTargets`)
- `crates/engine/src/resolve/player_action.rs` (`Action::Retarget`), `crates/engine/src/step/mod.rs`
  (`open_choose_new_targets`)
- `crates/engine/tests/stack.rs` (the `flagbearer_*` tests)

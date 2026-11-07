---
needs: [core-regions-substrate]
---

# Engine: `Condition::Matches` tells a current-state read from a last-known-information read

## Why

`Condition::Matches` (`crates/engine/src/condition.rs`) falls back to the bound snapshot
whenever the referenced object is gone, whatever the condition asks. That is right for a dies
trigger or an intervening "if" that inspects a departed object, but wrong for a current-state
guard: the snapshot matcher answers "on the battlefield" with the zone the object left, so a
departed object still counts as a permanent. Affectionate Indrik ("When this creature enters,
you may have it fight target creature you don't control") is the witness: if it leaves before
the trigger resolves, neither creature fights or deals damage [CR#701.14b].

## Done when

- Condition and reference evaluation carry which kind of read they are. The snapshot fallback
  is not turned off globally.
- Only a read entitled to last-known information consumes a snapshot: a look-back trigger's
  event, or a trigger condition about a departed event object [CR#603.10a,603.4]; or an effect
  that needs information from a specific object that has left the public zone it was expected
  in, including the ability's own source [CR#608.2h,113.7a].
- A current-state read never consumes a snapshot: a membership test such as "on the battlefield"
  or "is a creature" used as an action's guard is false for an object that has left.
- An Affectionate Indrik test: Indrik leaves before its fight trigger resolves; the
  both-creatures guard fails, no power is read, and no damage or fight event occurs.
- A dies-trigger test whose condition reads the departed object's snapshot still passes, so both
  sides of the distinction are pinned.

## Pointers

- `crates/engine/src/condition.rs` (`Condition::Matches`)
- `crates/engine/src/trigger.rs` (`filter_matches_snapshot_with_activation`, its `InZone` arm)
- `crates/engine/src/resolve/query.rs` (`eval_reference_product`)

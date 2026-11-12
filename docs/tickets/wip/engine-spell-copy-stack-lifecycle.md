---
needs: []
---
# Engine: a spell copy's targets become targeted, and the copy leaves the stack like a spell

## Why

Two departures in how the engine handles a copy of a spell:

1. **Copies never make anything "become the target".** `Copied::apply`
   (`crates/engine/src/step/stack.rs`) clones the original's targets and emits only
   `GameEvent::Copied`. The only producer of `BecameTarget` is `crates/engine/src/decide/pending/cast.rs`,
   at announcement. A copy of a spell is itself a spell with the original's targets [CR#707.10],
   put onto the stack "with those targets" [CR#707.10c]. So an object targeted by a copy becomes
   the target of a spell, and ward [CR#702.21a] and "becomes the target" abilities should trigger.
   Rulings on Dormant Gomazoa and Angelic Cub say so. Today Archive Dragon's ward never triggers on
   a storm copy of Astral Steel.
2. **Spell copies vanish on the stack instead of going to the graveyard.** On resolution and on a
   fizzle (`crates/engine/src/resolve/mod.rs`), and when countered (`crates/engine/src/step/stack.rs`),
   a copy leaves through `AbilityResolved`/`AbilityCountered` with no zone move. An instant or
   sorcery spell is put into its owner's graveyard as the final part of its resolution
   [CR#608.2n], and a countered spell is put into its owner's graveyard [CR#701.6a]. A copy is
   then a copy of a spell in a zone other than the stack, and it ceases to exist as a state-based
   action [CR#704.5e,707.10a]. Only watchers and replacements of "a spell is put into a graveyard"
   can see the difference.

## Done when

- `Copied::apply` emits one `BecameTarget` per distinct target of the copy, with the copy as the
  spell, deduplicated per object as `cast.rs` does. When the copy path that lets the controller
  choose new targets is built (`resolve/player_action.rs` handles only `CopyRetarget::AsIs` today),
  it emits for every final target. `engine-retarget-corners`' rule that an unchanged slot doesn't
  re-trigger applies only to changing the targets of an existing stack object [CR#115.7], never to
  a copy.
- A resolved, countered or fizzled instant or sorcery copy moves Stack→Graveyard with a
  `ZoneChange`. The copy-cease state-based action in `crates/engine/src/sba.rs` (stack-only today)
  then removes it from the graveyard.
- The copy-cease sites cite [CR#704.5e,707.10a] instead of [CR#704.5d] (the token rule):
  `crates/engine/src/sba.rs`, `crates/engine/src/event.rs`, `crates/engine/src/resolve/action.rs`,
  `crates/core/src/action.rs` (`Action::Cease`, which keeps [CR#704.5d] only if token ceasing goes
  through it). The `AbilityCountered` doc in `crates/engine/src/event.rs` cites [CR#608.2b] for a
  fizzle and [CR#608.2a] for an intervening-if removal, not [CR#608.2n]. The `StackEntry::copy`
  doc in `crates/engine/src/stack.rs` stops saying a copy "vanishes instead of moving zones".
- Tests in `crates/engine/tests/stack.rs` that pin the vanish model
  (`resolved_copy_vanishes_without_zone_move`, `countered_copy_vanishes`, and the docs repeating
  it) are rewritten to assert the graveyard move and then the cease.
- New tests: a storm copy targeting a creature with ward triggers ward (Astral Steel and Archive
  Dragon); a resolved instant copy produces a Stack→Graveyard `ZoneChange` and is gone after the
  next state-based action check.

## Not in scope

- Stamping a countering cause on fizzles: `engine-resolution-illegal-targets`.
- A copy of a permanent spell, which becomes a token as it resolves [CR#707.10f].

## Pointers

- `crates/engine/src/step/stack.rs` (`Copied::apply`, the counter path).
- `crates/engine/src/resolve/mod.rs` (`resolve_object`'s spell arm), `crates/engine/src/sba.rs`.
- `plugins/builtin/definitions/keyword_ability/storm.syl`.

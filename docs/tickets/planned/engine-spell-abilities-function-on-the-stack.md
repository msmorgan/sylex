---
needs: []
---
# Engine: an instant or sorcery spell's triggered abilities function on the stack

## Why

Abilities of an instant or sorcery spell usually function only while it is on the stack [CR#113.6],
and storm is a triggered ability that functions on the stack [CR#702.40a]. The trigger scan in
`crates/engine/src/trigger.rs` (`scan_event`) watches battlefield, graveyard, hand and command-zone
emblem objects only, and its default function zone is the battlefield for every non-emblem object.
So no stack object ever triggers: casting Weather the Storm or Astral Steel after another spell
makes no storm copies, and no engine test exercises storm.

## Done when

- The trigger scan watches spells on the stack, and an instant or sorcery's triggered ability with
  no stated zone functions from the stack [CR#113.6]; an ability that states its zones still
  functions only there [CR#113.6b].
- Storm triggers on its spell's cast and makes one copy per other spell cast before it this turn
  [CR#702.40a]; a spell cast in response to the storm trigger is not counted.
- Tests in `crates/engine/tests/stack.rs`: Weather the Storm cast third in a turn makes exactly two
  copies; a storm copy of Astral Steel targeting Archive Dragon triggers ward (replacing the
  copier in `spell_copy_targeting_a_ward_creature_triggers_ward` or alongside it).

## Not in scope

- Choosing new targets for storm copies [CR#707.10c]: `engine-copy-target-variants`.

## Pointers

- `crates/engine/src/trigger.rs` (`scan_event`: the watcher set and `default_zone`).
- `v1/plugins/builtin/macros/keyword/Storm.ron`, `plugins/builtin/definitions/keyword_ability/storm.syl`.
- `docs/tickets/done/engine-storm.md`.

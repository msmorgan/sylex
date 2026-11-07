---
needs: []
---

# Core has no card scope for designations, so the commander is object state

## Why

The commander designation "is not a characteristic of the object represented by the card; rather,
it is an attribute of the card itself. The card retains this designation even when it changes
zones" [CR#903.3]. `.syl` declares it that way (`commander.syl`: `scope = heldByCard`), but core's
`DesignationScope` has only `Object`, `Player` and `Game`. The v1 row the engine runs today
(`v1/plugins/builtin/macros/designations/Commander.ron`) declares `scope: Object` and buys
zone-independence from `persistence: Permanently` instead.

It is card state, not object state: [CR#400.7]'s exceptions have no commander entry, effects read
a commander's characteristics in every zone, library and hand included [CR#903.3e], and the
designation is fixed at deck construction [CR#903.3,903.3a]. Yet the queries are per object: a
melded or merged permanent containing the commander is the commander [CR#903.3b,903.3c], and
"control", "cast" and "in a zone" read the permanent, the spell or the card [CR#903.3d]. A deck
can have two commanders [CR#702.124a], so the designation is not unique.

## Done when

- Core represents a card-held designation, and the engine answers [CR#903.3d]'s three query forms
  and [CR#903.3e]'s hidden-zone read from it, including the meld and merge cases.
- Engine tests cover a commander changing zones and still being the commander, and a
  characteristic read in the library or hand.

## Not in scope

- `v1/`: its Commander row and the test pinning `DesignationScope::Object`
  (`v1/crates/plugin/tests/builtin.rs`) go when `v1/` is deleted.

## Pointers

- `crates/core/src/designation.rs` (`DesignationScope`, `DesignationDef`)
- `plugins/builtin/definitions/designation/commander.syl`
- `crates/engine/src/state.rs` (`DesignationStore`)

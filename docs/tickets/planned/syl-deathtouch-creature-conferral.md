---
needs: [lean-v3-action-clause-spine, lean-generated-canon-modules, lean-retirement, cleanup-step6-surface]
---
# Deathtouch's state-based action is a creature-type conferral

## Why

The creature card type confers its own state-based actions: the zero-toughness row [CR#704.5f]
and the lethal-damage row [CR#704.5g] are `stateBased` conferrals in
`plugins/builtin/definitions/card_type/creature.syl`; the lethal-damage row destroys with
"destroy", which regeneration can replace. The deathtouch row [CR#702.2b,704.5h] is the same kind
of rule about creatures but is written apart, as a standalone `sba` rule scoped by `isCreature`
(`plugins/builtin/rules/sba/deathtouchDestruction.syl`). Deathtouch's own declaration has an
empty body because its rule lives in that row; its combat-damage assignment rule [CR#702.2c] stays
with the engine.

Written against the surface (`cleanup-step6-surface`).

## Done when

- The deathtouch row is a `stateBased` conferral on `creature.syl` beside the [CR#704.5g] row,
  with the same condition: toughness greater than 0, and dealt damage by a source with deathtouch
  within the window from the last state-based action check to now (`within(durationBetween(
  stateBasedActionsChecked, now))`).
- Its citations and comment ([CR#702.2b,704.5h], and [CR#702.2d] for reading the source from any
  zone) move into the creature declaration's header comment.
- `plugins/builtin/rules/sba/deathtouchDestruction.syl` is deleted, and nothing names it.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

- `plugins/builtin/definitions/card_type/creature.syl`
- `plugins/builtin/rules/sba/deathtouchDestruction.syl`
- `plugins/builtin/definitions/keyword_ability/deathtouch.syl`
- `docs/decisions/semantics.md` §4.8 (Lookback), §5 (state-based actions as Conferrals)

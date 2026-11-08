---
needs: []
---

# Classify the WC99 probe's damage by cause, not by the source's zone

## Why

The WC99 probe (`crates/matchups/src/probe.rs`) sorts damage into its buckets by
whether the source is on the battlefield when the batch is observed. Its comment ("fine while the
fixture has no combat trades; revisit with blockers") is stale: both strategies now declare
`Block(BlockAll)`, so trades happen every batch. A combat attacker that died in the batch counts
as `spell_damage_to_creatures`, Mogg Fanatic's sacrifice-ability damage counts as
`spell_damage_to_players`, and an on-battlefield Cursed Scroll's ability damage counts as
`creature_damage_to_players`. So the "burn never went to the face" check in `tests/wc99.rs` can be
satisfied by ability damage.

## Done when

- The probe classifies each damage event by what dealt it: a spell, an ability, or combat.
- The stale comment is gone.
- `tests/wc99.rs`'s expected buckets are re-derived, and a test pins at least the Mogg Fanatic
  and dead-attacker cases.

## Pointers

`crates/matchups/src/probe.rs`, `crates/matchups/tests/wc99.rs`,
`crates/matchups/strategies/{sped_red,stompy}.ron`.

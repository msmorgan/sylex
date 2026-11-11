---
needs: [core-saga-chapters, engine-counters-api]
---
# Engine: Saga lore counters, the full final-chapter sacrifice, and read ahead

## Why

The engine fires chapter abilities on crossed lore thresholds [CR#715.2b] and sacrifices a
finished Saga through a `Property::StateBased` conferral [CR#715.4]. The rest of a Saga's life is
not executed:

- No executor runs `Property::TurnBased` (its doc in `crates/core/src/property.rs` says so), so the
  lore counter a player puts on each of their Sagas with chapter abilities as their precombat main
  phase begins [CR#715.3c] never arrives.
- The sacrifice condition lacks [CR#715.4]'s "and it isn't the source of a chapter ability that
  has triggered but not yet left the stack": the engine's Saga row (`saga_subtype()` in the
  `crates/engine/src/sba.rs` tests) checks only the lore count against the final chapter number,
  so a Saga at its last chapter is sacrificed before that chapter resolves.
- Read ahead is unbuilt: choosing a starting lore count between one and the final chapter number
  as the Saga enters [CR#702.155b,715.3b], and chapter abilities not triggering the turn it entered
  unless the lore count equals their chapter number [CR#702.155a].
- No engine test covers a Saga without read ahead entering with one lore counter [CR#715.3a].

## Done when

- `sylex_engine` tests cover, with real Sagas (History of Benalia; The Elder Dragon War
  for read ahead):
  - a Saga enters with one lore counter, and gains one more as its controller's precombat main
    phase begins (not the opponent's), with no stack object;
  - at its final chapter, the Saga stays on the battlefield while that chapter ability is on the
    stack and is sacrificed once it has left the stack;
  - a read-ahead Saga entering at chapter III triggers only chapter III that turn, and triggers
    normally on later turns.

## Pointers

- `plugins/builtin/definitions/subtype/enchantment/saga.syl` and
  `plugins/builtin/definitions/keyword_ability/readAhead.syl`: the rules as `.syl` states them.
- `crates/core/src/property.rs` (`TurnBased`, `StateBased`), `crates/core/src/count.rs`
  (`GreatestWatchedThreshold`), `crates/core/src/condition.rs` (the chapter crossing gate).
- `crates/engine/src/step/mod.rs` (`turn_based_actions`), `crates/engine/src/sba.rs`,
  `crates/engine/src/trigger.rs` (`saga_chapters_fire_on_crossed_thresholds_from_one_batch_fact`).

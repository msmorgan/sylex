---
needs: []
---
**Core carries the characteristic-defining mark on a static ability, and the engine applies such
abilities first in their layer.**

## Why

A characteristic-defining ability [CR#604.3,604.3a] applies in layer 7a when it defines power or
toughness [CR#613.4a], ahead of effects that set them [CR#613.4b], and first within layers 2–6
for the other characteristics [CR#613.3]. The engine's layer pass already orders by
`ContinuousEffect::is_cda`, but every site that builds a continuous effect from a static ability
sets `is_cda: false`, because nothing in core marks a static as characteristic-defining. `.syl`
already does (`characteristicDefining`, behind `definesPt` and its siblings), so Tarmogoyf and
Maro get correct values today but land in layer 7b, wrong as soon as another P/T effect applies.

## Done when

- Core's static-ability grammar can mark a static as characteristic-defining, and the engine's
  gather sites build `is_cda: true` effects from marked statics; unmarked statics are unchanged.
- A Tarmogoyf under Glorious Anthem has its count plus one, and a Tarmogoyf hit by Turn to Frog
  is 1/1: the 7b set applies after the 7a definition.
- A marked color CDA applies before other layer-5 effects in the same layer pass [CR#613.3].
- Tests use the real cards, by core fixture where they are not authored.

## Pointers

- `crates/engine/src/layer.rs`: `ContinuousEffect::is_cda`, `layer_of`, the gather sites that set
  `is_cda: false`, and `cda_sets_pt_to_dynamic_creature_count`.
- `crates/core/src/continuous.rs`: `NumericOp`, `Modification`.
- `plugins/builtin/primitives/static/characteristicDefining.syl`,
  `plugins/canon/cards/Tarmogoyf.syl`, `plugins/canon/cards/Maro.syl`.
- `docs/tickets/planned/engine-copy-cda-generalize.md`: its copy-exception drop can read this
  mark instead of matching ability shapes.

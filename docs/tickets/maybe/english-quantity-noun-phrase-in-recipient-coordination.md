---
needs: []
---
# Eternal Flame coordinates a recipient with an amount of damage

## Why

Eternal Flame prints *deals X damage to target opponent or planeswalker and half X damage,
rounded up, to you*. Its correct Readings coordinate two Complement clusters: *[X damage to
target opponent or planeswalker] and [half X damage, rounded up, to you]*. Two more Readings
take *target opponent or planeswalker and half X damage* as one coordinated Noun Phrase, the
Complement of the first *to*, which leaves *to you* a second recipient of *X damage*.

## Open questions

- What refuses the coordination: the conjuncts' unlike Noun Kinds (a player or permanent beside
  a measured amount), or a Targeting Noun Phrase coordinated with a non-Targeting one? Which
  real cards coordinate such unlike Noun Phrases correctly?

## Done when

- Eternal Flame has only its cluster-coordination Readings.
- The census shows no verdict change on the supported corpus, and every unit whose Readings
  change is checked.

## Pointers

- `crates/english/grammar/english.fug`: the `NounPhrase` coordination schema,
  `DeterminedQuantity`.
- `crates/english/judgments/targeting_projection.txt`, the Eternal Flame entry.
- `docs/tickets/done/english-multiplied-degree-quantity.md`.

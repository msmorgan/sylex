---
needs: [engine-snapshot-predicate-breadth]
---

# Engine: a zone-change snapshot keeps the attacking designation, so "if it was attacking" can be asked of a creature that died

## Why

A creature that leaves the battlefield is removed from combat [CR#506.4], so by the time a dies
trigger resolves, the live combat registry no longer lists it. Garna, Bloodfist of Keld
("Whenever another creature you control dies, draw a card if it was attacking. Otherwise, Garna
deals 1 damage to each opponent.") asks exactly that. The read reaches the snapshot matcher
(`filter_matches_snapshot_with_activation` in `crates/engine/src/trigger.rs`), which has no
`StatePredicate::Attacking` arm and falls into its catch-all `todo!`.

## Done when

- The snapshot captures the attacking designation the creature held immediately before it left
  [CR#506.4,603.10a], read from the combat registry before zone-change cleanup, and
  `StatePredicate::Attacking` over a snapshot reads it. It is not inferred from the zone the
  object left, and not read from the object the move created.
- Garna tests: a creature that dies while attacking draws a card; one that dies outside combat
  makes Garna deal 1 damage to each opponent; a snapshot of a non-attacking creature does not
  match `Attacking`. None reaches the catch-all.

## Not in scope

- The other combat predicates (`Blocking`, `Unblocked`), unless the shared representation makes
  them an inseparable, tested addition.

## Pointers

- `crates/engine/src/trigger.rs` (`filter_matches_snapshot_with_activation`)
- `crates/engine/src/lki.rs`; `crates/engine/src/combat.rs` (the combat registry)

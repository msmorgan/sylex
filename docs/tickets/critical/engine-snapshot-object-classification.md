---
needs: [engine-snapshot-predicate-breadth, engine-lki-robustness]
---

# Engine: object-class (token, nontoken) and subtype predicates evaluate against zone-change snapshots

## Why

`filter_matches_snapshot_with_activation` (`crates/engine/src/trigger.rs`) has no arm for
`Predicate::Class` or `CharacteristicPredicate::Subtype`, so both fall into its catch-all
`todo!`. Dies and enters triggers filtered on them abort: Midnight Reaper ("Whenever a nontoken
creature you control dies"), Valkyrie's Call ("nontoken, non-Angel creature"), Kalastria
Highborn ("another Vampire you control dies"), Gate Colossus ("Whenever a Gate you control
enters", watched from the graveyard). The snapshot's card spine already holds what these need:
the `CardInstance.is_token` flag [CR#111.6] and the face's subtypes.

## Done when

- `Predicate::Class` and `CharacteristicPredicate::Subtype` over a snapshot read the pre-move
  object, never the object the move created. A token keeps its token identity though its card
  definition is synthesized. Subtypes come from the snapshot's derived values once
  `engine-lki-robustness` captures them, never from a silent printed-face substitute.
- The witnesses are these Foundations cards: Arahbo, the First Fang; Crossway Troublemakers; Gate
  Colossus; Gateway Sneak; High-Society Hunter; Infernal Vessel; Kalastria Highborn; Lathliss,
  Dragon Queen; Midnight Reaper; Spinner of Souls; Valkyrie's Call; Wildborn Preserver; Youthful
  Valkyrie. Tests built on them cover a positive and a negative subtype match, a nontoken and a token reaching a
  dies filter, and another token you control entering while a subtype-filtered trigger watches.
  None reaches the catch-all.

## Pointers

- `crates/engine/src/trigger.rs` (`filter_matches_snapshot_with_activation`)
- `crates/engine/src/object.rs` (`CardInstance.is_token`), `crates/engine/src/target.rs`
  (`is_object_class`, the live `Class` arm)

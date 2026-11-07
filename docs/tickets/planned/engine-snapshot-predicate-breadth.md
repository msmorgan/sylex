---
needs: []
---
# Engine: split the snapshot matcher's unanswered `Predicate` leaves

## Why

`filter_matches_snapshot_with_activation` in `crates/engine/src/trigger.rs` evaluates a
`Predicate` against the last known information of an object that has left its zone
[CR#603.10a]. It answers fewer leaves than the live matcher, and the rest reach a catch-all
`other => todo!(…)` arm. Those leaves are not one feature. They fall into three classes:

1. **Unwired arms**, with no data missing: `RelationPredicate::TeammateOf` [CR#102.3], a sibling
   of the existing `OpponentOf`/`Controls` arm. `CharacteristicPredicate::Subtype` and `Has` look
   like the same shape as the characteristic leaves that already read `snapshot_face`.
2. **Uncaptured data**: `StatePredicate::Designated`, `Attacking`, `Blocking` [CR#509.1a],
   `Unblocked` [CR#509.1h], `WasPaidWith` and `WasCastWith` [CR#118.9,702.34a]. `LkiSnapshot`
   (`crates/engine/src/lki.rs`) records none of these facts. This is the capture work
   `engine-lki-robustness` does.
3. **Open meaning**, where what the leaf should mean for an object that has left its zone is a
   decision: `Predicate::PlayerStatCmp`, `Predicate::FromSource`, `Predicate::Entity`,
   `Predicate::Class`, and `Predicate::Ref` for every `Reference` except `Reg` (`Single`,
   `OpponentOf`, `ControllerOf`, `Coalesce`, `OwnerOf`, `AttachHostOf`).

## Done when

- Class 1 is a ticket of its own, or is wired along with this ticket.
- Class 2 is carried into `engine-lki-robustness` with its list, or is a ticket of its own.
- Each class 3 leaf has a ticket, and the owner has ruled on which `Reference`s are legal in a
  trigger condition. A reference that is not legal there is rejected when the card is loaded
  rather than reaching the engine.
- The catch-all arm is replaced by explicit arms whose panics name the owning ticket.

## Pointers

- `crates/engine/src/trigger.rs` (`filter_matches_snapshot_with_activation`).
- `crates/engine/src/target.rs` (`matches_with`, the live matcher; `walk_combinators`).
- `crates/core/src/filter.rs` (`Predicate` and its leaf enums), `crates/core/src/reference.rs`.

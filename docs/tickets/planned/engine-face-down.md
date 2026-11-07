---
needs: []
---
# Face-down spells, permanents and exiled cards

## Why

The engine stores no face-down status. `Status::FaceDown` and `Status::FaceUp` trip a `todo!` in
`crates/engine/src/target.rs` and its snapshot twin in `crates/engine/src/trigger.rs`, the
`EnterRider::FaceDown` rider is an unbuilt seam in `crates/engine/src/copy.rs`, and
`SpecialAction::TurnFaceUp` is a shell `legal_actions` never offers. Core already carries the
face-down payload (`FaceDownSpec`, `FaceDownCharacteristics` in `crates/core/src/status.rs`), and
the `.syl` side declares morph, megamorph, disguise, manifest, cloak and exiling face down.

## Done when

- A permanent's face-up/face-down status is stored and read by the `Status` predicate in
  `target.rs` and `trigger.rs` [CR#110.5].
- A card can be cast face down with morph, megamorph or disguise [CR#702.37c,702.168b] and is
  face down before it is put on the stack [CR#708.4]; manifest and cloak put a card onto the
  battlefield face down [CR#701.40a,701.58a,708.3], through the `EnterRider::FaceDown` rider.
- Turning a face-down permanent face up is offered as a special action to its controller
  whenever they have priority and its cost can be paid [CR#116.2b,702.37e,702.168d].
- Hidden information holds: only the controller may look at a face-down spell or permanent
  [CR#708.5], and a face-down permanent or spell leaving the battlefield or stack is revealed
  [CR#708.9].
- A card exiled face down has no characteristics and can't be examined except as an instruction
  allows [CR#406.3,406.3a].
- Tests use real cards: Exalted Angel cast with morph and turned face up, Hidden Dragonslayer's
  megamorph counter, Bubble Smuggler's disguise ward, Whisperwood Elemental's manifest, Cryptic
  Coat's cloak, and Bomat Courier exiling face down.

## Not in scope

- Deriving a face-down permanent's characteristics in layer 1b [CR#613.2b,708.2]:
  `engine-layers-1-copy-facedown-text`.
- The special-action dispatch itself (offering, deciding, rendering): `engine-special-actions`.

## Pointers

- `crates/core/src/status.rs`: `Status`, `Face`, `FaceDownSpec`, `FaceDownCharacteristics`.
- `crates/engine/src/target.rs` and `crates/engine/src/trigger.rs`: the `Status` arms.
- `crates/engine/src/copy.rs`: `EnterRider::FaceDown`, `has_unbuilt_enter_rider`.
- `crates/engine/src/decide/mod.rs`: `SpecialAction::TurnFaceUp`.
- `plugins/builtin/definitions/keyword_ability/{morph,megamorph,disguise}.syl`,
  `plugins/builtin/definitions/keyword_action/{manifest,cloak,exileFaceDown}.syl`.
- Face-down spells and permanents [CR#708] in `data/rules/cr.txt`.

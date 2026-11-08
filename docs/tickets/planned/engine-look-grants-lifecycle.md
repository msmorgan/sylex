---
needs: []
---
**Look grants end when the looked-at cards are shuffled or reordered, reveals last only as long
as their effect needs, and the face-down exile look right is its own ledger.**

## Why

`look_grants` is keyed on `ObjectId`, and its doc comment claims grants "expire for free on
shuffle" because an id is reminted on every zone change. A shuffle is not a zone change:
`handle_shuffled` only permutes the library, so a scry grant follows the card to its random new
position [CR#701.24a]. Scry, surveil and fateseal put cards back with `reposition_library`, which
keeps the id and so the grant, though reordered cards become new objects [CR#701.20d,701.20e].
`GameEvent::Revealed` grants every viewer a permanent entry, so a card revealed from hand for a
cost stays visible to opponents while it stays in hand [CR#701.20a]. Nothing reads `look_grants`
as a permission yet except tests, but `engine-player-view` is built on it.

## Design

- A shuffle remints every card in the shuffled library, and a shuffled face-down exile pile is
  reminted the same way; a grant keyed on the old `ObjectId` ends with that id.
- An effect that puts looked-at or revealed library cards back in an order (scry, surveil's top
  pile, fateseal, random-bottom remainders) remints them in place, with no zone change and no
  `ZoneChange` event, including a card put back in the position it came from.
- Whatever `look_grants` keeps beyond a reveal's window [CR#701.20a] is the engine's knowledge
  model (who has seen what), not a rules permission.
- The [CR#406.3] look right is a separate ledger, written as part of the face-down exile and
  keyed on the exiled object, so it survives the remint at the exile. Only that ledger and the
  look rights cards confer gate casting from face-down exile [CR#601.3f] and choosing a specific
  face-down exiled card [CR#406.4].

## Done when

- `Shuffled(player)` remints every card in that library [CR#701.24a,701.20d], and the reorders
  in Design remint in place [CR#701.20e,701.20d]; the reminting code's doc comment records that
  a card put back where it came from is reminted too.
- A reveal grant ends when its window closes: a revealed card stays revealed only as long as
  needed to complete the parts of the effect it is relevant to, and a card revealed for a cost
  until its spell or ability leaves the stack [CR#701.20a]. `look_grants`' doc comment says what
  it keeps beyond that is a knowledge model.
- Once face-down exile exists (`engine-face-down`), the [CR#406.3] ledger exists and gates
  [CR#601.3f,406.4]: a player who looked at a card and then exiled it face down, or was allowed
  to look at a card exiled face down, keeps looking until it leaves exile or is part of a
  shuffled pile. The card-specific rights stay data: `hideaway` [CR#702.75a] and the
  `foretoldCardLook` conferral [CR#702.143d].
- `scry_reposition_keeps_id_and_fires_no_zone_change` is rewritten to assert a new id and still
  no `ZoneChange`.
- The comments that claim free expiry or a [CR#406.3] basis are corrected (`state.rs` on
  `look_grants`, `resolve/effect.rs` at the top-of-library peek), and peeks and reveals cite
  [CR#701.20e,701.20a].
- [CR#401.7] (a card put "Nth from the top" of a library with fewer than N cards) is cited only for
  that clamp (`event.rs` on `ZoneChange`, `step/mod.rs` at the library insert). It is removed
  from the reposition sites (`agenda.rs` `RepositionLibrary`, `step/mod.rs` `Repositioned` and
  `reposition_library`, `resolve/action.rs` on same-library moves and the scry test) and from
  the other anchor sites that are not about the clamp (`target.rs` `ordered_zone_position`,
  `resolve/action.rs` anchor helpers and tests, `resolve/effect.rs` arrange detection).
- Tests:
  - Scry 1 keeping the card on top: the scrying player's grant is gone after a shuffle.
  - A scried card put back gets a new id and fires no `ZoneChange`.
  - A card revealed from hand for a cost is not visible to the opponent once its spell leaves
    the stack.

## Pointers

- `crates/engine/src/state.rs`: `look_grants`, `payment_observations`.
- `crates/engine/src/payment.rs`: `grant_payment_look` (the only writer, also feeding payment
  observations); `crates/engine/src/payment/replay.rs`.
- `crates/engine/src/step/zone.rs`: `handle_shuffled`.
- `crates/engine/src/step/mod.rs`: `reposition_library`, the `GameEvent::Revealed` arm.
- `crates/engine/src/agenda.rs`: `WorkItem::RepositionLibrary`.
- `crates/engine/src/resolve/effect.rs`: the `Instruction::Each` top-of-library peek.
- `crates/engine/tests/payment_replay.rs`: the look-grant assertions.
- `plugins/builtin/definitions/keyword_ability/hideaway.syl`,
  `plugins/builtin/rules/conferral/foretoldCardLook.syl`.

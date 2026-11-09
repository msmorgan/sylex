---
needs: [syl-single-face-card-call]
---

# The look-back point comes from the trigger, not from `move`

## Why

`move` takes an `at: ObservationPoint` (`arrival` or `departure`), chosen by hand at each call.
The rules derive whether an event looks back from the kind of trigger [CR#603.10a], so the hand
choice can disagree with them, and it does: the Aura state-based row
(`plugins/builtin/definitions/subtype/enchantment/aura.syl`) moves with `departure`, while the
creature and other state-based rows use `putInto`, observed on `arrival`.

Written against the start of step 6 (`syl-single-face-card-call`).

## Open questions

- Where the look-back point is decided once `move` no longer carries it: by the trigger that
  observes the event, by the event's kind, or elsewhere.

## Done when

- `move` has no `at` parameter, and no call in `plugins/` chooses an observation point by hand.
- The state-based rows that put a permanent into a graveyard observe it the same way.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/action/move.syl`, `plugins/builtin/primitives/observationPoint/`,
`plugins/builtin/templates/action/{putInto,putIntoFromBattlefield}.syl`,
`plugins/builtin/definitions/subtype/enchantment/aura.syl`,
`plugins/builtin/definitions/card_type/creature.syl`, `docs/decisions/semantics.md` §4.8.

---
needs: [syl-single-face-card-call]
---

# Whether a trigger looks back is derived, not written on `move`

## Why

`move` takes an `at: ObservationPoint` (`arrival` or `departure`), chosen by hand at each call.
The rules derive whether a trigger looks back in time from the kind of trigger
[CR#603.10a..603.10f], so the hand choice can disagree with them, and it does: `counter.syl`
writes `arrival` though a trigger on a countered spell looks back [CR#603.10e], and the Aura
state-based row (`plugins/builtin/definitions/subtype/enchantment/aura.syl`) moves with
`departure`, while the creature and other state-based rows use `putInto`, observed on `arrival`.

Written against the start of step 6 (`syl-single-face-card-call`).

## Design

- The Observation Point is written nowhere. `move` has no `at` parameter, and the
  `ObservationPoint` sort, its `arrival` and `departure` primitives and the Template
  `zoneChangeObservedOnDeparture` do not exist.
- The semantics derives the Observation Point per trigger-pattern arm and occurrence, from the
  closed list of look-back triggers: leaving the battlefield, a sacrifice, leaving a graveyard, a
  publicly visible object put into a hand or library [CR#603.10a], phasing out [CR#603.10b],
  becoming unattached [CR#603.10c], a player losing control of an object or an opponent gaining
  control of it from that player [CR#603.10d], a spell being countered
  [CR#603.10e] and a player losing the game [CR#603.10f]. "From anywhere" is never a
  leaves-the-battlefield ability [CR#603.6c].
- [CR#603.10a] sorts abilities, not occurrences; this design reads its publicly visible class
  per occurrence (whether the card put into a hand or library was visible to all players), so the
  derivation is not a function of the written pattern alone (*Dutiful Knowledge Seeker*: "put into
  a library from anywhere"). No ruling settles that reading.
- The point is not a slot on the trigger: one trigger's arms can differ (*Syr Konrad, the Grim*'s
  three arms look back, look after and look back), and its `whenever(or([…]))` names one trigger.
- It is not a `looksBack` field on each event head: the rule would be stated twice, since a `move`
  pattern needs the derivation from its origin anyway.
- "Put into a graveyard from the battlefield" is `putInto(…, graveyard, from = battlefield)`, the
  printed origin as `putInto`'s `from`; `putIntoFromBattlefield` does not exist.

## Done when

- `move` has no `at` parameter, and no call in `plugins/` chooses an observation point by hand.
- The state-based rows that put a permanent into a graveyard write the move the same way.
- A `move` written with an observation argument is refused
  (`crates/xtask/tests/suite/dsl_check.rs`).
- `docs/decisions/semantics.md` §2.1 records a zone change's origin: the historical read in a
  lookback ("if that creature dies this way") and the last-known read after a shuffle key on it.
- `docs/decisions/semantics.md` §4.5 states where the before-reading applies: a look-back pattern
  reads its own descriptions before the event, and a lookback's historical subject or patient is
  the object as it was; only the trigger's body reads after. §4.6 makes the Observation Point a
  derived property of the trigger pattern and occurrence [CR#603.10a..603.10f], and the glossary
  entry **Observation Point** says so.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- `putInto`'s default `from = anywhere` makes a pattern that states no origin the same term as a
  written "from anywhere", so the [CR#603.6c] exclusion applies to every origin-less pattern
  (*Stonebinder's Familiar*'s `putInto(…, exileZone)`).

## Pointers

`plugins/builtin/primitives/action/move.syl`, `plugins/builtin/primitives/observationPoint/`,
`plugins/builtin/templates/action/{putInto,putIntoFromBattlefield,zoneChange}.syl`,
`plugins/builtin/definitions/subtype/enchantment/aura.syl`,
`plugins/builtin/definitions/card_type/creature.syl`, `docs/decisions/semantics.md` §2.1, §4.5,
§4.6, §4.8, `docs/contexts/game-model/CONTEXT.md` (**Observation Point**).

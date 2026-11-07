---
needs: []
---
**Permissions to cast or play from a zone other than the hand, and the miracle and forecast
windows.**

## Why

A spell can begin to be cast only when a rule or effect allows it [CR#601.3]. The engine offers
casting only from the hand: `legal_actions` scans the hand, and a `May(Cast)` or `May(Play)` row
carrying a `from` zone is a loud `guard_deontic_seam` instead of a permission. Effect-driven
casts from any zone already work (`begin_cast_from`, [CR#608.2g]); permission casts from the
graveyard, exile or the top of the library do not. Miracle [CR#702.94a] and forecast
[CR#702.57a,702.57b] add hand-zone windows the engine also lacks.

## Done when

- `May(Cast(from: …))` and `May(Play(from: …))` rows are evaluated: the permitted cards in that
  zone are offered as `CastSpell` (or `PlayLand`) actions, cast through `begin_cast_from`, and
  the seam guard no longer covers those shapes.
- A permission limited to specific cards reads them through the linked or referenced set the
  granting ability names, so "you may play those cards" reaches exactly the exiled cards.
- Miracle: the first card a player draws in a turn may be revealed as it is drawn, and the linked
  trigger lets them cast it for its miracle cost [CR#702.94a,702.94b].
- Forecast: the ability is activatable only from its owner's hand, during their upkeep, once each
  turn [CR#702.57a,702.57b].
- Tests use real cards: Future Sight (top of library), Yawgmoth's Will (graveyard), Light Up the
  Stage (the exiled cards), Temporal Mastery (miracle), Proclamation of Rebirth (forecast), by
  core fixture where the cards are not authored.

## Not in scope

- Choosing an alternative cost at announcement (flashback's, a permission's own cost):
  `engine-alt-costs`.

## Pointers

- `crates/engine/src/legal.rs`: `legal_actions` (the hand scan and the `Cast`/`Play` seam guard),
  `may_cast_rows`.
- `crates/engine/src/cast.rs`: `castable_cost_ignoring_mana`, `begin_cast_from`.
- `crates/engine/src/activation.rs`: linked memory cells [CR#607.1].
- `crates/core/src/deontic.rs`: `DeonticAction::Cast`, `DeonticAction::Play`.

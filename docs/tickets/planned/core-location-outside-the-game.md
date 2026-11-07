---
needs: []
---
# Give the engine a position for objects outside the game

## Why

Wishes, companion, learn and venture reach cards outside the game, which is not a zone
[CR#400.11]. The engine has nowhere to hold such a card: `GameObject.zone` is an `Option<Zone>`
whose `None` means a player proxy, and `Zone` rightly has no sideboard. Modeling these effects
today would force a fake zone.

## Design

- No `Zone` variant stands for the sideboard or any other out-of-game store; a card outside the
  game is an object with no zone.
- A position that can be outside the game uses a type that wraps a `Zone` or names outside the
  game, with `Zone` its in-game subset.
- The sideboard is deck membership, modeled by deck construction as a partition of the deck
  [CR#100.4,400.11a], not as this type and not as a `Zone`.
- Open question: whether the wider type appears only where out-of-game cards enter the model
  (fetch and reveal sources: Wishes, learn's Lesson [CR#701.48a], companion [CR#116.2g], venture
  [CR#701.49a]) or `GameObject`'s stored position widens to it.

## Done when

- `sylex_core` has the out-of-game-capable position type, with serde round-trip tests that keep
  every zone's existing spelling.
- Its name fits `docs/contexts/game-model/CONTEXT.md`, where **Location** already means a Zone, a
  pile position, anywhere, or a combination (the `.syl` sort, which treats outside the game as a
  predicate, `outsideTheGame`, not a Location).
- The open question in Design is answered and written in the type's doc comment. A player proxy
  keeps a representation distinct from "outside the game".
- `Destination` and the other guaranteed-in-game APIs still take `Zone`.

## Not in scope

Deck construction (mainboard and sideboard membership, [CR#100.4]) and any card that uses the new
position: each fetch effect lands with its own card.

## Pointers

- `crates/core/src/zone.rs`, `crates/core/src/action.rs` (`Destination`),
  `crates/engine/src/object.rs` (`GameObject.zone`, `mint`)
- `plugins/builtin/templates/predicate/outsideTheGame.syl`,
  `plugins/builtin/definitions/keyword_ability/companion.syl`,
  `plugins/builtin/definitions/keyword_action/learn.syl`

---
needs: []
---
# "You control target player during that player's next turn" (Mindslaver, Worst Fears, Sorin Markov)

## Why

A player controlling another makes every choice and decision the controlled player is allowed or
told to make, while objects, resources and active-player status stay with the controlled player
[CR#723.3,723.5,723.5a]. Core has no action for it and the engine has no notion of a player's
controller: every decision goes to its nominal decider.

## Done when

- Core has an action that makes its performer the controller of a referenced player during that
  player's next turn.
- Each player carries at most one pending controller. A new player-controlling effect overwrites
  it [CR#723.1a]; it waits through skipped turns [CR#723.1b]; it is consumed when that player's
  turn begins and is the live controller for the whole turn [CR#723.1]; it is dropped if the
  controller has left the game [CR#800.4b].
- Control changes only who decides: objects keep their controllers, the controlled player stays
  the active player [CR#723.3], and costs paid for them use their resources [CR#723.5a].
- While control is live, every decision whose concrete decider is the controlled player, except
  conceding, is routed to the controller [CR#723.5,723.6], through one remap from the nominal
  decider (`DeciderSpec`) to the actual one. The controller keeps making their own decisions
  [CR#723.8], and a player given control of themselves decides as normal [CR#723.9].
- Tests with Mindslaver: the controller makes the controlled player's priority, attack and
  targeting decisions for that whole turn, objects keep their controllers, and the next turn
  routes normally; a second activation before that turn overwrites the first [CR#723.1a]. A core
  fixture is fine where the card is not authored.

## Not in scope

- Limited-duration control (Word of Command, Opposition Agent) [CR#723.2].
- Turn-structure changes such as extra or skipped turns (`engine-turn-modification`).
- The `.syl` construct for player control: `.syl` has none yet, so the cards have no canon file.

## Pointers

- `crates/core/src/decision.rs`: `DeciderSpec`.
- `crates/engine/src/decide/mod.rs`: `DecisionPoint`, `DecisionPointKind::decider_spec`,
  `decider_player`.
- `crates/engine/src/step/mod.rs`: `begin_turn`.
- `crates/core/src/action.rs`: `Action`.

---
needs: []
---

# Detect a loop of mandatory actions and end the game in a draw

## Why

A game that enters a loop of mandatory actions, repeating a sequence of events with no way
to stop, is a draw; a loop that contains an optional action is not [CR#104.4b,733.4]. The engine
has `GameOutcome::Draw` but sets it only when no player is left. `GameState` and `GameImage` have
no equality or stable hash, so the engine cannot tell that it has returned to an earlier state,
and such a game never ends.

## Done when

- The engine can compare two game states for rules-relevant equality (an equality or stable hash
  over objects, zones and turn state that excludes transients).
- A loop monitor notices the game repeating a sequence of events that contains only mandatory
  actions, and ends the game with `GameOutcome::Draw`.
- A loop with an optional action in it does not draw.
- Tests grounded in real cards cover both: a mandatory loop draws, and the same loop with a real
  choice in it does not (Worldgorger Dragon with Animate Dead is one candidate: with the Dragon
  the only creature card in a graveyard, Animate Dead has no other legal choice).

## Pointers

`crates/engine/src/state.rs` (`GameOutcome`, `GameImage`, `GameState`),
`crates/engine/src/step/mod.rs` (where `outcome` is set), `crates/engine/src/runner.rs`.

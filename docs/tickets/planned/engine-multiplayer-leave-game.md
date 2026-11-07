---
needs: []
---
# Remove a player who leaves a multiplayer game

## Why

In a game with more than two players, a player who loses or concedes leaves the game: objects
they own leave, effects giving them control end, their non-card stack objects cease to exist, and
anything they still control is exiled [CR#800.4a,104.3a]. The engine has none of this.
`concede` in `decide/mod.rs` hits a `todo!` when more than two players are live, but a loss by
state-based action or effect goes through `PlayerLost`'s apply in `step/player.rs`, which only
sets `lost = true` and leaves the departed player's objects, control effects and seat in the game.
Multiplayer is off the 1v1 happy path, and the owner wants the design talked through before any
implementation.

## Done when

- The owner has agreed the design for turn-order repair, pending-decision ownership and
  priority-round reshaping for a departed seat; the CR's answers are [CR#800.4g..800.4k].
- With more than two live players, every way a player leaves (concession, state-based loss,
  effect-driven loss) performs [CR#800.4a]'s steps in order, and [CR#800.4b..800.4m] hold for
  the departed player.
- Until that lands, the `PlayerLost` path fails loudly with more than two live players, as
  `concede` does.
- Tests cover a three-player game in which one player leaves while owning a permanent another
  player controls, controlling a permanent another player owns, and with an ability on the stack.

## Pointers

- `crates/engine/src/decide/mod.rs` (`concede`)
- `crates/engine/src/step/player.rs` (`PlayerLost` apply)
- `crates/engine/src/step/mod.rs` (`check_game_end`, `begin_turn`)
- `crates/engine/src/state.rs` (`live_count`)
- [CR#800.4] and its subrules in `data/rules/cr.txt`

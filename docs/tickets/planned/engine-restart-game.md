---
needs: []
---
# Restart the game (Karn Liberated)

## Why

Karn Liberated's "−14: Restart the game, leaving in exile all non-Aura permanent cards exiled with
Karn. Then put those cards onto the battlefield under your control." is the only way to restart a
game [CR#728.1]. Core's `Action::RestartGame` is nullary, so it cannot name exempted cards, and
its engine arm in `crates/engine/src/resolve/player_action.rs` is a `todo!`. The engine also does
not track which cards were exiled with a given source; `.syl` spells that link as `exiledWith`.

## Done when

- Core can express a restart with an exempted set, and the engine tracks exiled-with linkage
  [CR#607.2a] well enough to select it.
- Restarting ends the current game with no winner, loser or draw, and starts a new one with every
  card involved in the old game, phased-out permanents included, except the exempted cards.
  Ownership is unchanged [CR#728.1,728.2,728.5].
- The new game follows the start-of-game procedure, with the restarting effect's controller as
  the starting player [CR#728.1a].
- The rest of the restarting effect runs just before the first turn's untap step with no player
  holding priority. Its triggers wait for the first priority [CR#728.4].
- A Karn Liberated test: an exiled non-Aura permanent card ends up on the battlefield under Karn's
  controller in the new game, and an exiled Aura card does not stay in exile.

## Pointers

- `crates/core/src/action.rs` (`Action::RestartGame`), `crates/engine/src/resolve/player_action.rs`
- `crates/engine/src/state.rs` (`GameState::new`)
- `plugins/builtin/primitives/action/restartGame.syl`, `plugins/builtin/primitives/zone/exiledWith.syl`

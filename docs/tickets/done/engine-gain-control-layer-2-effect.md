---
needs: []
---

# Engine: a resolved "gain control" is a timestamped layer-2 effect, not a base-controller write

## Why

A resolving spell or ability that gains control of a permanent creates a continuous effect that
lasts until the end of the game unless it says otherwise [CR#611.2a], applied in layer 2 in
timestamp order [CR#613.1b,613.7]. The engine instead lowers `Action::GainControl` to a one-shot
`ControlChanged` event (`crates/engine/src/resolve/action.rs`) whose apply overwrites the base
`GameObject.controller` (`crates/engine/src/step/player.rs`), and every layer-2 effect applies on
top of that base whatever its timestamp. With any continuous control effect in play this gives the
wrong controller:

- Player 1's Control Magic holds player 0's creature; player 0 then resolves a "gain control of
  target creature" on it. The later effect should win [CR#613.7], but the base write is
  overridden by Control Magic's layer-2 effect and player 1 keeps it.
- Player 1 resolves "gain control of target creature" on a creature player 1 already controls
  through Control Magic. `GainControl` sees player 1 as the derived controller and emits nothing,
  so when Control Magic leaves the battlefield the creature returns to player 0, though player 1's
  later effect still applies.

The exchange form must keep [CR#701.12b]: permanents controlled by the same player are not
exchanged.

## Done when

- A resolved gain-control (and each half of a resolved exchange) creates a layer-2 continuous
  effect with its own timestamp and duration instead of writing the base controller; the base
  controller is only what the permanent entered under [CR#110.2].
- Tests: both bullets above, with Control Magic's effect as a locked `SetController`, and an
  exchange of two permanents one player controls doing nothing.
- The new controller's summoning sickness still comes from `GameState::observe_control_changes`.

## Pointers

- `crates/engine/src/resolve/action.rs` (`Action::GainControl`), `crates/engine/src/step/player.rs`
  (`ControlChanged` apply), `crates/engine/src/controller.rs`.
- `crates/engine/src/resolve/effect.rs` (Avarice Totem's `Continuously` exchange, the continuous
  spelling of the same swap).

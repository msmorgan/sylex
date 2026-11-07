---
needs: []
---
# Engine: offer and take special actions

## Why

A special action is one a player may take while they have priority, and it doesn't use the stack
[CR#116.1]. The engine models one of them, playing a land [CR#116.2a], as `Action::PlayLand`. The
rest exist only as a shape. `SpecialAction` in `crates/engine/src/decide/mod.rs` names
`TurnFaceUp`, `EndEffect`, `IgnoreStatic`, `SuspendCast`, `CompanionToHand`, `Foretell`,
`PlotExile` and `UnlockHalf` [CR#116.2b,116.2c,116.2d,116.2f,116.2g,116.2h,116.2k,116.2m], but nothing uses them:

- `legal_actions` never offers an `Action::Special`;
- `take_priority_action` (`crates/engine/src/decide/mod.rs`) and `describe_action`
  (`crates/engine/src/render.rs`) both `todo!` on one.

[CR#116.2e], Circling Vultures discarding itself "any time you could cast an instant", has no
`SpecialAction` variant.

Each special action has its own timing [CR#116.2] and its own prerequisite. The prerequisites
belong to the mechanics' own tickets (for example, turning a face-down creature face up belongs to
`engine-face-down`). This ticket builds the shared path from offering a special action to taking
it.

## Done when

- `legal_actions` offers a `SpecialAction` exactly when its [CR#116.2] timing holds and its
  mechanic says it is available.
- `take_priority_action` takes it without using the stack and leaves the player with priority.
  `describe_action` renders it.
- A variant whose mechanic is unbuilt is never offered. Its execute arm panics naming the ticket
  that owns the mechanic.
- At least one special action whose mechanic exists runs end to end in a `sylex_engine` test with
  a real card.

## Not in scope

- Each mechanic's own behavior: morph and the other face-down rules, suspend, companion, foretell,
  plot, Room unlocking.

## Pointers

- `crates/engine/src/decide/mod.rs` (`SpecialAction`, `Action`, `take_priority_action`).
- `crates/engine/src/legal.rs`, `crates/engine/src/render.rs`.
- `crates/engine/src/payment/replay.rs` (already rebinds `Action::Special`).

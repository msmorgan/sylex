---
needs: [engine-combatant-role, engine-history-tallies-cache, engine-derived-controller-reads]
---
# Derive summoning sickness from history

## Why

`GameObject.summoning_sick` is per-object mutable state that the engine keeps in step by hand: it
is set on battlefield arrival, token mint and control change, and cleared at the controller's turn
start (`step/mod.rs`, `step/player.rs`), and the payment simulation in `payment/fulfill.rs`
writes it too. Its one rules consumer is `StatePredicate::SummoningSick` (`target.rs`), which the
creature type's conditional `Cant` conferral reads through `Matches(This, SummoningSick)`; the TUI
reads it for display. Both facts it summarizes are already recorded history: battlefield entry is a
`ZoneChange` to the battlefield and a one-shot control change is `ControlChanged { of, to }`.
Evaluating `SummoningSick` as a history query removes the field and every write site.

[CR#302.6] makes the window controller-relative: a creature is sick unless it "has been under its
controller's control continuously since their most recent turn began". `Lookback::ThisTurn` is
the active player's turn, so it gives the wrong answer for a creature the non-active player gained
or put onto the battlefield (flash, Threaten on an opponent's turn) when another turn that is not
its controller's comes next (an extra turn, or a third player's turn).
`Lookback::SinceYour(PhaseStep)` is controller-relative but anchors on a step, not on the turn's
start, and is built for trigger watchers, not for a per-object `Matches(This, …)` read during the
layer gather.

A layer-2 control change ([CR#613.1b]) also starts the window. It writes no `ControlChanged`
event, so the query needs a fact for the derived Controller changing, which
`engine-derived-controller-reads` introduces when it corrects the bool.

The conferral condition is evaluated per creature per layer pass, so the history read must be
cheap: this waits for `engine-history-tallies-cache`.

## Done when

- `GameObject.summoning_sick` and all its write sites are gone; `StatePredicate::SummoningSick`
  is evaluated from history, as "no event since the controller's most recent turn began put this
  object under its current controller": battlefield entry, a one-shot control change, or a
  derived (layer-2) Controller change.
- The window binds to the object's current controller, not the active player, in the layer-gather
  `Matches(This, …)` context.
- No behavior change: the existing summoning-sickness, haste and control-change tests pass
  unchanged, plus new ones for a creature that entered or changed control on another player's
  turn (still sick on a later turn that is not its controller's, such as an extra turn, and not
  sick once its controller's turn begins) and for a layer-2 steal (Avarice Totem).
- The TUI's sickness display reads the derived predicate.

## Pointers

- `crates/engine/src/object.rs` (`summoning_sick`), `crates/engine/src/target.rs`
  (`StatePredicate::SummoningSick` arm), `crates/engine/src/trigger.rs` (snapshot arm).
- `rg summoning_sick crates/engine/src crates/tui/src` for the write sites.
- `crates/core/src/temporal.rs` (`Lookback`), `crates/core/src/condition.rs` (`Happened`),
  `crates/core/src/event.rs` (`ControlChanged`).
- `crates/engine/src/legal.rs` (the creature type's `Conditionally(SummoningSick & !Haste, …)`
  rows).

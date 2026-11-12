---
needs: []
---

# Engine: every control change records a `ControlChanged` fact

## Why

A permanent's controller changes whenever a layer-2 effect [CR#613.1b] begins, ends or changes
its affected set. The engine records a `ControlChanged` fact only for a resolved "gain control"
(`GameState::gain_control`, `crates/engine/src/controller.rs`). The other changes record nothing:
Control Magic entering or becoming attached, a resolved `Continuously` steal (Act of Treason's
shape), each half of Avarice Totem's exchange, and a control effect ending (Threaten's at
cleanup). `GameState::observe_control_changes` sees every one of them but only sets summoning
sickness.

So history and triggers miss them. Echo's "if this permanent came under your control since the
beginning of your last upkeep" [CR#702.30a] reads `ControlChanged` from history and misses a
permanent taken with Control Magic. A trigger on a player gaining control ("Whenever an opponent
gains control of a permanent from you") misses all but a resolved gain-control, and "When you
lose control of …" (`syl-lose-control-event`) has no fact to match, though that trigger looks
back in time [CR#603.10d].

## Done when

- Every change of a permanent's derived controller records one `ControlChanged` fact, whatever
  made it: an effect beginning, ending or changing its affected set; changes that happen together
  record one batch [CR#603.2c].
- The fact carries what a trigger looking back in time needs to match a player losing control
  [CR#603.10d].
- Tests: Echo on a creature stolen with a locked `SetController` (Control Magic's effect) asks
  for its echo at its new controller's next upkeep; Avarice Totem's exchange records a batch of
  two facts; a steal ending at cleanup records the return to the owner.

## Pointers

- `crates/engine/src/controller.rs` (`gain_control`, `observe_control_changes`).
- `crates/engine/src/step/player.rs` (`ControlChanged` apply), `crates/engine/src/eval.rs`
  (`FactKind::ControlChanged`), `crates/engine/src/trigger.rs`.
- `v1/plugins/builtin/macros/keyword/Echo.ron`.
- `docs/tickets/planned/engine-summoning-sickness-from-history.md` reads the same fact.

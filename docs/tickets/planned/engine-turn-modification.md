---
needs: []
---
# Extra turns, extra phases, skipping and ending the turn

## Why

The engine walks a fixed turn (`turn::successor`) and has no way to change it. Effects can add
turns [CR#500.7], phases [CR#500.8] and steps [CR#500.9,500.10], skip a step, phase or turn
[CR#500.11,614.10], or end the turn [CR#724.1]. `Action::ExtraPhase` exists in core but its engine
arm is a `todo!()` naming this ticket (`resolve/action.rs`). `Replacement::Skip` exists in core and
the replacement registry passes it through to a step-elision pass that does not exist
(`replace_registry.rs`). There is no extra-turn or end-the-turn action. Examples: Time Walk,
Relentless Assault, Stasis, Fatespinner, Time Stop, Sundial of the Infinite.

## Done when

- Extra turns are added directly after the specified turn, the most recently created first
  [CR#500.7].
- Extra phases and steps are inserted after (or before) the specified phase or step, the most
  recently created first [CR#500.8,500.9,500.10]; `Action::ExtraPhase` resolves.
- A `Replacement::Skip` makes the engine proceed past the step, phase or turn as though it didn't
  exist; anything scheduled for "the next" occurrence waits for the first one not skipped
  [CR#500.11,614.10,614.10a].
- Ending the turn follows [CR#724.1a..724.1f].
- Each is tested with one of the real cards above.

## Pointers

- `crates/engine/src/turn.rs` (`TurnState`, `successor`), `crates/engine/src/step/mod.rs`
  (`begin_step`)
- `crates/core/src/action.rs` (`ExtraPhase`), `crates/core/src/replacement.rs` (`Skip`),
  `crates/core/src/event.rs` (`PhaseKind`)
- `crates/engine/src/resolve/action.rs`, `crates/engine/src/replace_registry.rs`

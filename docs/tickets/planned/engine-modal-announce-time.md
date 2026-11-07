---
needs: []
---
# Announce the modes of a modal triggered ability when it is put on the stack

## Why

Spells and activated abilities already announce their modes while being cast or activated
(`cast.rs`: `ChooseModes` with `DecisionContinuation::AnnounceModes`, per-mode targets, per-mode
costs and the entwine/escalate rider folded into the total cost), and a copy keeps the original's
`chosen_modes` [CR#700.2g]. Triggered abilities do not: `place_one_trigger` in `trigger.rs` always
pushes an empty `chosen_modes`, so a modal trigger falls through to the resolution-time
`Instruction::Modal` arm in `resolve/effect.rs`. Its controller must instead choose the modes as
the ability is put on the stack, a mode with no legal targets can't be chosen, and the ability is
removed from the stack if no mode is chosen [CR#603.3c,700.2b]. Today a trigger whose modes carry
targets reaches the resolution-time arm and silently does nothing (an early `return`).

## Done when

- Placing a modal triggered ability surfaces its controller's (or the named chooser's [CR#700.2e])
  mode choice before its targets, offers only modes with legal targets, and drops the trigger
  when no mode can be chosen [CR#603.3c,700.2b,700.2c].
- The chosen modes ride on the trigger's `StackEntry` and resolve through the same announced-mode
  path spells use.
- The resolution-time `Instruction::Modal` arm serves only resolution-time choices (villainous
  choice [CR#701.55a] is spelled with `chooseOne`); a mode with targets, a cost, or a rider that
  reaches it fails with an error naming the defect instead of returning silently.
- The test `modal_with_per_mode_targets_or_rider_fizzles_without_panic` in `resolve/effect.rs`,
  which pins the silent no-op, is rewritten to assert the error; the stale comment there that
  calls announce-time modes unbuilt is corrected.
- Tests built on Entomber Exarch (an enters trigger whose two modes each target) cover choosing
  either mode and the drop when neither mode has a legal target.

## Pointers

- `crates/engine/src/trigger.rs` (`place_one_trigger`, `trigger_targets`)
- `crates/engine/src/cast.rs` (`announced_target_specs`, `announced_effect_items`, the mode
  announcement)
- `crates/engine/src/resolve/effect.rs` (the `Instruction::Modal` arm and its test)
- `crates/engine/src/step/stack.rs` (copy keeps `chosen_modes`)
- `plugins/builtin/definitions/keyword_action/faceAVillainousChoice.syl`

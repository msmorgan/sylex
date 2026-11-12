---
needs: []
---

# Engine: every rules-facing controller read uses the derived (layer-2) controller, and a layer-2 control change makes the permanent summoning sick for its new controller

## Why

A continuous control-changing effect applies in layer 2 [CR#613.1b]
(`Modification::SetController` in `crates/engine/src/layer.rs`) and changes only the layered
view's controller. Most of the engine reads the base `GameObject.controller`, and only the
one-shot `ControlChanged` apply (`crates/engine/src/step/player.rs`) sets `summoning_sick`. A
creature stolen by a continuous effect (Control Magic; the engine's test fixture of Avarice
Totem spells its exchange as two `Continuously(… SetController …)` halves) therefore:

- sheds summoning sickness and untaps on its owner's turn, not its new controller's
  (`begin_turn` and the untap step in `crates/engine/src/step/mod.rs`) [CR#302.6,502.3];
- has its triggered abilities controlled by its owner (watcher construction in
  `crates/engine/src/trigger.rs`) [CR#603.3a];
- can be sacrificed only by its owner (`Sacrifice` payability in `crates/engine/src/activate.rs`)
  [CR#701.21a];
- pays lifelink to its owner (`crates/engine/src/step/combat.rs`) [CR#702.15b];
- is not matched by its new controller's "creatures you control" (`RelationPredicate::ControlledBy`
  in `crates/engine/src/target.rs`);
- can attack for its new controller at once, which [CR#302.6] forbids. The test
  `stolen_creature_attacks_for_new_controller` (`crates/engine/tests/layers.rs`) asserts that
  wrong behavior.

These are the sites already traced; there are about 200 non-test `.controller` reads in the
crate. The snapshot's captured controller is `engine-lki-robustness`'s.

## Done when

- Every rules-facing controller read in `crates/engine/src` reads the derived controller
  (`state.layers().controller(id)`, or one view per step or scan). Each read that stays on the
  base field (the layer system's own input, the one-shot `ControlChanged` apply, a card off the
  battlefield whose controller is its owner [CR#108.4a]) says why in a comment.
- Whenever the derived controller of a battlefield permanent changes, through a one-shot or a
  continuous effect, the permanent is summoning sick for its new controller until that player's
  next turn begins [CR#302.6]. Both paths go through the same rule.
- `stolen_creature_attacks_for_new_controller` is rewritten: a stolen creature without haste
  can't attack that turn and can on its new controller's next turn; a stolen creature with haste
  can attack at once.
- Tests with a continuous steal (Avarice Totem's exchange or a locked `SetController` effect):
  the stolen creature untaps in its new controller's untap step, not its owner's; its triggered
  abilities are controlled by the new controller; the new controller can sacrifice it and the
  owner can't; its lifelink pays the new controller; the new controller's "creatures you
  control" matches it; it can't attack or pay a {T} cost until its new controller's next turn.

## Pointers

- `crates/engine/src/layer.rs` (`Modification::SetController`, the layered `controller` read)
- `crates/engine/src/step/mod.rs`, `step/player.rs`, `step/combat.rs`, `trigger.rs`,
  `activate.rs`, `target.rs`
- `crates/engine/tests/layers.rs` (`stolen_creature_attacks_for_new_controller`)
- `v1/plugins/canon/cards/Avarice Totem.ron` (the engine's test fixture)
- `engine-summoning-sickness-from-history` (planned) derives sickness from control-change
  history and must use the same rule.

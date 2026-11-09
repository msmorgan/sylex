---
needs: []
---
# Delete the Planechase surface

## Why

Planechase is permanently out of scope, so nothing may model it or reserve room for it. Core still
declares the planar die: `PlanarFace`, `Action::RollPlanarDie` and `EventFilter::RollPlanarDie`,
which the engine answers with a never-matching filter arm and a no-op special action. v1 mirrors
them in its semantics, lowering, render and dead-grammar gate, and a `.syl` comment says a die
roll's sides may name planar dice.

## Done when

- No `PlanarFace` or `RollPlanarDie` remains in `crates/` or `v1/crates/`: core's `mana.rs`,
  `action.rs`, `event.rs` and `lib.rs` re-export; the engine's `eval.rs`,
  `resolve/player_action.rs` and `payment/fulfill.rs` arms; v1's semantics (including the three
  `identity_registry.rs` rows), lowering (`mana.rs`, `action.rs`, `event.rs`, `minimal.rs`),
  render (`render/ability.rs`, `render/effect.rs`) and the three `BLOCKED` entries in
  `v1/crates/plugin/tests/no_dead_grammar.rs`. Tests whose only subject was a deleted item are
  deleted.
- The doc comments that cite [CR#901.9d] to say the planar die has no numeric result (in both
  lines' `action.rs`, `event.rs` and `continuous.rs`) no longer mention the planar die, and the
  planar-dice sentence in `plugins/builtin/primitives/action/rollDice.syl` is gone.
- `cargo xtask cite bless` drops the Planechase rules nothing cites any more; `cargo xtask cite
  check` reports 0 stale and `--list-noncompliant` is empty.

## Pointers

- `crates/core/src/mana.rs`, `crates/core/src/action.rs`, `crates/core/src/event.rs`
- `crates/engine/src/resolve/player_action.rs`, `crates/engine/src/eval.rs`

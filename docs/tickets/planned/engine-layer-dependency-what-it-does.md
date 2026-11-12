---
needs: []
---

# Engine: a layer effect depends on another that changes what it does

## Why

An effect depends on another in the same layer when applying the other would change its
existence, what it applies to, "or what it does to any of the things it applies to"
[CR#613.8a]; it then applies just after [CR#613.8b]. `depends_on` (`crates/engine/src/layer.rs`)
compares only existence and the affected set, so a dependency through what an effect does is
missed and the effects apply in timestamp order.

Layer 2 shows it with real cards. P0's Control Magic ("You control enchanted creature.") enchants
P1's Grizzly Bears; P1 later gains control of the Control Magic (a locked `SetController`, as
Avarice Totem's exchange makes). Applying P1's effect changes Control Magic's "you" [CR#109.5],
so Control Magic's effect depends on it and P1 controls the Bears. The engine applies Control
Magic's earlier effect first and leaves the Bears with P0. A throwaway test of this board (aura
timestamp 500, steal timestamp 1000) confirmed it.

The same gap covers any op whose value reads something an earlier-timestamped effect in the same
layer changes; the `depends_on` doc names value dependency as a limitation.

## Done when

- `depends_on` also reports a dependency when applying the other effect changes what the first
  does to an object it applies to, including its controller-relative "you" for a layer-2
  `SetController`.
- Tests: the Control Magic board above (P1 controls the Bears), and the same board with the steal
  timestamped before Control Magic, which already works
  (`stolen_control_magics_you_is_its_new_controller` in `crates/engine/src/controller.rs`).

## Pointers

- `crates/engine/src/layer.rs` (`depends_on`, `probe_apply`, `ActiveEffect::controller_in`).

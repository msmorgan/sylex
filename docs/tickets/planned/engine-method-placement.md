---
needs: []
---
# Move single-component methods off `GameState`

## Why

`GameState` (`crates/engine/src/state.rs`) derefs to `GameImage`, a struct of components
(`Zones`, `ObjectStore`, `TurnState`, `CombatState`, `PlayerState`, `Cards`, `DesignationStore`,
…), but nearly all behavior hangs off `impl GameState` blocks spread across some two dozen files.
Many of those methods read or write exactly one component. On `GameState` they hide what each
subsystem needs and force `let x = self.f(); self.g(x)` borrow workarounds that disjoint field
borrows would make unnecessary.

## Done when

- Each `impl GameState` method that touches only one component lives on that component's own
  `impl` (or is a module-local free function over that component's data), and its callers call it
  through the component (`self.zones.foo(..)`).
- A method that touches one component plus a cheap read of another takes that read as a
  parameter where the call site reads no worse; it stays on `GameState` otherwise.
- Moved methods get the narrowest visibility that compiles; no new `pub` items.
- No behavior changes.
- Moved doc comments keep their citations: `cargo xtask cite check` reports 0 stale.

## Not in scope

- Traits: each component has a single implementor, so this is plain method placement.
- Splitting large files; `split-trigger-rs` and `split-step-rs` own that, and a move may ride
  along with either.

## Pointers

- `crates/engine/src/state.rs` (`GameState`, `GameImage`)
- `crates/engine/src/zone.rs`, `object.rs`, `turn.rs`, `combat.rs`, `player.rs`
- `docs/tickets/planned/split-trigger-rs.md`, `docs/tickets/planned/split-step-rs.md`

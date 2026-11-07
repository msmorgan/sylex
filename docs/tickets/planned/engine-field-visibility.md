---
needs: []
---
# Let the compiler enforce that only the engine changes game state

## Why

`GameState`'s own fields are `pub(crate)`, but it derefs, mutably too, to `GameImage`
(`crates/engine/src/state.rs`, re-exported from `lib.rs`), whose rules-state fields (`zones`,
`objects`, `stack`, `turn`, `combat`, `rng`, `agenda`, `shields`, `history` and the rest) are
`pub`. Any consumer crate can write `state.zones` or `state.stack` directly and bypass the
zone-change pipeline (last known information, replacement shields, trigger noting, state-based
action scheduling). [Engine](../../decisions/engine.md) law 2 gives consumers policy, never state
transitions, and today nothing enforces it. A rough grep finds about 80 field accesses in
`crates/tui` and `crates/matchups` and about 1200 in `crates/engine/tests`.

## Done when

- No `pub` field on `GameState` or `GameImage` and no `pub` `DerefMut` to them is reachable
  from outside `sylex_engine`; fields are `pub(crate)`, or private where one module owns them.
- Every external access the compiler reports is classified and rerouted: a read-only use gets a
  narrow `pub` read accessor or view (never a `&mut` accessor); a mutation goes through an
  engine entry point (`step`, `submit_decision`, the `sim` harness) or a deliberate new API; test
  setup in `crates/engine/tests` uses fixtures rather than field writes.
- No behavior changes.

## Not in scope

- Visibility between modules inside `sylex_engine`.

## Pointers

- `crates/engine/src/state.rs`: `GameState`, `GameImage`, the `Deref`/`DerefMut` impls.
- `crates/engine/src/lib.rs`: the `pub use` re-exports.
- `crates/engine/src/sim.rs`, `crates/engine/src/test_support.rs`.
- `crates/tui/src`, `crates/matchups/src`, `crates/engine/tests`.

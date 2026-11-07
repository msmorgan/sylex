---
needs: []
---
# Split `step/mod.rs` along its method clusters

## Why

`crates/engine/src/step/mod.rs` is 3,405 lines: about 2,190 of `impl GameState` step methods and
about 1,020 of tests. The sibling files in `step/` hold only the `EventApply` impls, so the step
methods themselves were never split.

## Done when

- The `impl GameState` step methods live in `step/` submodules cut along their concept clusters,
  and `crate::step::…` paths used outside the module still resolve.
- Each submodule's tests sit with it, sharing fixtures through one `#[cfg(test)]` module.
- `cargo test -p sylex_engine` passes with the same number of tests, and
  `cargo clippy -p sylex_engine --tests -- -D warnings` is clean.

## Pointers

- `crates/engine/src/resolve/`: an earlier split of the same kind.
- `docs/tickets/planned/split-trigger-rs.md`: the sibling split.

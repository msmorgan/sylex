---
needs: []
---
**The legend rule is surfaced before, and apart from, the other state-based
actions.**

## Why

A state-based-action check performs every applicable state-based action
simultaneously as a single event [CR#704.3], and the legend rule is one of them
[CR#704.5j]. `check_sbas` instead opens the legend-rule choice first and runs
the mechanical sweep only in a later check, which can change what happens
together and what triggers see.

## Done when

- The legend-rule choice and the mechanical state-based actions of one check
  are performed as one event, or the investigation shows the current order is
  already equivalent and a test pins that behavior.
- A test covers a check where the legend rule and a mechanical state-based
  action apply at once (two same-name legends, one with lethal damage).
- `cargo test -p sylex_engine` and `cargo clippy -p sylex_engine --tests -- -D
  warnings` pass.

## Pointers

- `crates/engine/src/step/mod.rs`: `check_sbas`, `emit_sba_batch`.
- `crates/engine/src/sba.rs`: `sweep`, `legend_rule_groups`.
- [Engine](../../decisions/engine.md) law 5.

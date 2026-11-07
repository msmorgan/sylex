---
needs: []
---
# Engine: the headless strategies answer every decision kind explicitly

## Why

`mechanical()` in `crates/engine/src/sim.rs` ends in a catch-all `other => todo!(…)` arm. It
covers every `DecisionPointKind` the arms above it do not name: `ChooseModes`, `Division`, `Vote`,
`YesNo`, `ChooseCostOptions`, `ChooseReplacement`, `PreGame`, `LegendRule`
and `ArrangePile`. A new kind therefore compiles without an answer.

`StrategyEvaluator`'s fallback in `crates/engine/src/strategy.rs` names its gaps explicitly, but
two of them, `ChooseCostOptions` (raised in `crates/engine/src/cast.rs`) and `LegendRule` (raised
in `crates/engine/src/step/mod.rs`), are decisions the engine already raises, so a game that
reaches one panics. `PreGame` is not raised yet; its mechanic belongs to `engine-pregame-procedure`.

## Done when

- `mechanical()` has no catch-all arm: each `DecisionPointKind` gets an always-legal answer or an
  explicit arm.
- Both strategies answer `ChooseCostOptions` and `LegendRule` with an always-legal default.
- A kind the engine does not raise yet keeps an explicit loud arm that names the ticket building
  its mechanic, and that ticket replaces the arm with an answer when it lands.

## Not in scope

- `OrderReplacements`, which is never raised: `engine-order-replacements-decision` deletes it, and
  no strategy answers it.

## Pointers

- `crates/engine/src/sim.rs` (`mechanical`, the `Greedy*` strategies).
- `crates/engine/src/strategy.rs` (the `StrategyEvaluator` fallback).
- `crates/engine/src/decide/mod.rs` (`DecisionPointKind`).

---
needs: []
---
# Engine: an illegal answer re-asks the same decision, and runners break the loop

## Why

An illegal `submit_decision` should re-ask the same decision rather than end the caller. That is
almost how it works today: on `Err(DecisionError::Illegal { reason })` the decision stays
pending, and the next `step()` surfaces the same `NeedsDecision`. The surfaced decision does not
say it is a repeat or why the last answer failed, though. A deterministic driver that resubmits
the same illegal answer, a test especially, loops forever. And `sim::play` ends the game with
`.expect("a strategy submits only legal decisions")`.

## Open questions

- Whether this proposed shape is the one built:
  - `submit_decision` keeps returning `Err(Illegal)`, and the re-surfaced decision carries an
    attempt count and the last rejection reason as data. The engine has no retry cap.
  - Breaking the loop is each runner's policy: a test driver fails on a re-asked decision;
    `sim::play` answers a re-asked decision with the always-legal mechanical answer instead of
    asking the strategy again; the TUI re-prompts and shows the reason.
- Where the count and reason live: on each `DecisionPointKind` variant, or in a wrapper beside
  `pending`.
- Whether `Err` stays once every runner reads the re-asked decision.

## Done when

- A re-asked decision carries what the settled shape says, and each runner
  (`crates/engine/src/sim.rs`, `crates/engine/src/runner.rs`, `crates/tui/src/driver.rs`)
  applies its loop-breaking policy.
- Every decision kind has an always-legal fallback answer for `sim::play`. The gaps are
  `engine-shell-decision-strategies`.

## Pointers

- `crates/engine/src/decide/mod.rs` (`submit_decision`, `DecisionError`).
- `crates/engine/src/sim.rs` (`play`, `mechanical`), `crates/engine/src/strategy.rs`.
- `crates/engine/src/runner.rs`, `crates/tui/src/driver.rs`.

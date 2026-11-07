---
needs: [engine-payment-obligation-window]
---
# Check every cost instruction the payment protocol cannot run

## Why

A cost is a block of instructions (`Choose`, `Sample`, `Search`, `Let`, `Act`) in
`crates/core/src/cost.rs`, and `RunnableCostAction` keeps choices and random subjects out of the
paying action so each is its own locked IOU. Two shapes still pass core validation and then go
wrong in the engine:

- A cost `Search` with a non-empty `if_none` block is validated (`region.rs`) but the payment path
  (`payment.rs` locking, `payment/fulfill.rs`) never runs it: the whiff branch is silently
  dropped.
- A producing `Act` (one with a `dest`) is accepted for any runnable action, but fulfillment
  supports only `Move` and `Sacrifice` and rejects the rest at payment time.

## Done when

- Each of these shapes is either run by the payment protocol or rejected when plugin data is
  loaded, with an error naming it; none is accepted and then dropped or refused mid-payment.
- A shape that is run exposes every choice and random outcome to the payment transcript and is
  preflighted as part of the whole selected cost body, like the existing instructions.
- Tests cover a cost search whose `if_none` branch is taken, and a producing action outside
  `Move`/`Sacrifice` (run or rejected at load).

## Pointers

- `crates/core/src/cost.rs` (`CostComponent`, `RunnableCostAction`, the boundary tests)
- `crates/core/src/region.rs` (cost-instruction validation)
- `crates/engine/src/payment.rs` (`lock_components`, `automatic_step_witness`)
- `crates/engine/src/payment/fulfill.rs` (`IouKind::Act`, `IouKind::Search`)
- `crates/engine/tests/payment.rs`, `crates/engine/tests/payment_replay.rs`

---
needs: []
---
# Costs that move an object to a public zone, read by the ability's effect

## Why

If a cost moves an object to a public zone, the ability's effects can find that object
[CR#400.7j]. A producing cost is `CostComponent::Act { dest: Some(..), .. }`. Its fulfillment
(`IouKind::Act` in `crates/engine/src/payment/fulfill.rs`) writes the moved object's pre-move id,
and register reads chase it to the new object. Two gaps remain:

- Fulfillment accepts only `Move` and `Sacrifice` as producing actions and rejects any other
  cost-eligible action at fulfillment time. A producing discard ("Discard a card: … the discarded
  card's mana value", Mercurial Chemister) passes `is_cost_eligible` and announcement
  (`cost_step_feasible` and `can_pay_verbs` in `crates/engine/src/activate.rs`), then fails when
  paid.
- Effect-body reads are tested only for a sacrifice read through last-known information
  (`a_cost_block_binds_its_paid_product_for_the_ability_body` in `resolve/effect.rs`). Nothing
  tests a body finding the moved object itself in its new public zone.

## Done when

- Every action a producing cost can be built with is payable through fulfillment and writes its
  product, or is refused when the cost is built. None is refused only at fulfillment.
- Tests: Dread Defiler (exile a creature card from your graveyard; the opponent loses life equal to
  the exiled card's power), Mercurial Chemister (discard a card; damage equal to the discarded
  card's mana value), and a body that acts on the moved object in its new zone (Dollhouse of
  Horrors copies the exiled card).

## Pointers

- `crates/core/src/cost.rs` (`CostComponent::producing`, `RunnableCostAction`),
  `crates/core/src/action.rs` (`is_cost_eligible`)
- `crates/engine/src/payment/fulfill.rs`, `crates/engine/src/payment.rs` (`lock_action`)
- `crates/engine/src/activate.rs` (`cost_step_feasible`, `can_pay_verbs`)
- `crates/engine/tests/payment.rs` (`producer_cost_runs_the_producer_then_binds_its_moved_product`)

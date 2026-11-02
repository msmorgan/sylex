---
needs: [lean-retirement]
---
# Spelling-order argument stepping consistency pass across `.syl` and `discourse`

Standard constraints apply. Follow-up after `semantics-rust-rewrite` and `lean-retirement`: once
`discourse` has been verified against the Lean oracle using Lean's stepping order and `lean/` is
deleted, align `.syl` declaration parameter/spelling order and switch `discourse` from Lean's
order-recovering rules to declaration order.

- Audit the ~30 `.syl` declarations in `plugins_v3/builtin/` whose English surface order differs
  from their current parameter declaration order:
  - the 6 primitives in Lean's `ReadOrder.fieldOrders` (`preventDamage`, `redirectDamage`, `vote`,
    `removeCounters`, `moveCounters`, `pay`);
  - `doOnlyIf` / `unless` (postposed condition after action, removing `Action::DoIf.postposed`);
  - `when` / `whenever` / `at` / `Ability::Triggered` (intervening `if` condition before action);
  - `gatedBy` / `StaticSpec::GatedBy` (cost after subject and action);
  - the 16 declarations whose existing `spelling` holes reorder parameters.
- Settle and apply the uniform ordering rule across `plugins_v3/` and `semantics` / `discourse`
  (either ordering parameters in printed order with optional arguments passed by name, or reading
  argument stepping order from completed `spelling` strings once optional-clause spelling syntax is
  in place).
- Delete `Action::DoIf.postposed` and the `ReadOrder.fieldOrders` / special-case order overrides in
  `discourse`.

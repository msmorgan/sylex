---
needs: [lean-retirement]
---
# Make every declaration's read order its printed order, and drop `doIf`'s `postposed`

## Why

Read order is printed order (`docs/decisions/syl.md` §5): where a declaration's printed order
differs from its parameter order, the parameters are reordered to match. Several `.syl`
declarations still differ, and `doIf` carries a `postposed: Bool` flag that says in a boolean what
the printed order should say. The reorder waits for Lean's deletion because Lean's hand-written
proofs call the generated definitions positionally.

## Done when

- Every declaration in `plugins/builtin/` whose printed order differs from its parameter order is
  reordered to match, with its call sites in `plugins/` updated. Cases found so far (re-survey
  when claimed): the trigger templates `when`, `whenever` and `at` (the intervening `if` is
  printed before the action), `gatedBy` and `staticSpecGatedBy` (the cost is printed last), and
  the `preventDamage`, `removeCounters`, `vote` and `pay` families.
- `doIf` has no `postposed` parameter; `doOnlyIf` ("<action> if <condition>") says the printed
  order itself, and what the flag changed for reference resolution is kept by that order.
- Required arguments that end up after an optional one are passed by name where the argument
  binding rule requires it.

## Pointers

- `docs/decisions/syl.md` §5
- `plugins/builtin/templates/ability/when.syl`, `whenever.syl`, `at.syl`,
  `plugins/builtin/templates/static/gatedBy.syl`,
  `plugins/builtin/primitives/static/staticSpecGatedBy.syl`,
  `plugins/builtin/primitives/action/doIf.syl`, `plugins/builtin/templates/action/doOnlyIf.syl`

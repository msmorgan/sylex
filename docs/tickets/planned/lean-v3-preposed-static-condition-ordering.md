---
needs: [lean-generated-canon-modules]
---
# Preposed condition ordering on `StaticSpec.conditional` and Angelic Cub's second ability

Standard constraints apply.

## Context

In `lean/Semantics/Check/Phrase.lean:4721`, `StaticSpec.conditional spec cond _` always steps
`spec` before `cond`. For postposed static conditions (`"This creature has flying as long as it has
three or more +1/+1 counters on it"`), `spec` introduces the antecedent (`thisCreature`) and `cond`
resolves `"it"` against it. For preposed static conditions (`"As long as this creature has three or
more +1/+1 counters on it, it has flying"`, as printed on *Angelic Cub*), the condition clause
introduces the antecedent (`thisCreature`) and the static grant reads `"it"` from the condition.

Because `StaticSpec.conditional` currently lacks a preposed-condition ordering mode (analogous to
`Action.doIf` vs. preposed conditional actions), `plugins_v3/canon/cards/Angelic Cub.lean` currently
authors only its triggered ability (`angelicCubPutsTheCounterOnItself` in
`lean/Semantics/Proofs/CardsTrigger.lean:340`).

## Deliverables

1. Support preposed static conditions in `StaticSpec.conditional` (or `CondMarking`) so that
   `stepStaticSpecWith` steps `condition` before `spec` when the condition is preposed, threading
   the condition's DRS into `spec`.
2. Author *Angelic Cub*'s second ability (`"As long as this creature has three or more +1/+1
   counters on it, it has flying"`) on `plugins_v3/canon/cards/Angelic Cub.lean` with `"it"` in the
   grant resolving to `thisCreature` in the preposed condition.

## Acceptance

- `plugins_v3/canon/cards/Angelic Cub.lean` includes both printed abilities and passes `Card.check`.
- `cargo xtask lean check` pass.

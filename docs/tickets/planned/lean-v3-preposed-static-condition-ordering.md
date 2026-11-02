---
needs: [lean-generated-canon-modules, semantics-rust-rewrite]
---
# Preposed condition ordering on `StaticSpec.conditional` and Angelic Cub's second ability

Standard constraints apply.

## Context

The checker (Lean's `lean/Semantics/Check/Phrase.lean`, ported to `discourse`) steps
`StaticSpec.conditional spec cond _` with `spec` always before `cond`. For postposed static
conditions (`"This creature has flying as long as it has three or more +1/+1 counters on it"`),
`spec` introduces the antecedent (`thisCreature`) and `cond` resolves `"it"` against it. For
preposed static conditions (`"As long as this creature has three or more +1/+1 counters on it, it
has flying"`, as printed on *Angelic Cub*), the condition clause introduces the antecedent
(`thisCreature`) and the static grant reads `"it"` from the condition.

Because `StaticSpec.conditional` currently lacks a preposed-condition ordering mode (analogous to
`Action.doIf` vs. preposed conditional actions), `plugins_v3/canon/cards/Angelic Cub.syl`
currently authors only its triggered ability.

## Deliverables

1. `discourse` supports preposed static conditions in `StaticSpec.conditional` (or `CondMarking`):
   it steps `condition` before `spec` when the condition is preposed, threading the condition's
   DRS into `spec`.
2. Author *Angelic Cub*'s second ability (`"As long as this creature has three or more +1/+1
   counters on it, it has flying"`) on `plugins_v3/canon/cards/Angelic Cub.syl` with `"it"` in the
   grant resolving to `thisCreature` in the preposed condition, and drop the card's "Fragment"
   comment.

## Acceptance

- `plugins_v3/canon/cards/Angelic Cub.syl` includes both printed abilities and checks clean in
  `discourse`, with a per-card test that the grant's "it" resolves to `thisCreature`.
- `cargo test` and `cargo clippy --tests -- -D warnings` for `semantics` and `discourse`.

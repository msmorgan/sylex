---
needs: []
---

# Engine: a lowered `Fight` fights

## Why

The `Fight` macro lowers each damage half with its amount bound first, so the `If` guard's
`then` is `Simultaneously([Sequentially([Let(power), DealDamage(..)]), Sequentially([Let(power),
DealDamage(..)])])`. `sylex_core::fight_body_fighters` (`crates/core/src/action.rs`) reads only bare
`DealDamage` halves and returns `None` for this shape, and both engine readers treat `None` as a
fizzle: the `"Fight"` arms of `composite_items` (`resolve/action.rs`) and `batch_act_head`
(`resolve/effect.rs`). Every fight lowered from a card therefore does nothing, silently: Pounce
with two legal targets deals no damage, and Affectionate Indrik never fights.

Reading the fighters through the leading `Let`s is not enough on its own. The body then reaches
the `Instruction::Simultaneously` arm of `resolve/effect.rs`, which panics on a `Sequentially`
member (`todo!`, owner `engine-simultaneous-member-breadth`). That ticket says no card is known
to need a `Sequentially` member; every lowered fight does.

## Done when

- `fight_body_fighters` reads each half's dealer through the `Let`s that bind its amount, and a
  `Fight` body that names no two fighters fails loudly at both engine readers instead of fizzling.
- The `Simultaneously` arm runs a `Sequentially([Let.., Act])` member: its `Let`s are bound before
  the batch is read, so both halves read the fighters' powers before any damage is dealt and land
  as one batch of noncombat damage [CR#701.14a,701.14d].
- `engine-simultaneous-member-breadth` names this shape as handled.
- Tests through lowered cards: Pounce with two legal targets has each creature dealt the other's
  power and records the fight; Affectionate Indrik (spelled in the test, as Pounce's opposing
  responses are) fights its target when it is still on the battlefield as its trigger resolves,
  and fights nothing when it has been exiled first [CR#701.14b].

## Pointers

- `crates/core/src/action.rs` (`fight_body_fighters`)
- `crates/engine/src/resolve/action.rs` (`composite_items`, `"Fight"`),
  `crates/engine/src/resolve/effect.rs` (`batch_act_head`, the `Simultaneously` arm)
- `v1/plugins/builtin/macros/effect/Fight.ron`; `crates/engine/tests/illegal_targets.rs` (Pounce)

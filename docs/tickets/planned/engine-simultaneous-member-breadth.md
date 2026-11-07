---
needs: []
---
# Engine: decide which further `Simultaneously` member shapes real cards need

## Why

The `Instruction::Simultaneously` arm in `crates/engine/src/resolve/effect.rs` evaluates every
member against one snapshot and voids the whole set if any member comes up empty ("if the entire
exchange can't be completed, no part of the exchange occurs" [CR#701.12a]). It handles two member
shapes: an `Act` verb, lowered to events, and an `Instruction::Continuously` whose body is a bare
`StaticSpec::Modify(reference, change)`. Avarice Totem's control exchange uses that second shape:
its reference is evaluated read-only first and the continuous effect is created after.

Every other member panics:

- a `Continuously` with any other `StaticSpec` body (`Each`, `Deontic`, `CostModifier`,
  `CantHappen`, `Prevention` and the rest). Only `Modify` names one object that can be evaluated
  and tested for emptiness before anything is created, so "did this member come up empty" has no
  answer for the others;
- any other `Instruction` member: `Sequentially`, `If`, `May`, a nested `Simultaneously` and the
  rest.

No card is known to need either shape.

## Done when

- The in-scope corpus has been searched for cards that need each shape. Each shape a card needs
  has its own ticket naming that card; a shape no card needs gets none.
- Each panic's `owner:` text names the ticket that now owns its shape, and a shape no card needs
  says so in its panic.

## Not in scope

- An `Act` member whose verb lowers to a decision rather than events:
  `engine-simultaneous-choice-members`.

## Pointers

- `crates/engine/src/resolve/effect.rs` (the `Instruction::Simultaneously` arm and its Avarice
  Totem tests).
- `crates/core/src/effect.rs` (`Instruction`), `crates/core/src/continuous.rs` (`StaticSpec`).

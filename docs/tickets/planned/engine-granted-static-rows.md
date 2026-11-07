---
needs: []
---
# Continuous effects created by resolution, by static-spec kind

## Why

The `Instruction::Continuously` arm in `crates/engine/src/resolve/effect.rs` mints a continuous
effect from a resolving spell or ability [CR#611.2] for only some `StaticSpec` kinds: `Modify`,
`Each(SelectAll, Modify(<candidate register>, _))`, `Deontic`, `CostModifier` and `CantHappen`.
Every other kind trips a `todo!`:

- `Each(SelectAll, inner)` whose inner spec is not a `Modify` of the candidate;
- `Each(sel, _)` for any selection other than `SelectAll`;
- the catch-all: `Conditionally`, `Role`, `CostOption`, `TriggerMultiplier`, `ModifyPlayer`,
  `Replacement`, `CantPrevent`, `SpendAsThough`, `AsThough`, `ConditionallyDo`, `OutcomeGate`,
  `ReplaceRoll` and `PayPips`.

## Done when

- Each kind in the catch-all that a Vintage-legal card can create by resolution is listed with
  that card, and has its own mint path and a test grounded in the card.
- The two `Each` shapes are wired for the cards that need them, each with a test.
- A kind no card can create by resolution keeps failing loudly, naming the kind.

## Not in scope

- `Prevention`: `engine-granted-prevention-rows`.
- `BecomesCopy`: `engine-layers-1-copy-facedown-text`.
- The positive "as though it had flash" counterfactual (Alchemist's Refuge):
  `engine-asthough-had-flash`.

## Pointers

- `crates/engine/src/resolve/effect.rs`: the `Instruction::Continuously` arm.
- `crates/core/src/continuous.rs`: `StaticSpec`.
- `crates/engine/src/layer.rs`: how `ContinuousEffect.changes` and `rows` are read.

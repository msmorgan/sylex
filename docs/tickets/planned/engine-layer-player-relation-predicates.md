---
needs: []
---

# Engine: a static's scope never matches a player through a relation predicate

## Why

Elesh Norn, Grand Cenobite says "Creatures your opponents control get -2/-2." Its scope is
`And([Creature, ControlledBy(OpponentOf(Ref(You)))])`. In a two-player game, P0's Elesh Norn
leaves P1's Grizzly Bears at 2/2 instead of 0/0 [CR#102.2,613.4c]; its "Other creatures you
control get +2/+2." half applies.

In the layer pass, `matches_derived` (`crates/engine/src/layer.rs`) matches `ControlledBy(inner)`
by running `inner` against the controller's player proxy. A player proxy has no entry in the
working map, so `inner` goes to `static_reference_matches`, which matches a `Ref` register or
`Any` and returns false for every other predicate. So `OpponentOf(…)` never matches, and neither
does any other player predicate (a relation or a combinator over one) under `ControlledBy` or
`Owner` in a static's scope.

## Done when

- A player predicate under `ControlledBy` or `Owner` in a layer-pass scope matches as it does
  outside the pass, reading the "you" derived so far in the pass [CR#109.5,613.1b].
- Tests: P0's Elesh Norn makes P1's Grizzly Bears 0/0 and P0's 4/4. When P1 controls the Norn
  (a locked `SetController`), it makes P0's Bears 0/0 and P1's 4/4.

## Pointers

- `crates/engine/src/layer.rs` (`matches_derived`, `static_reference_matches`).
- `v1/plugins/builtin/macros/filter/CreatureOpponentControls.ron`.

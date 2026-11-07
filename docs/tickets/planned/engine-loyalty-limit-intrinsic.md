---
needs: []
---
# Apply the loyalty once-per-turn gate as a rule, not a card limit

## Why

Only one loyalty ability of a permanent may be activated each turn [CR#306.5d,606.3]. That is a
rule of the game, never printed on a card, yet core spells it as `UseLimit::LoyaltyOncePerTurn`,
which every loyalty ability must carry for the engine to enforce it. The engine already has to
recognize loyalty abilities from their cost (`is_loyalty_ability` in `activate.rs`) to find a
permanent's other loyalty abilities, and core's `Ability::mana_profile` reads the same variant to
keep loyalty abilities from being mana abilities [CR#605.1a]. The variant is redundant authoring
surface: whether an ability is a loyalty ability follows from its cost.

## Done when

- `UseLimit::LoyaltyOncePerTurn` no longer exists in `sylex_core`.
- `can_activate` blocks every loyalty ability of a permanent once any of its loyalty abilities was
  activated this turn, whatever limits the ability carries [CR#606.3].
- `mana_profile` excludes loyalty abilities by the same loyalty-ability test the engine uses, so
  the two cannot diverge [CR#605.1a].
- The `LoyaltyOncePerTurn` arms in `step/trigger.rs` and `payment/mana.rs` are gone.
- The Jace Beleren proof in `tests/planeswalker_jace.rs` still blocks a second loyalty activation
  in one turn, and a test shows two different loyalty abilities of one permanent share the gate.

## Not in scope

- The `v1/` lowering and renderer change only as far as compiling against the new core needs.

## Pointers

- `crates/core/src/ability.rs` (`UseLimit`, `mana_profile`)
- `crates/engine/src/activate.rs` (`is_loyalty_ability`, `can_activate`)
- `crates/engine/src/step/trigger.rs`, `crates/engine/src/payment/mana.rs`
- `crates/engine/tests/planeswalker_jace.rs`, `crates/engine/tests/payment_mana.rs`
- `plugins/builtin/primitives/cost/loyaltySymbol.syl`: in `.syl` a loyalty cost is its own
  primitive

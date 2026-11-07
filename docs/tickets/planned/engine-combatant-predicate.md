---
needs: []
---
**Predicates can test the Combatant role, and the fight guard and creature state-based actions
read it instead of the Creature type.**

## Why

The Combatant role is conferred (`StaticSpec::Role`) and drives attack, block and
summoning-sickness rules through `is_combatant`, but core's `Predicate` has no atom that tests a
role. So the fight both-or-neither guard [CR#701.14b] and the toughness and lethal-damage
state-based actions [CR#704.5f,704.5g] still test `Type(Creature)` as a stand-in for the role.

## Done when

- Core's `Predicate` has a role test (`HasRole(Combatant)`), evaluated by the engine through the
  same conferred-role read as `is_combatant`.
- The fight guard and the two creature state-based actions test the role, and no `Type(Creature)`
  proxy for the role remains in them.
- Tests: a fight (Prey Upon) where one fighter stops being a Combatant before resolution deals no damage; a
  Combatant with toughness 0 is put into its owner's graveyard.

## Pointers

- `crates/core/src/continuous.rs`: `Role`; `crates/core/src/filter.rs`: `Predicate`.
- `crates/engine/src/legal.rs`: `is_combatant`.
- `crates/engine/src/condition.rs`: the fight guard's gone-object branch.
- `plugins/builtin/definitions/card_type/creature.syl`,
  `plugins/builtin/definitions/keyword_action/fight.syl`: where `.syl` declares these rules.
- [Game Model glossary](../../contexts/game-model/CONTEXT.md): Combatant.

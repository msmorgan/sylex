---
needs: []
---
# Count a player's party

## Why

A party is up to one each of a Cleric, a Rogue, a Warrior and a Wizard the player controls
[CR#700.8]; its size, 0 to 4, is computed by the game, not declared [CR#700.8a]. A creature with
several of those types fills only one role, counted the way that gives the highest result
[CR#700.8b], so party size is a maximum matching of creatures to roles. Neither `CountDistinct`
(a lone Cleric Wizard such as Shieldmage Elder gives 2, not 1) nor a creature count (two Clerics
give 2, not 1) computes it, and no other `Count` does. The `.syl` builtins already spell the
party as `oneEachOf(<your creatures>, partyRoles)`, with `partySizeOf` as `numberOf` of that group
and `fullPartyOf` as size equal to 4 [CR#700.8c]; core and the engine cannot express or evaluate
it.

## Done when

- Core expresses the number of members of a `oneEachOf(pool, roles)` group, with the roles taken
  from the group rather than hard-coded.
- Both count evaluators (`resolve/count.rs` and `layer.rs`) compute it as the maximum matching of
  pool members to roles, each member filling at most one role.
- Tests: a lone Shieldmage Elder gives party size 1; Shieldmage Elder with a Wizard gives 2; four
  creatures covering the four roles give a full party; Archpriest of Iona's power tracks party
  size through the layer evaluator.

## Not in scope

- Stick Together's chosen party [CR#700.8d], where players choose the members.

## Pointers

- `plugins/builtin/templates/noun/partyOf.syl`, `predicate/partyRoles.syl`,
  `amount/partySizeOf.syl`, `condition/fullPartyOf.syl`
- `plugins/builtin/primitives/determiner/oneEachOf.syl`
- `crates/core/src/count.rs` (`Count`, `CountDistinct`)
- `crates/engine/src/resolve/count.rs`, `crates/engine/src/layer.rs`

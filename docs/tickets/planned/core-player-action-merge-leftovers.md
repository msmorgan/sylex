---
needs: []
---

# Comments and names left behind by the `PlayerAction` merge

## Why

Core's `PlayerAction` enum and its `By(who, …)` wrapper were merged into `Action`, but
comments and one file name still describe the old shape. A reader is sent looking for types that
do not exist.

## Done when

- `crates/engine/src/resolve/player_action.rs` and its `player_action_items` are named for what
  they lower (the player-performed verbs), and no comment in `crates/core` or `crates/engine` calls
  them the "former `PlayerAction`" family.
- No comment in `crates/core` or `crates/engine` spells an action as `By(who, …)` or names
  `PlayerAction` (`rg 'PlayerAction|By\(' crates/core crates/engine` finds only live code).
- `Action`'s doc comment in `crates/core/src/action.rs` states the current shape as plain fact,
  with no reshape history or date.

## Not in scope

`v1/`, which is deleted at parity.

## Pointers

`crates/engine/src/resolve/{player_action,action}.rs`,
`crates/core/src/{action,copy}.rs`, `crates/engine/src/{cast,replace_registry}.rs`.

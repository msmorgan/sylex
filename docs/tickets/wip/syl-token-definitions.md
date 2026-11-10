---
needs: [lean-retirement, syl-single-face-card-call]
---

# Predefined tokens are `token` Definitions

## Why

The rules define each predefined token's characteristics [CR#111.10], so each is a
Definition, not a Template. The `.syl` `token` declaration kind exists (sort `PredefinedToken`)
but nothing declares one, and `plugins/builtin/definitions/token/` does not exist. Clue, Food and
Treasure are written as `template … : CharacteristicBundle` under `templates/noun/`, and core
keeps a second hand-synchronized copy of every predefined token's characteristics
(`PredefinedToken` in `crates/core/src/token.rs`).

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

## Done when

- Each predefined token [CR#111.10a..111.10x] is a `token` declaration under
  `plugins/builtin/definitions/token/`. The declaration's name is the key an effect writes ("create
  a Clue token"), not its subtype: Role tokens are told apart by name, and a Walker is a Zombie
  [CR#111.10d].
- `create` takes a token Definition, wrapped as `defined(clueToken)`, where it now takes
  `written(clueToken)`; `investigate` and
  the canon cards that create Treasure (*Ragavan, Nimble
  Pilferer*, *Spell Swindle*, *Zidane, Tantalus Thief*) use it; `clueToken.syl`, `foodToken.syl`
  and `treasureToken.syl` are deleted.
- A Predicate over a token Definition, `hasSubtypesOf(token)`, tests for the subtypes it declares,
  so a rule written over "a <subtype> token" takes the subtype from the Definition it creates
  (needed by `syl-empower-token-parameter`).
- A token a keyword action's rule defines in full, as [CR#701.71a] defines the Jace planeswalker
  token, is a `token` Definition too, and the glossary names the term that covers both.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

Removing core's `PredefinedToken`, once the engine's input comes from `.syl`.

## Pointers

`plugins/builtin/templates/noun/{clueToken,foodToken,treasureToken}.syl`,
`plugins/builtin/definitions/keyword_action/{investigate,empowerJace}.syl`, `crates/core/src/token.rs`,
`crates/xtask/src/dsl_check/schema.rs` (the `token` kind).

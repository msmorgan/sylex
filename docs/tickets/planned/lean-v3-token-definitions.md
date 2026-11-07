---
needs: [lean-retirement]
---

# Predefined tokens are `token` Definitions

**Why.** The rules define each predefined token's characteristics [CR#111.10], so each is a
Definition, not a Template. The `.syl` `token` declaration kind exists (sort `PredefinedToken`)
but nothing declares one, and `plugins/builtin/definitions/token/` does not exist. Clue, Food and
Treasure are written as `template … : CharacteristicBundle` under `templates/noun/`, and core
keeps a second hand-synchronized copy of every predefined token's characteristics
(`PredefinedToken` in `crates/core/src/token.rs`).

**Done when.**
- Each predefined token [CR#111.10a..111.10x] is a `token` declaration under
  `plugins/builtin/definitions/token/`. The declaration's name is the key an effect writes ("create
  a Clue token"), not its subtype: Role tokens are told apart by name, and a Walker is a Zombie
  [CR#111.10d].
- `create` takes a token Definition where it now takes `written(clueToken)`; `investigate`,
  `giftAFood`, `giftATreasure` and the canon cards that create Treasure (*Ragavan, Nimble
  Pilferer*, *Spell Swindle*, *Zidane, Tantalus Thief*) use it; `clueToken.syl`, `foodToken.syl`
  and `treasureToken.syl` are deleted.
- A token Definition's declared characteristics, at least its subtype, can be named from `.syl`,
  so a rule written over "a <subtype> token" can take the subtype from the Definition it creates
  (needed by `lean-v3-empower-token-parameter`).
- A token a keyword action's rule defines in full, as [CR#701.71a] defines the Jace planeswalker
  token, is a `token` Definition too, and the glossary names the term that covers both.
- Core's hardcoded predefined tokens are gone; the token Definitions are the one copy.
- Everything passes the `.syl` checks.

**Pointers.** `plugins/builtin/templates/noun/{clueToken,foodToken,treasureToken}.syl`,
`plugins/builtin/definitions/keyword_action/{investigate,empowerJace}.syl`,
`plugins/builtin/templates/ability/{giftAFood,giftATreasure}.syl`, `crates/core/src/token.rs`,
`crates/xtask/src/dsl.rs` (`kind_sort`).

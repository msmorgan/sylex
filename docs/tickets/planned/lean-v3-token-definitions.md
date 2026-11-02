---
needs: [semantics-rust-rewrite]
---
# Predefined tokens are `token` Definitions

The rules define the Predefined Tokens' characteristics [CR#111.10], so by the glossary each is a
Definition, not a Template. The pieces for that exist but are unused: the `PredefinedToken`
structure (`lean/Semantics/Rules.lean` today, Rust `semantics` after the port), the `.syl` `token`
declaration (sort `PredefinedToken`, checked like the other rule-registry declarations), and `plugins_v3/builtin/definitions/token/`,
which semantics-v3 §4.5 reserves and which does not exist. Meanwhile Clue, Food and Treasure are
written as `template … : CharacteristicBundle` under `templates/noun/`, and the Lean proof suite
(`lean/Semantics/Proofs/Rules.lean`, carried to Rust tests by `semantics-rust-rewrite`) keeps its
own hand-written `treasure`, `food`, `gold`, `clue` and `blood` values that nothing creates from.

## Change

- Declare each Predefined Token [CR#111.10a..111.10x] as a `token` under
  `definitions/token/`, starting with the three in use (Clue, Food, Treasure). A Role token is told
  apart from the other Roles by its name, Walker by its name (it is a Zombie [CR#111.10d]), so the
  declaration's name is the key an effect writes ("create a Clue token"), not its subtype.
- `create` takes a token Definition where it now takes `written(clueToken)`; update `investigate`,
  `giftAFood`, `giftATreasure` and the canon cards that create Treasure (Ragavan, Spell Swindle,
  Zidane). Delete `clueToken.syl`, `foodToken.syl` and `treasureToken.syl`.
- Delete the duplicate values in the carried-over tests; checks of the `token` declarations
  replace them.
- A token Definition's declared characteristics can be named from `.syl`, at least its subtype, so
  a rule written over "a <subtype> token" can take the subtype from the Definition it creates
  (needed by `lean-v3-empower-token-parameter`).
- Widen `PredefinedToken`'s doc and the glossary's **Predefined Token** entry to also cover a token
  a keyword action's rule defines in full, as [CR#701.71a] defines the Jace planeswalker token, or
  name that a separate term; it is a Definition either way.

## Relation to other tickets

`tokens-design` and `tokens-templates` (maybe/) cover ad-hoc token authoring and extraction from
Oracle text. This ticket is only the rules-defined tokens.

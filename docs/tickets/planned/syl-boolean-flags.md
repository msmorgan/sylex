---
needs: [syl-single-face-card-call]
---

# No meaning carried by a `Bool` parameter

## Why

Several declarations say in a `Bool` what a typed construct or a separate declaration should say:
`addedCost(cost, offered: Bool, …)`, `characteristics(values, typesAdded: Bool)` and
`copyCharacteristics`, `bySymbols(…, coloredOnly: Bool)` and `lessBySymbols`, `whenSpentOn`'s
`only`, `asCopyOf`'s `optional`, and `blocked(value: Bool)`. A call such as `addedCost(cost, true)`
can't be read without the declaration.

## Done when

- Each listed `Bool` parameter is replaced by a typed construct or split into declarations named
  for what they mean, with every caller in `plugins/` respelled; any kept states why in its doc
  comment.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- `doIf`'s `postposed`: `syl-printed-read-order`.
- `addedCost`'s offered form as enlist needs it: `syl-enlist-optional-attack-cost` (whichever
  lands first replaces it).
- `card_type`'s `permanentType`: `syl-card-type-permanent-type`.

## Pointers

`plugins/builtin/primitives/static/{addedCost,characteristics}.syl`,
`plugins/builtin/templates/noun/copyCharacteristics.syl`,
`plugins/builtin/primitives/cost/bySymbols.syl`, `plugins/builtin/templates/cost/lessBySymbols.syl`,
`plugins/builtin/primitives/mana/whenSpentOn.syl`,
`plugins/builtin/primitives/zone/asCopyOf.syl`, `plugins/builtin/primitives/action/blocked.syl`.

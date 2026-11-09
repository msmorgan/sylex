---
needs: []
---

# `cardClass` becomes a `permanentType` bool on `card_type`

## Why

A `card_type` declaration carries `cardClass: Option(CardClass)`: `permanentCard` on the six
permanent types, `instantOrSorceryCard` on `instant` and `sorcery`, and nothing on `kindred` and
`dungeon` (`plugins/builtin/definitions/card_type/`, ten files). The rules define the six
permanent types on the card type itself [CR#110.4] (glossary: **Permanent Type**), and a
permanent card is defined by its types [CR#110.4a]. Whether a card is one belongs to a face
through its types, since a card's faces can differ, so a "card class" names nothing the rules
define.

## Design

- Every `card_type` declaration carries a `permanentType` Bool: `true` on `artifact`, `battle`,
  `creature`, `enchantment`, `land` and `planeswalker`, `false` on `instant`, `sorcery`, `kindred`
  and `dungeon` [CR#110.4].
- The `CardClass` sort and its primitives `permanentCard` and `instantOrSorceryCard`
  (`plugins/builtin/primitives/cardClass/`) are deleted.
- `byCardClass` (`plugins/builtin/primitives/ability/byCardClass.syl`) is renamed to match.

## Open questions

- Whether `permanentType` is required or defaulted.
- The exact new name for `byCardClass`.

## Done when

- `crates/xtask/src/dsl_check/schema.rs` declares `permanentType` at `Bool` on `card_type`,
  required or defaulted as settled, in place of `cardClass`, and its "Step 6" note there is gone;
  if required, a negative test refuses a `card_type` without it.
- All ten `card_type` declarations write `permanentType` (or rely on the settled default),
  `plugins/builtin/primitives/cardClass/` is deleted, and `CardClass` is not a known sort.
- `byCardClass` is renamed as settled, and its three callers,
  `plugins/builtin/definitions/keyword_ability/{ascend,haunt,gift}.syl`, call the new name.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`crates/xtask/src/dsl_check/schema.rs` (`card_type`), `plugins/builtin/definitions/card_type/`,
`plugins/builtin/primitives/cardClass/`, `plugins/builtin/primitives/ability/byCardClass.syl`,
`docs/contexts/game-model/CONTEXT.md` (**Permanent Type**).

---
needs: [syl-single-face-card-call]
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

Written against the start of step 6 (`syl-single-face-card-call`).

## Design

- Every `card_type` declaration carries a `permanentType` Bool: `true` on `artifact`, `battle`,
  `creature`, `enchantment`, `land` and `planeswalker`, `false` on `instant`, `sorcery`, `kindred`
  and `dungeon` [CR#110.4].
- The `CardClass` sort and its primitives `permanentCard` and `instantOrSorceryCard`
  (`plugins/builtin/primitives/cardClass/`) are deleted.
- `permanentType` is required: the ten declarations split six `true` and four `false`, and for
  `kindred` an explicit `false` is a rules statement [CR#110.4], not a blank.
- `byCardClass` (`plugins/builtin/primitives/ability/byCardClass.syl`) is renamed
  `ifPermanentType(onPermanent, onInstantOrSorcery)`: one Ability when the object is a permanent,
  one when it is an instant or sorcery spell, as the CR words each keyword
  [CR#702.131a,702.131b], [CR#702.55a], [CR#702.174b]. It is defined for those two cases only.
- `plugins/builtin/templates/predicate/isPermanentType.syl` reads the field instead of listing the
  six types by hand, if today's `.syl` vocabulary can express it.

## Done when

- `crates/xtask/src/dsl_check/schema.rs` declares `permanentType` at `Bool` on `card_type`,
  as a required field in place of `cardClass`, and its "Step 6" note there is gone; a negative
  test refuses a `card_type` without it.
- All ten `card_type` declarations write `permanentType`,
  `plugins/builtin/primitives/cardClass/` is deleted, and `CardClass` is not a known sort.
- `byCardClass` is renamed `ifPermanentType`, and its three callers,
  `plugins/builtin/definitions/keyword_ability/{ascend,haunt,gift}.syl`, call the new name.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`crates/xtask/src/dsl_check/schema.rs` (`card_type`), `plugins/builtin/definitions/card_type/`,
`plugins/builtin/primitives/cardClass/`, `plugins/builtin/primitives/ability/byCardClass.syl`,
`docs/contexts/game-model/CONTEXT.md` (**Permanent Type**).

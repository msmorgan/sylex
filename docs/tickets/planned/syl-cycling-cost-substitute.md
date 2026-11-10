---
needs: [cleanup-step6-surface]
---

# New Perspectives replaces the cycling cost, not the cycling ability

## Why

*New Perspectives* prints "As long as you have seven or more cards in hand, you may pay {0} rather
than pay cycling costs." Its `.syl` writes the static `altCost` with the whole cycling ability,
`allOf(abilityHead(keyword(cycling)))`, as its subject. But `altCost` is "You may <cost> rather
than pay <subject>'s mana cost", an alternative cost of a spell [CR#118.9], and cycling's cost is
only the `[Cost]` part of "[Cost], Discard this card: Draw a card" [CR#702.29a]: a player who pays
{0} instead still discards the card.

## Done when

- *New Perspectives*' static names what it replaces, the cycling cost, so the discard stays part of
  the cost [CR#702.29a].
- Nothing writes an alternative cost for an ability through `altCost`'s "mana cost" reading.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/canon/cards/New Perspectives.syl`, `plugins/builtin/primitives/static/altCost.syl`,
`plugins/builtin/definitions/keyword_ability/cycling.syl`.

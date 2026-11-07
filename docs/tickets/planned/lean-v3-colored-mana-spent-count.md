---
needs: [lean-v3-spell-abilities-share-targets, plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---
# "If at least three [color] mana was spent to cast this spell"

## Why

Adamant conditions count the mana of one type spent to cast a spell, and `.syl` has no amount
for that. The Linked Facets today are `colorsSpent` (how many colors) and `manaValueSpent` (how
much in total). Neither counts mana of one stated color, nor the most mana of any one color.

- Stated color: *Searing Barrage* ("If at least three red mana was spent to cast this spell,
  Searing Barrage deals 3 damage to that creature's controller"), *Rally for the Throne* (white),
  *Unexplained Vision* (blue).
- Colorless: *Desecrate Reality* ("If at least three colorless mana was spent to cast this
  spell, …").
- Same color: *Clockwork Servant* ("When this creature enters, if at least three mana of the
  same color was spent to cast it, draw a card"), *Henge Walker* ("If at least three mana of the
  same color was spent to cast this spell, this creature enters with a +1/+1 counter on it").

## Design

- One Linked Facet, `manaSpent(type: ManaTypeTerm)`: the amount of mana of that type spent to cast
  the object, read through `paid(facet, subject)`. Colorless is a mana type [CR#106.1b], so the
  same facet serves *Desecrate Reality*.
- "Mana of the same color" is its own facet, the greatest amount of mana of any one color spent to
  cast the object, not a condition quantifying over colors. It ranges over the five colors only
  [CR#105.1,106.1b], so `{C}{C}{C}` never satisfies it.

## Done when

- Both facets exist, each with a Template reading it for a spell beside `colorsSpentToCast`, the
  same-color facet named against the game-model glossary.
- The six cards above are spelled in full in `plugins/canon/cards/`, each under `adamant`.

## Pointers

- `plugins/builtin/primitives/quantity/colorsSpent.syl`, `manaValueSpent.syl`, `paid.syl`
- `plugins/builtin/templates/amount/colorsSpentToCast.syl`, `manaValueSpentToCast.syl`
- `plugins/builtin/templates/ability_word/adamant.syl`, `plugins/builtin/primitives/mana_type/ofColor.syl`
- Glossary: **Linked Facet** (`docs/contexts/game-model/CONTEXT.md`)

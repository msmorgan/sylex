---
needs: [lean-v3-spell-abilities-share-targets, plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---
# "If at least three [color] mana was spent to cast this spell"

Standard constraints apply. Folded in: "at least three mana of the same color"
(was `lean-v3-same-color-mana-spent-count`, section below). Both add a `LinkedFacet`.

## Problem

Adamant is an ability word [CR#207.2c]. It has no rules meaning of its own: the condition is
written out in each ability it heads, "if at least three [color] mana was spent to cast this
spell". The checker has no amount for how much mana of one color was spent. `LinkedFacet` (the
AST, `lean/Semantics/Abilities.lean` today, Rust `semantics` after the port) has only
`colorsSpent` (how many colors of mana were spent, `colorsSpentToCast`) and `manaValueSpent` (how
much mana was spent in total, `manaValueSpentToCast`). Neither counts the mana of one stated color.

## Witnesses

- *Searing Barrage* {4}{R}: "Searing Barrage deals 5 damage to target creature. / Adamant — If at
  least three red mana was spent to cast this spell, Searing Barrage deals 3 damage to that
  creature's controller."
- *Rally for the Throne* {2}{W}: "Create two 1/1 white Human creature tokens. / Adamant — If at
  least three white mana was spent to cast this spell, you gain 1 life for each creature you
  control."
- *Unexplained Vision* {4}{U}: "Draw three cards. / Adamant — If at least three blue mana was spent
  to cast this spell, scry 3."

## Ruling (owner, 2026-10-24)

Use `ManaTypeTerm` [CR#106.1b] (`.colorless | .ofColor (color : ColorTerm)`, verified).

## Change

- Add `LinkedFacet.manaSpent (type : ManaTypeTerm)`, the amount of mana of that type spent to cast
  the object. Templates beside `colorsSpentToCast`
  (`plugins/builtin/templates/amount/colorsSpentToCast.syl`): `manaOfColorSpentToCast` and
  `colorlessManaSpentToCast`. Adamant (`plugins/builtin/templates/ability_word/adamant.syl`)
  then reads `compareAmt (manaOfColorSpentToCast .red thisSpell) .atLeast 3`.
- The same facet covers *Desecrate Reality*'s "three colorless mana".
- "Three mana of the same color" is the next section.

## Same color (owner ruling 2026-10-25)

*Clockwork Servant* and *Henge Walker* print adamant conditions on "at least three mana of the same
color" (*Clockwork Servant*: "When this creature enters, if at least three mana of the same color
was spent to cast it, draw a card."; *Henge Walker*: "If at least three mana of the same color
was spent to cast this spell, this creature enters with a +1/+1 counter on it."). No color is
stated, so `manaSpent` with a `ManaTypeTerm` cannot name it. The condition quantifies over colors:
some color has at least three mana spent of it.

**Ruled 2026-10-25: a dedicated facet**, not a condition quantifying over colors: `ColorTerm`
(`lit`, `chosen`, `colorOf`) has no binder, so quantifying would need new binder machinery in
Core and the checker. Add a Linked Facet (alongside `LinkedFacet.colorsSpent` [CR#702.44a]) for
the greatest amount of mana of any one color spent to cast it, with a Template reading it for the
spell, so both cards compare it `.atLeast 3`. It ranges over the five colors [CR#105.1]; colorless
mana is a mana type but not a color [CR#106.1b] and does not count. It stays distinct from the
stated-type count above. Name the facet per the game-model glossary.

## Tests

- Each witness spelled in full under `adamant`.
- *Desecrate Reality*'s "three colorless mana" via `colorlessManaSpentToCast`.
- *Clockwork Servant* and *Henge Walker* spelled in full.
- A test that a stated-color count does not satisfy the same-color form and vice versa.

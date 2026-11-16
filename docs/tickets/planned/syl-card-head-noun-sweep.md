---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement, syl-single-face-card-call, cleanup-step6-surface]
---
# Canon descriptions write their printed head nouns

## Why

`syl-nominal-descriptions` folded every item of this sweep: restoring printed "permanent" and
"card" heads across `plugins/canon/cards/` (*Seedborn Muse*, *Blatant Thievery*, *Mutagen
Connoisseur*, *Force of Will*, *Sunscour*, the five Shoals, *Drach'Nyen*, *Aven Shrine*, *Rune
Snag*, *Tower Winder*, *Vraska's Scorn* and the type-before-card sites), writing a printed "on the
battlefield" as the postmodifier `in(battlefield)` (*Topple*, *Éomer of the Riddermark*, *Caller of
the Hunt*, *Celestial Judgment*, *Chain Reaction*, *Coat of Arms*, *Stag Beetle*, *Clone*,
*Quicksilver Gargantuan*, *Sculpting Steel*), dropping
unprinted card types beside subtypes, and restoring the heads on `hexproof` and the other builtin
keyword bodies.

## Done when

- Satisfied in full by `syl-nominal-descriptions`; ready to close.

## Not in scope

- A card not yet in `plugins/canon/cards/` is spelled with its head nouns when it is written.
- Printed "it" written as a self-reference (`thisCreature` for "it"):
  `syl-printed-it-self-references`.
- Type words before a dropped "card", and unprinted card types beside a subtype:
  `syl-nominal-descriptions`.

## Pointers

- `plugins/builtin/primitives/nominal/{card,permanent}.syl`,
  `plugins/builtin/primitives/postModifier/in.syl`,
  `plugins/builtin/templates/postModifier/{youControl,controlledBy}.syl`,
  `plugins/builtin/definitions/keyword_ability/hexproof.syl`
- Glossary: **Head Noun**, **Nominal** (`docs/contexts/game-model/CONTEXT.md`)
- Oracle text: `data/mtgjson/AllPrintings.sqlite`

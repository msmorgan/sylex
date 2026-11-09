---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement, syl-single-face-card-call]
---
# Restore the printed head nouns that canon card spellings drop

## Why

Where Oracle prints "permanent" or "card", many `plugins/canon/cards/` spellings leave the head
noun out and write only the other conjuncts. That changes the set described:

- "Permanent": `youControl` and `controlledBy` (`plugins/builtin/templates/predicate/`) test only
  the controller, which a spell on the stack also has. *Seedborn Muse* ("Untap all permanents you
  control during each other player's untap step") writes `allOf(youControl)`; *Blatant Thievery*
  ("gain control of target permanent that player controls") writes
  `target(controlledBy(that(player)))`. A permanent is on the battlefield [CR#110.1];
  `isPermanent` says so.
- "Card": a description with a card type and a zone but no "card" admits tokens and copies where
  the text means a Magic card [CR#108.2,109.2a]. *Force of Will* ("exile a blue card from your
  hand") writes `and([hasColor(blue), in(yourHand)])`.

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

## Done when

- Every canon description where Oracle prints "permanent" carries `isPermanent`, and every one
  where it prints "card" carries `isCard` (or uses `cardIn(zone)`, which is
  `and([isCard, in(zone)])`). `isCard` goes immediately after the last card-type, subtype or
  color conjunct, as in `and([isCreature, isCard, in(yourHand)])`. Nothing else in a site changes:
  not its determiner, quantity, zone or other conjuncts, and no Template, Definition or primitive.
- The sweep covers at least these sites, found by scanning conjunctions with a non-battlefield
  zone and searches (the scan misses nested conjunctions, so check each card's Oracle text, not
  only the scan):
  - no head noun at all: *Seedborn Muse*, *Blatant Thievery* (`isPermanent`); *Drach'Nyen*
    (`powerOf(the(in(exiledWith(thisEquipment))))` → `cardIn`), *Aven Shrine*, *Vraska's Scorn*
    and *Tower Winder* (`named(…)`), *Force of Will*, *Sunscour*, the five Shoals (*Shining*,
    *Disrupting*, *Blazing*, *Sickening*, *Nourishing*), *Rune Snag* (`isCard`);
  - a type but no "card": *Abolish*, *Ashes of the Fallen*, *Bleeding Effect*, *Bloodfire
    Enforcers*, *Cairn Wanderer*, *Carrion Grub*, *Celebrate the Harvest*, *Charnel Troll*,
    *Crypt Lurker*, *Cultivator Colossus*, *Custodi Squire*, *Death-Mask Duplicant*, *Death or
    Glory*, *Delivery Moogle*, *Garruk Relentless // Garruk, the Veil-Cursed*, *Hymn of Rebirth*,
    *Land Tax*, *Midnight Scavengers // Chittering Host*, *Mox Diamond*, *Old-Growth Dryads*,
    *Phyrexian Ingester*, *Rogues' Gallery*, *Scapeshift*, *Sphinx of the Chimes*, *Syr Konrad,
    the Grim*, *Thawing Glaciers*, *Urborg Scavengers*, *Voracious Brood*.
- `rg 'allOf\(youControl\)' plugins/canon/cards` is empty.
- If adding a head noun makes a later reference in the card ("that card", "it") resolve
  differently or fail, the site is reported, not left without its head noun.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- A card not yet in `plugins/canon/cards/` is spelled with its head nouns when it is written.
- Printed "it" written as a self-reference (`thisCreature` for "it"):
  `syl-printed-it-self-references`.

## Pointers

- `plugins/builtin/templates/predicate/isPermanent.syl`, `cardIn.syl`, `youControl.syl`,
  `controlledBy.syl`; `plugins/builtin/primitives/predicate/isCard.syl`
- Glossary: **Head Noun** (`docs/contexts/game-model/CONTEXT.md`)
- Oracle text: `data/mtgjson/AllPrintings.sqlite`

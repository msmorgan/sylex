---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement, cleanup-step6-surface]
---
# Craft's body and its materials parameter

## Why

"Craft with [materials] [cost]" means "[Cost], Exile this permanent, Exile [materials] from among
permanents you control and/or cards in your graveyard: Return this card to the battlefield
transformed under its owner's control. Activate only as a sorcery." [CR#702.167a].
`craft.syl` takes only the cost, carries a `TODO(spelling)` saying the materials are folded into
it, and has `body = []`.

The printed materials take every noun-phrase shape: counted ("six artifacts", "one or more
creatures"), coordinated ("a Dinosaur, a Merfolk, a Pirate, and a Vampire", `oneEachOf`), or an
agreement ("two that share a card type", `agree`). The body must restrict whatever the card wrote
to "from among permanents you control and/or cards in your graveyard", and nothing today restricts
a `NounPhrase` parameter from outside. A `Location` cannot say it either: `andOr([battlefield,
yourGraveyard])` admits an opponent's permanents.

Written against the surface (`cleanup-step6-surface`).

## Open questions

- Which shape the materials take:
  1. a noun phrase over a pool placeholder that the body fills with the domain; or
  2. a Template the card calls builds the whole materials phrase with the domain as its
     `someOf`/`oneEachOf`/`agree` group, and the Definition receives that phrase.

## Done when

- `craft` takes a materials parameter separate from its cost, and its body writes the
  [CR#702.167a] ability in full.
- The body keeps the [CR#702.167b] exception: a material described only by a card type or subtype,
  without "card", is a permanent of that type on the battlefield or a card of it in a graveyard.
- The exiled materials stay readable by the crafted permanent's abilities [CR#702.167c].
- Canon holds craft cards covering a counted, a coordinated and an agreement materials phrase,
  chosen from the 24 Vintage-legal craft cards (*Braided Net // Braided Quipu*, *Clay-Fired Bricks
  // Cosmium Kiln*, …).
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

- `plugins/builtin/definitions/keyword_ability/craft.syl`
- `plugins/builtin/primitives/determiner/oneEachOf.syl`, `plugins/builtin/primitives/noun/agree.syl`,
  `plugins/builtin/templates/determiner/someOf.syl`, `plugins/builtin/templates/zone/andOr.syl`
- `data/derived/cards.jsonl` (the craft cards' printed phrasings)

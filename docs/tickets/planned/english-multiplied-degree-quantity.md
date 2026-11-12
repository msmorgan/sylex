---
needs: []
---
# *Twice that many cards* brackets the Noun Phrase, and *target* determines a quantity

## Why

Two defects in the quantity Productions:

- A multiplier with a numeral or variable forms the quantity (*[half X] damage*, *[twice X]
  damage*, *[three times X] cards*), but with a degree determiner it still takes the whole
  Noun Phrase: *twice [that many cards]* (Doubling Season), *twice [that much {G}]* (Fangorn,
  Tree Shepherd), *half [that many +1/+1 counters]* (Ochre Jelly), *three times [that many of
  those tokens]* (Ojer Taq, Deepest Foundation). *That many* and *that much* are built by
  `DegreeDeterminer`, which the multiplier Productions do not take as the number.
- `DeterminedQuantity` takes the Targeting Marker as the determiner of a quantified nominal,
  so Eternal Flame (*deals X damage to target opponent or planeswalker and half X damage,
  rounded up, to you*) has Readings with *target* over *[opponent or planeswalker and half] X
  damage*, the noun *half* a conjunct of a noun premodifier. No card prints *target* before a
  quantity.

## Design

- A multiplier with *that many* or *that much* forms the quantity, as with a numeral or
  variable: *[twice that many] cards*.

## Done when

- Each of the four phrases above has one tree, with the multiplier and the degree determiner
  one constituent, on every supported-corpus unit that prints the form; *twice that damage*,
  *double that damage* and *half their life*, where a Noun Phrase follows the multiplier, keep
  their Readings.
- `DeterminedQuantity` takes no Targeting determiner, and Eternal Flame keeps its Readings
  that coordinate *[X damage to target opponent or planeswalker]* with *[half X damage,
  rounded up, to you]*.
- A judgment on a real card's text pins each.
- The census shows no verdict change on the supported corpus, and every unit whose Readings
  change is checked; `cargo test -p sylex_english` and `cargo clippy -p sylex_english --tests
  -- -D warnings` pass.

## Pointers

- `crates/english/grammar/english.fug`: `DegreeDeterminer`, `MultiplierDeterminer`,
  `MultipliedDeterminer`, `AdverbMultipliedDeterminer`, `MultiplierPredetermination`,
  `MultipliedNounPhrase`, `AdverbMultipliedNounPhrase`, `DeterminedQuantity`,
  `QuantifiedNominalPremodifier`.
- `docs/tickets/done/english-quantity-bracketing.md`.

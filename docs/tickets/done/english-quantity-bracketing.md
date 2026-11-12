---
needs: []
---
# *Any one color* and *half X damage* each have two trees, and *three cards X* is an Amount

## Why

Three defects in the quantity Productions:

- *Any one color* (*add three mana of any one color*, Lotus Vale) has two trees of one
  analysis: `SingularQuantifier` builds the determinative phrase *[any one]* over *color*, and
  `DeterminedQuantity` builds *any* over the quantified nominal *[one color]*.
- *Half X damage* (Banshee; also *half X cards*, *half X life*, *half X Food tokens*) has two
  trees, `DeterminedQuantity` over the nominal *[X damage]* and `MultiplierPredetermination`
  over the Noun Phrase *[X damage]*. Neither brackets *half X* as the number it is, and
  *five times X damage* (Crackle with Power) is also `MultipliedNounPhrase`'s *five times [X
  damage]*. *Twice X damage* (Stonesplitter Bolt) is only `AdverbMultipliedNounPhrase`'s
  *twice [X damage]*.
- `MultipliedAmount` requires only that its multiplier be plural, where its siblings
  `MultipliedDeterminer` and `MultipliedNounPhrase` require a serial noun (*times*), so *three
  cards X* reads as an Amount as *three times X* does.

## Design

- *Any one* is a determinative phrase: *[any one] color* is the Reading, and *any [one color]*
  is not built.
- *Half X* is a quantity: *[half X] damage* is the Reading, with *half* multiplying the
  variable, and *half [X damage]* is not built by either Production. *Twice X* and *N times X*
  are quantities the same way: *[twice X] damage*, *[five times X] damage*.

## Done when

- *Any one color* has one tree, through *[any one]*, on every supported-corpus unit that
  prints *any one*; the Reading removed is removed where it is built, and *the two spells*
  (Powerbalance) and *that 1 damage* (Rock Hydra), which `DeterminedQuantity` also builds,
  keep their Readings.
- *Half X damage* and the other *half X* units have one tree, with *half X* a constituent;
  *half their life* and *double that damage* keep their Readings. *N times X* units have
  *[N times X]* as their only bracketing, and *three times this creature's power* keeps its Reading.
  *Twice X* units have *[twice X]* as their only bracketing, and *twice its power* and *twice
  the number of ...* keep their Readings.
- `MultipliedAmount` requires a serial multiplier, and *three cards X* is no Amount.
- A judgment on a real card's text pins each of the first two, with the tree count.
- The census shows no verdict change on the supported corpus, and every unit whose Readings
  change is checked.
- `cargo test -p sylex_english` and `cargo clippy -p sylex_english --tests -- -D warnings`
  pass.

## Pointers

- `crates/english/grammar/english.fug`: `SingularQuantifier`, `FreeChoiceDetermined`,
  `DeterminedQuantity`, `MultiplierPredetermination`, `NumericMassPremodifier`,
  `MeasuredMass`, `VariableAmount`, `MultipliedAmount`, `MultipliedDeterminer`,
  `MultipliedNounPhrase`.
- `crates/english/judgments/README.md`.

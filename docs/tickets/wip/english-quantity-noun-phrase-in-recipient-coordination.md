---
needs: []
---
# Eternal Flame coordinates a recipient with an amount of damage

## Why

Eternal Flame prints *deals X damage to target opponent or planeswalker and half X damage,
rounded up, to you*. Its correct Readings coordinate two Complement clusters: *[X damage to
target opponent or planeswalker] and [half X damage, rounded up, to you]*. Two more Readings
take *target opponent or planeswalker and half X damage* as one coordinated Noun Phrase, the
Complement of the first *to*, which leaves *to you* a second recipient of *X damage*.

## Design

- Add `Resource` to `feature NounKind` and at the matching variant index of `feature
  ObliqueNounKind`, assign `NounKind = Resource` to measured-quantity Noun Phrase Productions
  (`MeasuredMass`, `MaximumMeasuredMass`, `ComparativeQuantityMassDetermined`,
  `DistributiveMeasure`) and `SymbolNominalPhrase` (`{R}`, `{1}`) while leaving definite property
  Noun Phrases (*the planeswalker's loyalty* on *Soul Burn* and *Drain Life*) `Ordinary`, and remove
  `NounKind` from `proximate:` on `coordination NounPhrase` so coordinates share `NounKind`.

## Done when

- *Eternal Flame* has only its cluster-coordination Readings, the other measured-quantity +
  entity/temporal coordinations (*Dragonlord Atarka*, *Fury*, *Mox Amber*, *Fire and Brimstone*,
  *Open the Omenpaths*, *Scrying Glass*) lose their spurious `NounPhrase` coordinations, and
  `{R} or one mana`, `{1} and 1 life` and *Soul Burn*/*Drain Life* keep theirs.
- The census shows no verdict change on the supported corpus, every unit whose Readings change is
  checked, and `cargo xtask english strata --check`, `cargo test -p sylex_english` and
  `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `crates/english/grammar/english.fug`: `feature NounKind`, `feature ObliqueNounKind`,
  `MeasuredMass`, `MaximumMeasuredMass`, `ComparativeQuantityMassDetermined`, `DistributiveMeasure`,
  `SymbolNominalPhrase`, `coordination NounPhrase`.
- `crates/english/judgments/targeting_projection.txt`, the *Eternal Flame* entry.

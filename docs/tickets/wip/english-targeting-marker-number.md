---
needs: []
---
# Restrict `TargetedNounPhrase` to `TargetingMarker` and pass outer determiner distribution on `MarkedDeterminer`

## Why

`det:target` carries `number: Singular` (`closed_class.ron`), since on its own the Targeting Marker
determines a singular count noun (*target creature*; `*Exile target creatures.` has no Reading).
Under *any number of*, `TargetedNounPhrase` takes `determiner:DeterminerPhrase` with
`determiner.DeterminativeDistribution := Targeting` and `determiner.Number := Singular`, assuming
only bare *target* carries `Targeting`. Because `MarkedDeterminer` (`another target`, `a target`)
inherits `Targeting` from its `TargetingMarker` head, `"Exile any number of another target
creatures."` and `"Exile any number of a target creatures."` both parse today. Meanwhile, all 7
corpus uses of `UndeterminedModifiedMarker` (*other target*) are also under *any number of*, while
`UndeterminedModifiedMarker` as a general `DeterminerPhrase` admits standalone `*Destroy other
target creature(s).`.

## Design

- Keep `number: Singular` on `det:target` in `closed_class.ron`.
- Switch `TargetedNounPhrase` to take `determiner:TargetingMarker` (`BareTargetingMarker` and
  `ModifiedTargetingMarker`), retiring `UndeterminedModifiedMarker`.
- Value `$.DeterminativeDistribution = determiner.DeterminativeDistribution` (`unshared:
  [DeterminativeDistribution]`) on `MarkedDeterminer`.
- Update `TargetedQuantity`'s comment so it states *target*'s singular default and its exception
  under *number of* without attributing oblique-internal agreement waiver to CGEL number
  transparency.

## Done when

- `"Exile any number of another target creatures."`, `"Exile any number of a target creatures."`
  and `"Destroy other target creatures."` have no Reading, while every *any number of target …*
  (151 units) and *any number of other target …* (7 units) card keeps its Reading.
- `UndeterminedModifiedMarker` is retired, `MarkedDeterminer` carries its outer determiner's
  `DeterminativeDistribution`, and `TargetedQuantity`'s comment states the boundary accurately.
- The census shows no Reading lost on the supported corpus, and `cargo xtask english strata
  --check`, `cargo test -p sylex_english` and `cargo clippy -p sylex_english --tests -- -D warnings`
  pass.

## Pointers

- `crates/english/grammar/english.fug`: `TargetedQuantity`, `TargetedOblique`,
  `TargetedNounPhrase`, `TargetingMarker`, `BareTargetingMarker`, `ModifiedTargetingMarker`,
  `UndeterminedModifiedMarker`, `MarkedDeterminer`, `MarkedMaximum`.
- `crates/vocabulary/lexicon/english/closed_class.ron` (*target*).

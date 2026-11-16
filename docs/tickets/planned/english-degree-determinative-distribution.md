---
needs: []
---

# Fold QuantityKind = Degree into DeterminativeDistribution

## Why

`feature QuantityKind = Degree` is a single-valued feature carried on `DeterminerPhrase`,
`NounPhrase`, `FusedNounPhrase` and `PartitiveHead` only to mark `DegreeDeterminer` (*that many*,
*that much*) and `EquativeDeterminer` (*as many*, *as much*) for `MultipliedDegree` and the
multiplier exclusions on Noun Phrases. Every one of those four Categories already carries
`DeterminativeDistribution`, where `DegreeDeterminer` and `EquativeDeterminer` inherit
`Dependent` from `DegreeQuantifier`. Carrying a parallel feature widens the grammatical summary of
all four Categories for one determinative class.

## Design

- Add `Degree` to `DeterminativeDistribution` and at the matching variant index of
  `ObliqueDistribution` (for `valence DeterminerPhrase`'s `agree` mapping), value
  `DegreeDeterminer` and `EquativeDeterminer` as `DeterminativeDistribution = Degree`
  (`unshared: [DeterminativeDistribution]`), update `FusedDegreeDeterminer`, `DegreePartitive`,
  `TemporalLocation` (`!=` list), `MultipliedDegree` and the Noun Phrase multiplier exclusions to
  read `DeterminativeDistribution`, and delete `feature QuantityKind`.

## Done when

- `feature QuantityKind` is deleted from `crates/english/grammar/english.fug`, and
  `DeterminerPhrase`, `NounPhrase`, `FusedNounPhrase` and `PartitiveHead` no longer carry it.
- The Census shows identical Readings and verdicts across the supported corpus, and
  `crates/english/judgments/multiplier_predeterminer.txt` and
  `crates/english/judgments/equative_multiplier.txt` pass.
- `cargo test -p sylex_english` and `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `crates/english/grammar/english.fug`: `feature QuantityKind`,
  `feature DeterminativeDistribution`, `DegreeDeterminer`, `EquativeDeterminer`,
  `FusedDegreeDeterminer`, `DegreePartitive`, `MultipliedDegree`, `Determined`,
  `AdverbMultipliedNounPhrase`, `MultipliedNounPhrase`, `MultiplierPredetermination`.

---
needs: [english-degree-determinative-distribution]
---

# Canonicalize arithmetic over fused degree determiners onto `DegreeArithmetic`

## Why

A degree determiner (`that many`) takes a `plus`/`minus` modifier both inside `DeterminerPhrase`
before `FusedDegreeDeterminer` (`DegreeArithmetic`: `[[that many plus one]]`) and outside
`FusedDegreeDeterminer` (`NounPhraseArithmetic`: `[[that many] plus one]`), building two trees for
one analysis (`docs/design/english.md` §8). On 7 Corpus Units (*Izzet Generatorium*'s `"you get
that many plus one {E} instead"`, *Winding Constrictor*'s `"that many plus one of each of those
kinds of counters"`, *Brad Boimler, Eager Ensign*, *Doc Samson, Super Psychiatrist*, *Pir,
Imaginative Rascal*, *Lae'zel, Vlaakith's Champion*), `NounPhraseArithmetic` also splits the
following `{E}` or partitive oblique across `plus` (`[that many] plus [one {E}]`, `[that many] plus
[one of each …]`) beside `DegreeArithmetic`. `MultiplierPredetermination` already excludes `Degree`
so `[twice that many]` is canonical.

## Design

- Exclude bare fused `Degree` Noun Phrases (`!(head.Determination = Bare &
  head.DeterminativeDistribution = Degree)`) from `NounPhraseArithmetic` in
  `crates/english/grammar/english.fug`, keeping overtly determined delayed arithmetic (`that many
  cards plus one`, `head.Determination = Overt`).

## Done when

- `"Draw cards equal to that many plus one."`, *Izzet Generatorium* and *Winding Constrictor* build
  their degree arithmetic through `DegreeArithmetic` with no `NounPhraseArithmetic` duplicate over
  fused `that many`, while `"draw that many cards plus one"` (*Colossus of the Blood Age*) keeps its
  `NounPhraseArithmetic` Reading; judgments pin both.
- Before/after census shows zero lost faces on the supported corpus, and `cargo xtask english
  strata --check`, `cargo test -p sylex_english` and `cargo clippy -p sylex_english --tests -- -D
  warnings` pass.

## Pointers

- `crates/english/grammar/english.fug`: `DegreeDeterminer`, `FusedDegreeDeterminer`,
  `DegreeArithmetic`, `NounPhraseArithmetic`, `MultiplierPredetermination`.

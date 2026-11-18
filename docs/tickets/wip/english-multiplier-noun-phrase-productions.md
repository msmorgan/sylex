---
needs: [english-degree-determinative-distribution]
---
# The Noun Phrase multiplier Productions restate `Multiplier`

## Why

`Multiplier` states the three multiplier forms once (*half*, *twice*, *three times*) and
`MultipliedNumber` and `MultipliedDegree` take it. The three Productions that put a multiplier
before a whole Noun Phrase (*twice its power*, *half their life*, *three times this creature's
power*) predate it: `AdverbMultipliedNounPhrase`, `MultiplierPredetermination` and
`MultipliedNounPhrase` each restate one form, each repeats the exclusion of a Numerical or
Degree head, and the serial-noun form (*three times*) is written out again in
`MultipliedAmount`. They differ in one constraint: `AdverbMultipliedNounPhrase` does not
require an overtly determined head, and the other two do.

Separately, a multiplier before an object also reads as a frequency Adjunct of the verb: *it
produces twice [as much of that mana]* (Mana Reflection) and *enters with three times that
many* each keep such a Reading beside the correct one.

## Design

- Fold `DeterminativeDistribution = Multiplier` (*half*, *double*, *triple*) into `Multiplier`,
  replace `MultiplierPredetermination`, `AdverbMultipliedNounPhrase` and `MultipliedNounPhrase` with
  one `MultipliedNounPhrase: NounPhrase -> multiplier:Multiplier NounPhrase` requiring
  `head.Determination = Overt`, `head.Case = Plain` and
  `head.DeterminativeDistribution != Numerical | Degree`, license `multiplier:Multiplier Symbols`
  projecting to `FusedNounPhrase` (`NounPhrase`) for `"pays twice {X}"` (*Thassa's Intervention*),
  and delete unused `MultipliedAmount`.
- Exclude `Multiplying` (`head.AdjunctPosition != Multiplying`) from `InterleavedAdverb` and
  `Serial` (`head.NounDistribution != Serial`) from `InterleavedTemporal`, so heavy-NP shift never
  reads pre-object *twice* or *three times* as an interleaved frequency Adjunct.

## Done when

- One Production places a `Multiplier` before a `NounPhrase` (`Determination = Overt`, plus
  `multiplier:Multiplier Symbols` for *Thassa's Intervention*), `MultipliedAmount` is deleted, and
  the serial-noun form is declared once.
- *Mana Reflection* has no Reading with *twice* as an Adjunct of *produces*, *Amplifire* loses its
  genitive-determiner multiplier Readings, and *Copy that spell twice.* keeps its Reading.
- The census shows no verdict change on the supported corpus, and `cargo xtask english strata
  --check`, `cargo test -p sylex_english` and `cargo clippy -p sylex_english --tests -- -D warnings`
  pass.

## Pointers

- `crates/english/grammar/english.fug`: `Multiplier`, `MultipliedNumber`, `MultipliedDegree`,
  `AdverbMultipliedNounPhrase`, `MultiplierPredetermination`, `MultipliedNounPhrase`,
  `MultipliedAmount`.
- CGEL 5§12:434 (multipliers with definite Noun Phrases).

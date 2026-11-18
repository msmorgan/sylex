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

Separately, a multiplier or frequency phrase is read on the wrong side of the Object/Adjunct
line. Before an Object it also reads as a frequency Adjunct of the verb before a shifted Object:
*it produces twice [as much of that mana]* (Mana Reflection) and *it produces three times as
much of that mana* (Nyxbloom Ancient) each keep such a Reading beside the correct one. And a
frequency Adjunct reads as a multiplied Object: *This ability triggers only [twice each turn]*
(Nadu, Winged Wisdom).

## Design

- Replace `MultiplierPredetermination`, `AdverbMultipliedNounPhrase` and `MultipliedNounPhrase`
  with one `MultipliedNounPhrase: NounPhrase -> multiplier:Multiplier NounPhrase` requiring
  `head.Determination = Overt`, `head.Case = Plain` and
  `head.DeterminativeDistribution != Numerical | Degree | Distributive` (multipliers occur only
  with definite Noun Phrases, CGEL 5§12:434), license `multiplier:Multiplier Symbols` projecting
  to `FusedNounPhrase` (`NounPhrase`) for *pays twice {X}* (Thassa's Intervention), and delete
  unused `MultipliedAmount`.
- A multiplying adverb is a frequency End adverb by a grammar Production, not a second lexicon
  position, and both its Adverb Phrases carry `AdverbDistribution = Serial`;
  `TemporalNounPhrase` carries `NounDistribution`. `InterleavedAdverb` excludes
  `AdverbDistribution = Serial` and `InterleavedTemporal` excludes `NounDistribution = Serial`,
  so heavy-NP shift never reads pre-object *twice* or *three times* as an interleaved frequency
  Adjunct.

## Done when

- One Production places a `Multiplier` before a `NounPhrase` (`Determination = Overt`, plus
  `multiplier:Multiplier Symbols` for *Thassa's Intervention*), `MultipliedAmount` is deleted, and
  the serial-noun form is declared once.
- *Mana Reflection* has no Reading with *twice* as an Adjunct of *produces*, *Amplifire* loses its
  genitive-determiner multiplier Readings, *Nadu, Winged Wisdom* has no Reading with *twice each
  turn* as an Object, and *Copy that spell twice.* keeps its Reading.
- The census shows no verdict change on the supported corpus, and `cargo xtask english strata
  --check`, `cargo test -p sylex_english`, `cargo test -p xtask` and
  `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `crates/english/grammar/english.fug`: `Multiplier`, `MultipliedNumber`, `MultipliedDegree`,
  `AdverbMultipliedNounPhrase`, `MultiplierPredetermination`, `MultipliedNounPhrase`,
  `MultipliedAmount`, `InterleavedAdverb`, `InterleavedTemporal`.
- CGEL 5§12:434 (multipliers with definite Noun Phrases).

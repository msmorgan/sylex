---
needs: []
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

## Open questions

- Does any real card print *twice* before a bare Noun Phrase, so that the missing
  `Determination = Overt` on `AdverbMultipliedNounPhrase` is needed?
- What tells the frequency *twice* (*copy that spell twice*) from the multiplier before an
  Object: position before the Object alone, or the degree determinative that follows?

## Done when

- One Production places a `Multiplier` before a Noun Phrase, and the serial-noun form is
  declared once.
- Mana Reflection has no Reading with *twice* as an Adjunct of *produces*, and *Copy that spell
  twice.* keeps its Reading.
- The census shows no verdict change on the supported corpus.

## Pointers

- `crates/english/grammar/english.fug`: `Multiplier`, `MultipliedNumber`, `MultipliedDegree`,
  `AdverbMultipliedNounPhrase`, `MultiplierPredetermination`, `MultipliedNounPhrase`,
  `MultipliedAmount`.
- CGEL 5§12:434 (multipliers with definite Noun Phrases).

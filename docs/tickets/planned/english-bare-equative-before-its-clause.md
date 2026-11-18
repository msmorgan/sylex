---
needs: []
---

# A bare equative does not leave its own comparative clause to attach elsewhere

## Why

An equative determinative stands without its comparative clause where the comparison is
recoverable (*it produces three times as much of that mana instead*, Nyxbloom Ancient), so
`EquativeDeterminer` takes no Complement. When the *as* phrase that is its comparative clause
follows, that bare equative lets the *as* phrase attach elsewhere, as an Adjunct of a Verb
Phrase or a Modifier of a Nominal, analyses no reader has:

- *repeat this process [as many times] [as they choose]* (Trade Secrets): a manner Adjunct of
  *repeat* or *may*, in six of the unit's eight Derivations;
- *draws as many [cards [as they discarded this way]]* (Forget): a temporal Modifier of *cards*;
- *controls at least as many lands [as you do]* (Land Equilibrium): a temporal Modifier of
  *lands* or *opponent*, or a temporal Adjunct of *controls*.

## Done when

- Where an equative's comparative clause follows it, the *as* phrase is that clause's
  Complement, neither an Adjunct of a Verb Phrase nor a Modifier of a Nominal: the three units
  above keep only Readings with the comparative clause, and *three times as much of that mana*
  (Nyxbloom Ancient, Mana Reflection) keeps its bare equative.
- The census shows no verdict change, and `cargo xtask english strata --check`,
  `cargo test -p sylex_english`, `cargo test -p xtask` and
  `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Not in scope

- *As [many times]* read as predicative *as* with a Noun Phrase Complement:
  `english-predicative-as-degree-complement`.

## Pointers

- `crates/english/grammar/english.fug`: `EquativeDeterminer`, `EquativeCountDetermined`,
  `SerialTemporal`, `MannerComplementClause`.
- `crates/vocabulary/lexicon/english/prepositions.ron`: *as*.

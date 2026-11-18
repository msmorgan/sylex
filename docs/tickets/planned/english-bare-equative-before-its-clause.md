---
needs: []
---

# A bare equative does not leave its own comparative clause to attach elsewhere

## Why

An equative determinative never stands alone on a card. Eight Corpus Units print *as much* or
*as many*: *as much* is followed by an *of* phrase in each of its three (*it produces three times
as much of that mana instead*, Nyxbloom Ancient, Virtue of Strength, Mana Reflection), and
*as many* by its *as* clause in each of its five (*as many times as they choose*, Trade Secrets;
*as many times as you choose*, Lim-Dûl's Vault). `EquativeDeterminer` nevertheless takes no
Complement, so it stands complete with neither, and the *as* phrase that is its comparative
clause attaches elsewhere, as an Adjunct of a Verb Phrase or a Modifier of a Nominal, analyses no
reader has:

- *repeat this process [as many times] [as they choose]* (Trade Secrets): a manner Adjunct of
  *repeat* or *may*, in six of the unit's eight Derivations;
- *draws as many [cards [as they discarded this way]]* (Forget): a temporal Modifier of *cards*;
- *controls at least as many lands [as you do]* (Land Equilibrium): a temporal Modifier of
  *lands* or *opponent*, or a temporal Adjunct of *controls*.

## Design

- An equative determinative takes its *of* phrase or its *as* clause: with neither it has no
  Reading, and an *as* clause that follows it is its Complement.

## Done when

- Where an equative's comparative clause follows it, the *as* phrase is that clause's
  Complement, neither an Adjunct of a Verb Phrase nor a Modifier of a Nominal: the three units
  above keep only Readings with the comparative clause, and *three times as much of that mana*
  (Nyxbloom Ancient, Mana Reflection) keeps its Reading, with its *of* phrase.
- An equative determinative with neither an *of* phrase nor an *as* clause has no Reading.
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

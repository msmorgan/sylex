---
needs: []
---
# Predicative *as* takes an equative degree phrase as its Noun Phrase

## Why

The preposition *as* with a predicative Noun Phrase Complement (*as a 2/2*) also takes a Noun
Phrase that is really the equative degree determinative *as much* / *as many*, split after its
*as*. The spurious Readings, each beside the correct equative one:

- *it produces three times [as [much of that mana]]* (Nyxbloom Ancient, Virtue of Strength): a
  predicative Adjunct of *produces*, which leaves *three times* as its Object;
- *repeat this process [as [many times]] as they choose* (Trade Secrets), where *as they
  choose* is then an Adjunct of *repeat* or *may*.

The forest database has three units with such a Preposition Phrase, none correct.

## Done when

- Predicative *as* never takes a Noun Phrase whose determiner or fused head is a degree
  determinative (*much*, *many*), and the three units above keep only their equative Readings.
- The census shows no verdict change on the supported corpus, and `cargo xtask english strata
  --check`, `cargo test -p sylex_english`, `cargo test -p xtask` and
  `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `crates/vocabulary/lexicon/english/prepositions.ron`: *as* and its predicative frame.
- `crates/english/grammar/english.fug`: `EquativeDeterminer`, `DegreePartitive`.

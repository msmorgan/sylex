---
needs: []
---

# An open slot covers an unknown word only where a parse reads it

## Why

A Kind Name is proposed at the word before every spelling of *counter*, the verb's included (*X
counter target spell*), and a Set Name before every *expansion*. `AnalyzedText::unknown_words_given`
counts both as known whether or not any Reading uses them, so an unknown word at those positions
is never reported as a vocabulary gap. An Introduced Name is already known only where a parse
introduces it.

## Done when

- A word that only a Kind Name or a Set Name covers is a vocabulary gap unless some complete
  Derivation reads it as one, with a lexicon or syntax test for each slot.
- The census shows no verdict change, and `cargo test` and `cargo clippy --tests -- -D warnings`
  pass for every changed crate.

## Pointers

- `crates/lexicon/src/analysis.rs` (`unknown_words_given`), `crates/syntax/src/chart.rs`
  (`vocabulary_gaps`, `introduced_leaves`).

---
needs: [english-degree-determinative-distribution]
---
# Retire `SingularOrQuantified` onto `IndefiniteArticle`

## Why

`DeterminativeDistribution` has a 1-lexeme value `SingularOrQuantified` held by *another* alone. No
equation in `crates/english/grammar/english.fug` names `SingularOrQuantified`, *another three
creatures* has `0` Derivations despite the doc comments in `english.fug` and `schema.rs`, and zero
Vintage-legal cards print *another* before a numeral or use *another* in `TemporalLocation` (*Take
another turn* is a direct Object where `IndefiniteArticle` behaves identically).

## Design

- Delete `SingularOrQuantified` from `DeterminativeDistribution` and `ObliqueDistribution` in
  `english.fug` and from `sylex_vocabulary` (`schema.rs`, `build.rs`), giving `another` in
  `closed_class.ron` `IndefiniteArticle` alongside `a`, and remove the unbacked doc-comment claim.

## Done when

- `SingularOrQuantified` is removed from `english.fug`, `schema.rs`, `build.rs` and
  `closed_class.ron`, and the `IndefiniteArticle` comment describes what the grammar builds.
- The census shows identical Readings and verdicts on the supported corpus, and `cargo xtask
  english strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass for
  `sylex_vocabulary` and `sylex_english`.

## Pointers

- `crates/english/grammar/english.fug`: `feature DeterminativeDistribution`,
  `feature ObliqueDistribution`, `TemporalLocation`.
- `crates/vocabulary/src/schema.rs`, `crates/vocabulary/src/build.rs`,
  `crates/vocabulary/lexicon/english/closed_class.ron` (*another*).

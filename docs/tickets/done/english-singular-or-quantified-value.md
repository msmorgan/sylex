---
needs: [english-degree-determinative-distribution]
---
# `SingularOrQuantified` claims a quantified plural the grammar does not build

## Why

`DeterminativeDistribution` has a 1-lexeme value `SingularOrQuantified`, held by *another* alone.
Its name and its doc comments in `english.fug` and `schema.rs` say *another* also takes a bare
quantified plural (*another three bodies*; CGEL 5§3.4:353), but no Production builds that since
`SingularOrQuantifiedDetermination` was deleted (a modified one, *another additional two tokens*,
is `DeterminedModifiedQuantity`'s, as it is for *a*): *Create another two 1/1 white Soldier creature
tokens* has `0` Derivations, and no Vintage-legal card prints *another* before a numeral.

The value itself is load-bearing. `TemporalLocation` refuses `IndefiniteArticle` (*begin [an extra
turn]*, Gerrard's Hourglass Pendant) and takes *another*, as English does (*We can do that another
day*; CGEL 8§6.3:698), and the owner ruled both values stay as single holders for that reason
(`docs/tickets/done/english-v4-another-class-unification.md`, round 11). Folding *another* into
`IndefiniteArticle` takes *You may cast this spell another turn* from `2` Derivations to `0`.

## Design

- Rename the value after what it holds, CGEL's alternative-additive determinative *another*
  (5§7.9:391): `AlternativeAdditive`, in `english.fug`, `schema.rs` and `closed_class.ron`.
- Its doc comments state what the grammar builds: unlike *a*, *another* determines a temporal
  location Noun Phrase, and like *a* it reaches a quantified plural only through a modifier.

## Done when

- No `SingularOrQuantified` remains in the crates, and no comment claims the grammar builds a
  bare quantified plural after *another*.
- The census shows identical Readings and verdicts on the supported corpus, and `cargo xtask
  english strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass for
  `sylex_vocabulary`, `sylex_english` and `xtask`.

## Pointers

- `crates/english/grammar/english.fug`: `feature DeterminativeDistribution`,
  `feature ObliqueDistribution`, `TemporalLocation`, `TransparentQuantity`.
- `crates/vocabulary/src/schema.rs`, `crates/vocabulary/lexicon/english/closed_class.ron`
  (*another*).

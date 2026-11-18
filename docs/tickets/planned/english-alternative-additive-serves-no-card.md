---
needs: []
---

# `AlternativeAdditive` serves no card

## Why

`DeterminativeDistribution::AlternativeAdditive` exists to keep *another* apart from *a*, because
`TemporalLocation` admits *another* with a period noun and refuses *a*. The comment in
`crates/english/grammar/english.fug` and the doc comment in `crates/vocabulary/src/schema.rs`
justify that by CGEL's *We can do that another day* (Ch. 8 §6.3, p. 698) and *another additional
two tokens*. No Corpus Unit has a `TemporalLocation` node opening with *another*, and no unit
prints *another* before a period noun or a quantified plural (forest database, `units.text` and
`nodes` under `TemporalLocation`). Admission follows card text (`docs/design/english.md` §4), so
the value is kept for a form no card prints.

## Done when

- *Another* takes no `TemporalLocation` Reading, by removing the value and giving *another* the
  distribution whose Productions card text needs, or by a measured alternative; the choice and
  its effect on the Productions that name either value (`TemporalLocation`, `Determined`,
  `TransparentQuantity`, `FusedDegreeDeterminer`, `DegreePartitive`) are reported.
- The comments that cite *another day*, *another additional two tokens* and *another three
  bodies* are gone or name card text.
- The census shows no face lost and no verdict change; units whose Readings change are reported.

## Pointers

- `crates/english/grammar/english.fug`: `feature DeterminativeDistribution`, `TemporalLocation`,
  `Determined`, `TransparentQuantity`.
- `crates/vocabulary/src/schema.rs` (`DeterminativeDistribution`),
  `crates/vocabulary/lexicon/english/closed_class.ron` (*another*).

---
needs: []
---
# A determined Targeting Marker is taken as a Targeting Marker again

## Why

*Exile two three target creatures.* has a Reading, as do *two three four target creatures*,
*up to two three target creatures*, *two other three target creatures* and *target target
creature*. `BareTargetingMarker` takes any `DeterminerPhrase` whose
`DeterminativeDistribution` is `Targeting` as a `TargetingMarker`, and `MarkedDeterminer`,
`ModifiedTargetingMarker` and `MarkedMaximum` each build a `DeterminerPhrase` that keeps the
Marker's `Targeting` value. Their result is therefore a Targeting Marker once more, and a
further outer determiner or adjective determines it, without bound. The comment on
`BareTargetingMarker` says it is "the Targeting Marker alone (*target creature*)"; its equation
admits every phrase that ends in one. No English grammar stacks cardinal determiners, so each
of these is an ungrammatical Reading (`docs/decisions/english.md` §8).

## Done when

- `BareTargetingMarker` admits the Targeting Marker word alone, or the three Productions that
  determine or modify a Marker no longer yield one; the comment matches the equations.
- *Exile two three target creatures.*, *Exile target target creature.* and *Exile up to two
  three target creatures.* have no Reading, each pinned by a negative judgment.
- It is recorded, from the corpus, which orders of outer determiner and adjective over the
  Marker cards print (*two other target creatures*, *up to two other target creatures*), and
  each printed order keeps its Reading with one tree.
- The census shows no Reading lost on the supported corpus.

## Not in scope

- The Marker's Number under *any number of*: `english-targeting-marker-number`.

## Pointers

- `crates/english/grammar/english.fug`: `category TargetingMarker`, `BareTargetingMarker`,
  `ModifiedTargetingMarker`, `MarkedDeterminer`, `MarkedMaximum`.
- `crates/english/judgments/README.md`.

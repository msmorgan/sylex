---
needs: []
---
# Narrow `CompoundPart` exclusion on `Nominal` coordination so slash-measure nominals coordinate

## Why

`MeasureNominal` builds a slash measure as a noun under a determiner (*a 3/3*, *an additional
+2/+2*) and sets `$.CompoundPart = Base` so that `"a +2/+0, +1/+1, or +0/+2 counter"` gets only its
`WordPartCoordination` tree rather than a second `Nominal` coordination pairing `MeasureNominal`
with `SharedHeadCompound` (`+0/+2 counter`). Because both `coordination Nominal` schemas enforce
`first.CompoundPart != Base` unconditionally, every `Nominal` coordination whose non-final conjunct
is a determined size (`+NounPhrase a 2/2 or 3/3`, `+NounPhrase a white 2/2 and blue 3/3`) fails with
`0` Derivations.

## Design

- Value `SharedHeadCompound` nominals as `CompoundPart = Whole` on `BareNominal` (matching
  `WholeCompound` inside `CompoundCoordination`), put `CompoundPart` in `proximate:` on both
  `coordination Nominal` schemas, and narrow `first.CompoundPart != Base` to
  `!(first.CompoundPart = Base & last.CompoundPart = Whole)`.

## Done when

- `+NounPhrase a 2/2 or 3/3` and `+NounPhrase a white 2/2 and blue 3/3` each have 1 Reading, while
  `+NounPhrase a +2/+0, +1/+1, or +0/+2 counter` and `+NounPhrase a +1/+1 or -1/-1 counter` each
  keep 1 Reading (`WordPartCoordination`).
- The census shows no Reading lost on the supported corpus, and `cargo xtask english strata
  --check`, `cargo test -p sylex_english` and `cargo clippy -p sylex_english --tests -- -D warnings`
  pass.

## Pointers

- `crates/english/grammar/english.fug`: `MeasureNominal`, `feature CompoundPart`,
  `coordination Nominal` (both schemas), `WordPartCoordination`, `MeasureBase`, `WholeCompound`,
  `Premodifier`.

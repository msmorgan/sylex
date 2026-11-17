---
needs: []
---
# Narrow the `MeasureHead` exclusion on `Nominal` coordination so slash-measure nominals coordinate

## Why

`MeasureNominal` builds a slash measure as a noun under a determiner (*a 3/3*, *an additional
+2/+2*) and sets `$.MeasureHead = Fused`, and both `coordination Nominal` schemas refuse a
`first.MeasureHead = Fused` conjunct, so that *Frankenstein's Monster*'s "a +2/+0, +1/+1, or +0/+2
counter" gets only its coordination of the counter's kinds (`Kind`) and not also a
`Nominal` coordination pairing `MeasureNominal`s with "+0/+2 counter". The exclusion holds for
every `Nominal` coordination, so one whose non-final conjunct is a determined size
(`+NounPhrase a 2/2 or 3/3`, `+NounPhrase a white 2/2 and blue 3/3`) has `0` Derivations.

## Design

- Refuse a fused measure conjunct only beside a last conjunct headed by the noun *counter*: put
  `KindLicense` on `Nominal` and in `proximate:` on both `coordination Nominal` schemas,
  and narrow `first.MeasureHead != Fused` to `!(first.MeasureHead = Fused &
  last.KindLicense = Head)`.

## Done when

- `+NounPhrase a 2/2 or 3/3` and `+NounPhrase a white 2/2 and blue 3/3` each have 1 Reading, while
  `+NounPhrase a +2/+0, +1/+1, or +0/+2 counter` and `+NounPhrase a +1/+1 or -1/-1 counter` each
  keep 1 Reading (`Kind`).
- The census shows no Reading lost on the supported corpus, and `cargo xtask english strata
  --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass for every changed crate.

## Pointers

- `crates/english/grammar/english.fug`: `MeasureNominal`, `feature MeasureHead`,
  `feature KindLicense`, `coordination Nominal` (both schemas), `KindPremodifier`,
  `coordination Kind`.
- `docs/design/english.md` §3 (license features), §6 (admission gates).

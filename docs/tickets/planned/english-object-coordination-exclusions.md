---
needs: []
---
# Object Coordination's exclusions miss medial conjuncts, and it fills every Object slot

## Why

`coordination ObjectCoordination` is `serial: true`, and its comment says a slash measure is
never an Object beside a Noun Phrase or a quoted ability. Its four `!(first... & last...)`
exclusions test only `first` and `last`. The engine applies a coordination's joint exclusions
when it closes the coordination (`close_with` in `crates/syntax/src/compile.rs`), which appears
to check only the final pair, so a measure in a medial position (*base power and toughness 5/5,
flying, and +1/+1*) passes. Separately, `CoordinatedObject` puts `ObjectCoordination` into
`ComplementNounPhrase`, so any head that takes a Noun Phrase Complement takes an unlike Object
coordination (*destroy target creature and flying*), not only the heads whose frames select
each conjunct's kind. `MeasureObjectConjunct` serves one card (*... from among flying, first
strike, ..., and +1/+1*).

## Done when

- It is confirmed in `compile.rs` whether joint exclusions reach a serial coordination's medial
  conjuncts, and whether a three-conjunct coordination with a medial measure beside a Noun
  Phrase is admitted today.
- The exclusions hold for every pair of conjuncts, or the coordination is restructured so the
  rule its comment states holds; a test with a medial conjunct pins it.
- An unlike Object coordination is admitted only where the head's frames select the conjuncts'
  kinds, or the Production's comment states the slots it fills and why.
- The census shows no Reading lost on the supported corpus.

## Pointers

- `crates/english/grammar/english.fug`: `coordination ObjectCoordination`,
  `category ObjectConjunct`, `category ObjectCoordination`, `NominalObjectConjunct`,
  `KeywordObjectConjunct`, `MeasureObjectConjunct`, `QuotedObjectConjunct`,
  `CoordinatedObject`; contrast `AmountConjunct` and `NounPhraseConjunct`.
- `crates/syntax/src/compile.rs`: `close_with`, and the `AmountOrNP` derivation from frames.
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/gemini.md` (B4),
  `docs/evidence/english-fug-hax-check/fable.md` (F12).

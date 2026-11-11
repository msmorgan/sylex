---
needs: []
---
# The comparative Productions are a hand-written cross product

## Why

About twenty-five Productions write out comparatives cell by cell over {determiner, fused head,
adverb, frequency Noun Phrase, Amount} × {with *than*, without} × {modified, unmodified} ×
{count, mass}, the multiplication ADR §1 calls a defect. The copies have drifted:
`ModifiedComparativeDeterminer` takes any `Lex(Determinative)` as its modifier (so *each more
than two* as well as *no more than two*), while `DegreeModifiedComparativeDeterminer` takes
`Word(No)` only; a modified variant exists for the determiner (`ModifiedComparativeDeterminer`),
the adverb (`ModifiedComparativeAdverb`) and the frequency Noun Phrase
(`ModifiedComparativeFrequency`) but not for `ComparativeCountDetermined`.

`ComparativeAmount` types *less than 1* (*reduce it to less than 1*) as an `Amount` tagged
`ComplementKind = PP`, and two exclusions then keep it out where an Amount would go:
`NumericNounPhrase` (`head.ComplementKind != PP | Amount`) and `Unmodified`
(`!(head.ComplementKind = Amount & head.ObliqueKind = PP)`). `ComparativeFusedHead` already
builds a comparative bound as a fused-head Noun Phrase. A Production that reads `BareNominal`
without passing through `Unmodified` may still take the Amount (*total power less than 3*).

## Open questions

- One comparative determinative with a *than* frame and an optional degree modifier, its
  function decided by the existing determiner, fused-head and adjunct Productions (which may
  need the engine to take a mother's Number from a Complement), or keep the Productions and
  repair the drift cell by cell?
- Should *less than 1* after *to* be a fused-head Noun Phrase rather than an `Amount`?

## Done when

- Whether `ModifiedComparativeDeterminer` admits a non-degree modifier (*each more than two*),
  and whether a `BareNominal` consumer outside `Unmodified` takes `ComparativeAmount`'s
  Amount, are confirmed by parse.
- The comparatives follow the owner's ruling; any engine capability the single analysis needs
  is ticketed.
- `NumericNounPhrase`'s and `Unmodified`'s exclusions are gone or justified by an ablation.
- The census shows no Reading lost on the supported corpus.

## Not in scope

- The `=` on `Grade` in the comparative Productions: `english-require-on-unvalued-features`.

## Pointers

- `crates/english/grammar/english.fug`: `ComparativeFrequency`, `ModifiedComparativeFrequency`,
  `EquativeFrequency`, `MaximumSerialTemporal`, `ComparativeBoundedAlternative`,
  `ComparativeBoundedAmount`, `ComparativeAmount`, `MeasuredComparative`,
  `MeasuredStandardComparative`, `ComparativeCountDetermined`, `ComparativeMassDetermined`,
  `ComparativeDeterminer`, `ModifiedComparativeDeterminer`,
  `DegreeModifiedComparativeDeterminer`, `ComparativeAdverb`, `ModifiedComparativeAdverb`,
  `ComparativeIndirectComplement`, `ExceptiveFusedHead`, `ComparativeFusedHead`,
  `ComparativeDeterminedStandard`, `MeasuredComparativeStandard`,
  `MeasuredComparativeDetermined`, `MeasuredComparativeDeterminer`,
  `ComparativeQuantityCountDetermined`, `ComparativeQuantityMassDetermined`,
  `NumericNounPhrase`, `Unmodified`.
- `docs/decisions/english.md` §1.
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F11),
  `docs/evidence/english-fug-hax-check/gemini.md` (A3).

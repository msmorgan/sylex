---
needs: [english-frames-held-in-grammar, english-comparative-clause-gap]
---
# Consolidate the comparative Productions onto `valence DeterminerPhrase`

## Why

Thirty Productions in `crates/english/grammar/english.fug` write out comparatives and equatives cell
by cell over `{determiner, cluster, fused head, adverb, frequency Noun Phrase, Amount}` × `{with
than, without}` × `{modified, unmodified}` × `{count, mass}` (including `ComparativeCountCluster`
and `ComparativeMassCluster`). The copies have drifted and leak two bugs:

- `ModifiedComparativeDeterminer` takes any `Lex(Determinative)` as its modifier, admitting
  `*each more than two creatures` (all 16 corpus uses are *no more than …*), while
  `DegreeModifiedComparativeDeterminer` takes `Word(No)` only.
- `ComparativeAmount` (*reduce your life total to less than 1*) is kept out of `Noun`'s `Amount`
  complement by a local patch on `Unmodified` (`!(head.ComplementKind = Amount &
  head.ObliqueKind = PP)`), so any `BareNominal` consumer outside `Unmodified` leaks it
  (`+NounPhrase the mana value less than 3's owner` parses today).

## Design

- Fold comparative determinatives (*more*, *fewer*, *less*) with their *than* frame and degree
  modifier (*no*) into `valence DeterminerPhrase`, and collapse the Count/Mass/Cluster comparative
  copies using joint `!(...)` exclusions.
- Keep `ComparativeAmount` as an `Amount` for `prep:to`'s `Amount` frame (*reduce your life total to
  less than 1*, avoiding `NounPhrase`/`AdjectivePhrase` duplication with scalar *less* after copular
  *be*) and keep `NumericNounPhrase`'s `head.ComplementKind != PP | Amount` guard, while moving the
  `ComplementKind != PP` restriction onto `valence BareNominal`'s `Amount` complement (preserving
  `prep:to`'s `Amount` frame) so `Unmodified`'s local `!(head.ComplementKind = Amount &
  head.ObliqueKind = PP)` patch goes away.

## Done when

- Comparative determinatives and clusters build through `valence DeterminerPhrase` without the
  hand-copied Count/Mass cross-product, and `*each more than two creatures` has no Reading.
- `valence BareNominal`'s `Amount` complement refuses `ComparativeAmount` directly (`*the mana value
  less than 3's owner` has no Reading while `"reduce your life total to less than 1"` keeps its
  Reading), and `Unmodified`'s `!(head.ComplementKind = Amount & head.ObliqueKind = PP)` exclusion
  is gone.
- The census shows no Reading lost on the supported corpus, and `cargo xtask english strata
  --check`, `cargo test -p sylex_english` and `cargo clippy -p sylex_english --tests -- -D warnings`
  pass.

## Pointers

- `crates/english/grammar/english.fug`: `ComparativeFrequency`, `ModifiedComparativeFrequency`,
  `EquativeFrequency`, `ComparativeBoundedAlternative`, `ComparativeBoundedAmount`,
  `ComparativeAmount`, `MeasuredComparative`, `MeasuredStandardComparative`,
  `ComparativeCountDetermined`, `ComparativeMassDetermined`, `ComparativeCountCluster`,
  `ComparativeMassCluster`, `ComparativeDeterminer`, `ModifiedComparativeDeterminer`,
  `DegreeModifiedComparativeDeterminer`, `ComparativeAdverb`, `ModifiedComparativeAdverb`,
  `ComparativeIndirectComplement`, `ExceptiveFusedHead`, `ComparativeFusedHead`,
  `ComparativeDeterminedStandard`, `MeasuredComparativeStandard`, `MeasuredComparativeDetermined`,
  `MeasuredComparativeDeterminer`, `ComparativeQuantityCountDetermined`,
  `ComparativeQuantityMassDetermined`, `NumericNounPhrase`, `Unmodified`, `valence BareNominal`.
- `docs/design/english.md` §1.

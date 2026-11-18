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
  head.ObliqueKind = PP)`, repeated on `ExpandedBareMass` and `UnmodifiedBareMass`), so any
  `BareNominal` consumer outside them leaks it: `+NounPhrase the mana value less than 3 chosen`
  has one Derivation, `SingleParticipialPostmodifier` over *mana value* with *less than 3* its
  `Amount` Complement. (*The mana value less than 3's owner* is no instance: its one Derivation
  is *the mana value [less than [3's owner]]*, the bare numeral *3* a possessor inside
  `PostpositiveAdjective`'s *than* Complement, with no `ComparativeAmount`.)

## Design

- Fold comparative determinatives (*more*, *fewer*, *less*) with their *than* frame and degree
  modifier (*no*) into `valence DeterminerPhrase`, and collapse the Count/Mass/Cluster comparative
  copies using joint `!(...)` exclusions where they differ only in agreement. Count and mass
  Productions that value a feature differently are distinct constructions: a Cluster's Number is
  the standard's for count (*more than one creature*) and Singular for mass (*less than three
  mana*; CGEL 13§4.1.2:1126–1127), and a mass Noun Phrase is `Resource` (a measured mass, as
  `MeasuredMass`'s is) where a count one keeps its head's NounKind. A joint exclusion refuses a
  combination and values nothing, so it cannot merge them. A Production whose *than* Complement stands apart from the determinative
  (*more lands than you*) or is absent (*no more counters*) is no valence phrase.
- Keep `ComparativeAmount` as an `Amount` for `prep:to`'s `Amount` frame (*reduce your life total to
  less than 1*, avoiding `NounPhrase`/`AdjectivePhrase` duplication with scalar *less* after copular
  *be*) and keep `NumericNounPhrase`'s `head.ComplementKind != PP | Amount` guard, while moving the
  `ComplementKind != PP` restriction onto `valence BareNominal`'s `Amount` complement (preserving
  `prep:to`'s `Amount` frame) so `Unmodified`'s local `!(head.ComplementKind = Amount &
  head.ObliqueKind = PP)` patch goes away.

## Done when

- Comparative determinatives and clusters build through `valence DeterminerPhrase` without the
  hand-copied Count/Mass cross-product (copies that differ only in agreement are one Production),
  and `*each more than two creatures` has no Reading.
- `valence BareNominal`'s `Amount` complement refuses `ComparativeAmount` directly (`+NounPhrase
  the mana value less than 3 chosen` has no `ComparativeAmount` Derivation while `"reduce your life
  total to less than 1"` keeps its Reading), and `Unmodified`'s `!(head.ComplementKind = Amount &
  head.ObliqueKind = PP)` exclusion is gone.
- The census shows no Reading lost on the supported corpus, and `cargo xtask english strata
  --check`, `cargo test -p sylex_english` and `cargo clippy -p sylex_english --tests -- -D warnings`
  pass.

## Pointers

- `crates/english/grammar/english.fug`: `ComparedFrequency`, `ModifiedComparativeFrequency`,
  `ComparativeBoundedAlternative`, `ComparativeBoundedAmount`,
  `ComparativeAmount`, `MeasuredComparative`, `MeasuredStandardComparative`,
  `ComparativeDetermined`, `ComparativeCountCluster`,
  `ComparativeMassCluster`, the `Comparative` variant of `valence DeterminerPhrase`,
  `ModifiedComparativeDeterminer`,
  `DegreeModifiedComparativeDeterminer`, `ComparativeAdverb`, `ModifiedComparativeAdverb`,
  `ComparativeIndirectComplement`, `ExceptiveFusedHead`, `ComparativeFusedHead`,
  `ComparativeDeterminedStandard`, `MeasuredComparativeStandard`, `MeasuredComparativeDetermined`,
  `MeasuredComparativeDeterminer`, `ComparativeQuantityCountDetermined`,
  `ComparativeQuantityMassDetermined`, `NumericNounPhrase`, `Unmodified`, `ExpandedBareMass`,
  `UnmodifiedBareMass`, `valence BareNominal`.
- `docs/design/english.md` §1.

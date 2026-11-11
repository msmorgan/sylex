---
needs: []
---
# Fused *each* is banned from Complements, and three declarations work around the ban

## Why

`NonNumericComplement` excludes a distributive fused quantity from every Noun Phrase
Complement, so that an *of* phrase after fused *each* is always its partitive oblique. The ban
is wider than that reason, which is same-head complement priority (ADR §3) for one following
*of* phrase. CGEL Ch. 5 §9.2, p. 413 puts *each* with *either* and *neither* as determinatives
that fuse in an implicit partitive, with an Object among its examples; `FusedChoice` lets
*either* and *neither* be Complements. Three declarations then work around the ban:

- `NominalPredicative` repeats the exclusion for Predicatives.
- `QuantityPartitiveOfEach` builds *create one of each* as a flat `FusedNounPhrase`, whose
  comment explains it by the ban. It sets `$.DeterminativeDistribution = Distributive` over a
  `Numerical` head, apparently so that `DegreeArithmetic` can refuse it.
- `DegreeArithmetic` excludes a distributive operand (*that many plus one of each of those
  kinds*).

The likely cause is that the partitive is no frame: *each* has none in the lexicon, and
`Partitive` and its siblings are Productions over the hand-listed `class PartitiveQuantifier`,
so the engine's complement priority cannot apply to *each of ...*.

## Open questions

- Give the partitive determinatives an *of* frame in the lexicon and let complement priority
  decide, or give the fused-quantifier Productions an engine-level priority over a following
  *of* phrase?

## Done when

- It is confirmed, by ablating the `NonNumericComplement` exclusion, which spurious Readings it
  prevents on real cards and whether they are all *of*-phrase attachments to fused *each*. If
  the exclusion prevents something complement priority cannot, the ticket closes with the
  comments of the four declarations saying so instead.
- Fused *each* is admitted as a Complement and a Predicative, and an *of* phrase right after
  it is its partitive oblique only.
- `QuantityPartitiveOfEach` and the `DegreeArithmetic` and `NominalPredicative` exclusions are
  deleted, or kept with an ablation that justifies each.
- No Production sets a `DeterminativeDistribution` its head does not have.
- *Create one of each* and *that many plus one of each of those kinds* each keep a correct
  Reading, and the census shows no Reading lost on the supported corpus.

## Not in scope

- `SynesisDistributivePartitive`: `english-synesis-each-partitive`.

## Pointers

- `crates/english/grammar/english.fug`: `NonNumericComplement`, `NominalPredicative`,
  `QuantityPartitiveOfEach`, `DegreeArithmetic`, `class FusedQuantifier`, `class FusedChoice`,
  `class PartitiveQuantifier`, `Partitive`, `DistributivePartitive`, `high_attachment`.
- `crates/vocabulary/lexicon/english/closed_class.ron`.
- `docs/decisions/english.md` §3 (same-head complement priority).
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F3),
  `docs/evidence/english-fug-hax-check/gemini.md` (A2).

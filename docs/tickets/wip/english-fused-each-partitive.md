---
needs: []
---
# Fused *each* is banned from Complements, and two declarations work around the ban

## Why

`NonNumericComplement` excludes a distributive fused quantity from every Noun Phrase
Complement, so that an *of* phrase after fused *each* is always its partitive oblique. That
reason is same-head complement priority (ADR §3) for one following *of* phrase; the ban is wider,
because it also refuses fused *each* before a Nominal it could determine. CGEL Ch. 5 §9.2, p. 413
would put *each* with *either* and *neither* as determinatives that fuse only in partitives but
for its fixed uses, and gives *either* as an Object; `FusedChoice` lets *either* and *neither* be
Complements. Two declarations then work around the ban:

- `QuantityPartitiveOfEach` builds *create one of each* as a flat `FusedNounPhrase`, because
  fused *each* is no Complement. It sets `$.DeterminativeDistribution = Distributive` over a
  `Numerical` head so that `DegreeArithmetic` can refuse it.
- `DegreeArithmetic` excludes a distributive operand (*that many plus one of each of those
  kinds*).

Separately, `NominalPredicative` excludes fused *each* as a Predicative, which guards *are
[each]* with *equal to ...* a depictive.

The cause is that the partitive is no frame: *each* has none in the lexicon, and `Partitive` and
its siblings are Productions over the hand-listed `class PartitiveQuantifier`, so the engine's
complement priority cannot apply to *each of ...*.

## Design

- The Complement ban, `QuantityPartitiveOfEach` and the `DegreeArithmetic` and
  `NominalPredicative` exclusions stay; their comments say what each prevents.
- Giving the partitive determinatives an *of* frame belongs to `english-frames-held-in-grammar`.

## Done when

- The comments of `NonNumericComplement`, `QuantityPartitiveOfEach`, `DegreeArithmetic` and
  `NominalPredicative` each name a card and the structure their restriction prevents, found by
  ablating the restriction.
- No comment says fused *each* is a Subject only, or sets *each* against *either* and *neither*
  by a semantic gloss.
- The census shows no change.

## Not in scope

- `SynesisDistributivePartitive`: `english-synesis-each-partitive`.
- An *of* frame on the partitive determinatives and complement priority for the fused head:
  `english-frames-held-in-grammar`.

## Pointers

- `crates/english/grammar/english.fug`: `NonNumericComplement`, `NominalPredicative`,
  `QuantityPartitiveOfEach`, `DegreeArithmetic`, `class FusedQuantifier`, `class FusedChoice`,
  `class PartitiveQuantifier`, `Partitive`, `DistributivePartitive`, `high_attachment`.
- `crates/vocabulary/lexicon/english/closed_class.ron`.
- `docs/decisions/english.md` §3 (same-head complement priority).
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F3),
  `docs/evidence/english-fug-hax-check/gemini.md` (A2).

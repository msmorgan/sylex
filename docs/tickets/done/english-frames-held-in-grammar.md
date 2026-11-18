---
needs: [english-preposition-frame-constraints]
---
# Complement selection written as grammar classes instead of frames

## Why

english.md §3 says a frame is declared once, in the lexicon. Three selections are held in the
grammar instead, as a hand-listed class or word list with Productions over it:

- `class PartitiveQuantifier`, with `Partitive`, `DistributivePartitive` and their siblings: the
  determinatives that take an *of* oblique.
- `class ComparativeQuantifier`, with the comparative Productions: the determinatives that take
  a *than* Complement.
- The preposition list `In, On, Among, Under, Outside, After` in `LocatedExistentialBe`, which
  restates *be*'s prepositional frame.

They appear to be in the grammar because a frame cannot do two things these need: build a fused
determiner-head, and give the mother its Number from the Complement (`ComparativeDeterminer`
names the comparative `quantifier`, not `head`, for that reason). The cost is that what english.md
promises for frames does not reach them: same-head complement priority cannot apply to *each of
...*, and nothing keeps the lists and the lexicon alike.

The half of same-head complement priority by which a bare head with its Complement omitted takes
the following Preposition Phrase its frame selects is limited to a non-verbal `valence` Schema's
`Bare` Production, whose leaf records its lexeme's frames; a hand-written Production over a
`Class` leaf, such as `FusedQuantifierNominalPhrase`, gets none even when the word has a frame.
An *of* frame on *each* alone also makes `ExceptiveFusedHead`, `ComparativeFusedHead` and
`ComplementedPartitive` accept *each of X*, since nothing carries the oblique's preposition.
With priority on fused *each*, the *of*-phrase half of the `NonNumericComplement` ban becomes
redundant, but the ban stays until fused *each* before a word it could determine (*pays {X} for
each attacking creature they control*, War Tax) has another refusal:
`english-fused-determinative-before-nominal`. While it stays, *of each* has no Complement, so
`QuantityPartitiveOfEach` stays as the ban's one exception (*create one of each*, Academy
Manufactor), without the `Distributive` value it set over its `Numerical` head. The
`DegreeArithmetic` exclusion becomes removable; `NominalPredicative`'s does not (fused *each* as
the Predicative of *power and toughness are each equal to ...*).

## Design

- Extend `valence` in `sylex_syntax` to derive fused determiner-head projections, complement-to-mother
  `agree` mappings with `unshared` features, and existential-*be* preposition selection from lexicon
  frames.
- Move `PartitiveQuantifier` (*of* oblique), `ComparativeQuantifier` (*than* complement) and
  existential-*be*'s preposition list to lexicon frames in `closed_class.ron`,
  deleting the grammar classes and hand-listed prepositions along with `DegreeArithmetic`'s
  exclusion.

## Done when

- `PartitiveQuantifier`, `ComparativeQuantifier`, existential-*be*'s hand-listed prepositions
  and `DegreeArithmetic`'s exclusion are retired in favor of lexicon frames: compiled through
  `valence` where the Complement follows its head, and read by the Production where it places
  or omits the Complement itself.
- No Production gives its phrase a `DeterminativeDistribution` other than the one its head
  carries; a head that carries none (a numeral, a genitive pronoun) takes the phrase's from its
  Production.
- The census shows no Reading lost on the supported corpus, and `cargo xtask english strata
  --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_syntax`,
  `sylex_vocabulary` and `sylex_english`.

## Not in scope

- Collapsing the comparative Productions: `english-comparative-productions`.
- Retiring the `NonNumericComplement` ban and `QuantityPartitiveOfEach`:
  `english-fused-determinative-before-nominal`.

## Pointers

- `crates/english/grammar/english.fug`: `class PartitiveQuantifier`,
  `class ComparativeQuantifier`, `Partitive`, `ComparativeDeterminer`, `LocatedExistentialBe`,
  `valence DeterminerPhrase`.
- `crates/vocabulary/lexicon/english/closed_class.ron`, `crates/syntax/src/compile.rs`.
- `docs/design/english.md` §1, §3.

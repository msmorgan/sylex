---
needs: []
---

# A comparative clause has its gap

## Why

A comparative clause, the Complement of *than* or equative *as*, must leave empty the
position its comparative governor corresponds to (CGEL, Ch. 13 §2.1, p. 1108, [4]: *as deep as
[it is — wide]*; *more creatures than [you do —]*). The grammar takes any finite Clause there,
so *than it resolves* and *as it resolves* parse as Preposition Phrases with a comparative
clause, though neither clause has a gap. *As* has a temporal clause frame and a comparative
clause frame, so the Preposition Phrase *as it resolves* (*Lilah, Undefeated Slickshot*) has a
Reading for each, and `as_clause_np_licensing.txt` cannot state that it has one. And a
comparative clause after a modified equative quantity has no comparative analysis at all: in
*an opponent who controls at least as many lands as you do* (*Land Equilibrium*), *as you do*
reads only as a temporal clause.

## Design

- Add a `StrandedDo` `VerbPhrase` Production over `["verb:do", "verb:don't"]` (removing
  `Intransitive` from `verb:do` in `verbs.ron` and `verb:don't` in `closed_class.ron`, while keeping
  `Intransitive` on `verb:do so`, CGEL, Ch. 17, §7.5, p. 1531), and mark `StrandedModal`,
  `StrandedBe`, `StrandedPerfect` and `StrandedDo` with an ellipsis feature (`Code`) while keeping
  `Gap = None` so relative clauses (`Gap = NP`) never admit them.
- Set `fused_quality: true` on `odd` and `even` in `adjectives.ron`, and license
  `QualityHead := Fused` adjectives in `Mention` so `"Choose odd or even."` takes a direct Object.
  `verb:choose` keeps `Intransitive` here: *divided as you choose* reads only through it until
  `english-manner-as-clause` gives the manner clause its frame and removes it.
- Replace `Clause(Finite)` on `prep:than` and equative `prep:as` with `ComparativeClause`
  (`ComplementOnly`), licensed by either `Gap = NP` (*Forget* `"as many cards as they discarded __
  this way"`, *Lim-Dûl's Vault* `"as many times as you choose __"`) or `Gap = None` with `Code`
  (*Keeper*/*Oath* `"than you do"`, *Land Equilibrium* `"as you do"`), and refactor
  `EquativeCountDetermined` and `EquativeFrequency` to take `EquativeDeterminer` including
  `BoundingAdverb` (*at least*).

## Done when

- A comparative clause Complement of *than* or *as* has its gap (or `Code` ellipsis), and a gapless
  lexical finite Clause is no comparative clause: the Preposition Phrase *as it resolves* has one
  Reading, and `as_clause_np_licensing.txt` states it (`readings 1`).
- *At least as many lands as you do* (*Land Equilibrium*) has a Reading with a comparative clause,
  and `"Choose odd or even."` (*Extinction Event*) reads with *odd or even* as Complement of
  *choose*.
- *As many times as you choose*, *as many cards as they discarded this way* (*Forget*), *the same
  name as that permanent* and *more lands than you* keep their Readings.
- `cargo xtask english strata --check` passes, and `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Pointers

- `crates/vocabulary/lexicon/english/prepositions.ron` (*as* and *than*),
  `crates/vocabulary/lexicon/english/verbs.ron` (*choose*, *do*, *do so*),
  `crates/vocabulary/lexicon/english/closed_class.ron` (*don't*),
  `crates/vocabulary/lexicon/english/adjectives.ron` (*odd*, *even*).
- `crates/english/grammar/english.fug` (`StrandedModal`, `StrandedBe`, `StrandedPerfect`,
  `EquativeCountDetermined`, `EquativeFrequency`, `Mention`, the comparative determiners).
- `crates/english/judgments/as_clause_np_licensing.txt`.

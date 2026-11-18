---
needs: [english-comparative-clause-gap]
---

# A manner *as* clause is not a temporal clause

## Why

*As* has one finite-clause frame for an Adjunct or Modifier, a `TemporalClause` (*as you cast
this spell*, *as the turn began*): the `TemporalClause` slot over `FiniteClause`, which takes a
gapless finite Clause and gives the phrase its time. *Divided as you choose* and *divided as its
controller chooses* (*Master of the Wild Hunt*) are manner clauses, yet they parse as temporal
ones over intransitive *choose*: they give their host time, and they modify a Nominal as *life
total as the turn began* does (*Sengir, the Dark Baron*). So 84 units read *as you choose*, *as
its controller chooses* or *as they choose* as a temporal clause, most of them with a Reading in
which it modifies a noun (*Fire Covenant*: *damage divided [as you choose]* under *damage*).

## Design

- Add a `Predicate`-only manner-clause frame on `prep:as`, a `MannerClause` Complement, which
  gives its host no time. Its Clause has a manner Gap (`Gap = Adjunct`, CGEL Ch. 13 §5.5,
  p. 1149, [34]), which only finite *choose* with its Complement left unexpressed introduces
  (*as you choose [to divide it]*; CGEL Ch. 17 §7.4, p. 1527). An object Gap (`Gap = NP`) is not the manner clause's: it gives
  a manner Reading to *as the turn began* over transitive *begin* (*Sengir*), *as many cards as
  they discarded this way* (*Forget*) and *as you do* over transitive *do* (*Land Equilibrium*).
  The grammar takes no elliptical manner clause (`Code`), which Oracle text does not print, so
  *Land Equilibrium*'s *as you do* gets no manner Reading. A complete
  Clause stays temporal only.
- Remove `Intransitive` from `verb:choose` in `verbs.ron`: *divided as you choose* and *reorder
  your graveyard as you choose* (*Fossil Find*) read today only as a temporal clause over
  intransitive *choose*, and with the manner frame they no longer need it.

## Done when

- *Divided as you choose* has a manner Reading and no temporal one, and no Reading in which the
  clause modifies a Nominal; a judgment pins it on a real card.
- `"Choose odd or even."` (*Extinction Event*) has one Reading, with *odd or even* the Object of
  *choose*, and *as many times as they choose* (*Trade Secrets*) has no temporal Reading of *as
  they choose*.
- *As you cast this spell* (*Jeska's Will*) and *as the turn began* (*Sengir, the Dark Baron*)
  keep their temporal Readings and gain no manner one.
- `cargo xtask english strata --check` passes, and `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Pointers

- `crates/vocabulary/lexicon/english/prepositions.ron` (*as*).
- `crates/english/grammar/english.fug` (slot `TemporalClause`, `FiniteComplementClause`,
  `dependents TimeGiving`).

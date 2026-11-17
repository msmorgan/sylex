---
needs: [english-comparative-clause-gap]
---

# A manner *as* clause is not a temporal clause

## Why

*As* has one finite-clause frame for an Adjunct or Modifier, a `TemporalClause` (*as you cast
this spell*, *as the turn began*). *Divided as you choose* and *divided as its controller
chooses* (*Master of the Wild Hunt*) are manner clauses, yet they parse as temporal ones: they
give their host time, and they modify a Nominal as *life total as the turn began* does (*Sengir,
the Dark Baron*). So 74 units with *divided as you choose* have a Reading in which the clause
modifies a noun (*Fire Covenant*: *damage divided [as you choose]* under *damage*), and a manner
clause under a past host with a present verb would be refused as a time mismatch.

## Design

- Add a `Predicate`-only, non-`TimeGiving` manner-clause frame on `prep:as` requiring `Gap = NP`
  (neither `Gap = Adjunct`, which would give complete clauses a manner Reading, nor `Code`
  `Gap = None`, which would add a manner Reading on *Land Equilibrium*'s `"controls at least as many
  lands as you do"`), while `TemporalClause` keeps `NominalAndPredicate` and `Gap = None`. Since
  `verb:choose` is transitive, `"divided as you choose"` parses only as a manner clause (`Gap = NP`)
  and complete clauses parse only as temporal.
- Remove `Intransitive` from `verb:choose` in `verbs.ron`: *divided as you choose* and *reorder
  your graveyard as you choose* (*Fossil Find*) read today only as a temporal clause over
  intransitive *choose*, and with the manner frame they no longer need it.

## Done when

- *Divided as you choose* has a manner Reading and no temporal one, and no Reading in which the
  clause modifies a Nominal; a judgment pins it on a real card.
- `"Choose odd or even."` (*Extinction Event*) has one Reading, with *odd or even* the Object of
  *choose*, and *as many times as you choose* (*Trade Secrets*) has no temporal Reading of *as you
  choose*.
- *As you cast this spell* (*Jeska's Will*) and *as the turn began* (*Sengir, the Dark Baron*)
  keep their temporal Readings and gain no manner one.
- `cargo xtask english strata --check` passes, and `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Pointers

- `crates/vocabulary/lexicon/english/prepositions.ron` (*as*).
- `crates/english/grammar/english.fug` (`TemporalClause`, `dependents TimeGiving`).

---
needs: []
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

## Open questions

- What tells the two apart in the grammar. A manner *as* clause has a gap or an elided
  Complement that its host supplies (*as you choose [to divide it]*), and a temporal one is
  complete; a second frame with no such restriction would give every temporal *as* clause a
  manner Reading that no time restricts.

## Done when

- *Divided as you choose* has a manner Reading and no temporal one, and no Reading in which the
  clause modifies a Nominal; a judgment pins it on a real card.
- *As you cast this spell* (*Jeska's Will*) and *as the turn began* (*Sengir, the Dark Baron*)
  keep their temporal Readings and gain no manner one.
- `cargo xtask english strata --check` passes, and `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Pointers

- `crates/vocabulary/lexicon/english/prepositions.ron` (*as*).
- `crates/english/grammar/english.fug` (`TemporalClause`, `dependents TimeGiving`).
- `docs/tickets/planned/english-comparative-clause-gap.md`, which gives a comparative *as*
  clause its gap.

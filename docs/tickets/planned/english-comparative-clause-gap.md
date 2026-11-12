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
*an opponent who controls at least as many lands as you do* (Land Equilibrium), *as you do*
reads only as a temporal clause.

## Done when

- A comparative clause Complement of *than* or *as* has its gap, and a gapless finite Clause is
  no comparative clause: the Preposition Phrase *as it resolves* has one Reading, and
  `as_clause_np_licensing.txt` states it (`readings 1`).
- *At least as many lands as you do* (Land Equilibrium) has a Reading with a comparative clause.
- *As many times as you choose*, *as many cards as they discarded this way* (Forget), *the same
  name as that permanent* and *more lands than you* keep their Readings.
- `cargo xtask english strata --check` passes, and `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Pointers

- `crates/vocabulary/lexicon/english/prepositions.ron` (*as* and *than*).
- `crates/english/grammar/english.fug` (`EquativeFrequency`, the comparative determiners).
- `crates/english/judgments/as_clause_np_licensing.txt`.

---
needs: []
---

# A comma-fronted Adjunct may scope over the first coordinate alone

## Why

A dependent fronted with a comma scopes over the whole of a Clause coordination that is not
delimited before its coordinator (`Fronting::Wide`), so it is never part of the first conjunct.
CGEL lets a front Adjunct apply to the first coordinate only (*Yesterday Ed was taken ill and
the lecture's been cancelled*, Ch. 15, §1.3.4, p. 1286, [33iv a]). A temporal clause agrees in
time with every conjunct of the coordination it modifies, so *When this creature entered, you
lost 1 life and you draw a card* has no Reading at all, where it should have the one in which
the clause modifies *you lost 1 life*. No unit of the pool is known to have this shape.

## Done when

- A comma-fronted Adjunct before a coordination of Clauses has the Reading in which it modifies
  the first conjunct, where the whole-coordination Reading is unavailable; a judgment derived
  from a named real card pins it.
- A fronted trigger or condition before conjuncts it applies to as a whole keeps its
  whole-coordination Reading and gains no first-conjunct one (*When this creature enters, draw
  a card and you lose 1 life*).
- `cargo xtask english strata --check` passes, and `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Pointers

- `crates/syntax/src/step.rs` (`Fronting`, `comma_scope_excludes`).
- `docs/decisions/english.md` §8.
- `crates/english/grammar/english.fug` (`dependents TimeGiving`).

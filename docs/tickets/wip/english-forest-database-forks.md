---
needs: [english-forest-database, english-forest-forks]
---

# Forks in the pool's forest database

## Why

The pool database (`english-forest-database`) stores every unit's Packed Forest with its exact
counts. A question about where analyses differ ("every fork whose options are a wide and a
narrow attachment over a coordination") needs the forks of `english-forest-forks` there too.

## Design

- Tables for forks and their options, with counts, as `ForkForest::forks` gives them
  (`english-forest-forks`): each fork's kind, leaf, aliases, context and whether conditional;
  each option's answer, its aliases' answers and its count.
- A named view of every fork whose options are a wide and a narrow attachment over a
  coordination.

## Done when

- Forks and options are written for every unit of the census corpus, and a test checks on real
  units that the stored forks equal what `ForkForest::forks` computes.
- The view is exercised by a query in the landing, with its row count.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Pointers

- `docs/tickets/planned/english-forest-database.md` (its schema and command).
- `crates/english/src/census.rs`; `crates/syntax/src/forks.rs`.

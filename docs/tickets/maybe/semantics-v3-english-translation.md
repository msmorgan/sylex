---
needs: []
---

# Recover `.syl` terms from `sylex_english` Readings

## Why

A card's meaning is a `.syl` term written by hand today. Spelling is meant to be a
construction grammar: each Template or Definition with its printed wording pairs English structure
with semantic structure, so the same grammar recovers a resolved term from a Reading and spells a
term back as English (`architecture.md`, "Spelling is a construction grammar"). Nothing maps a
`sylex_english` Reading to a `.syl` term yet, and the resolved tree it would target does not exist
yet either.

## Done when

- For a canon card, recovery reads the Packed Forest of its Oracle text and yields candidate
  resolved terms built from the builtin Templates and Definitions; candidates that discourse or
  the checker refuses drop out.
- Every canon card's recovery is run and reported: one surviving candidate that matches the
  card's `.syl` file, a mismatch, no candidate, or more than one candidate (reported, never stored
  as a choice).

## Pointers

`docs/decisions/architecture.md` ("Spelling is a construction grammar"),
`docs/decisions/english.md` §4, §8, `docs/decisions/semantics.md` §1, §3,
`docs/decisions/syl.md`, `crates/english/`, `plugins/canon/cards/`.

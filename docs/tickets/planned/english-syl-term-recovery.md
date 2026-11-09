---
needs: [syl-spelling-conventions, syl-spelling-optional-segments]
---

# Recover `.syl` terms from `sylex_english` Readings

## Why

A card's meaning is a `.syl` term written by hand today. Spelling is meant to be a
construction grammar: each Template or Definition with its printed wording pairs English structure
with semantic structure, so the same grammar recovers a resolved term from a Reading and spells a
term back as English (`architecture.md`, "Spelling is a construction grammar"). Recovery reads
each Template's and Definition's `spelling`, so it needs the spelling conventions
(`syl-spelling-conventions`) and the optional and repeated segments
(`syl-spelling-optional-segments`) that fill the 151 `TODO(spelling)` markers. Nothing maps a
`sylex_english` Reading to a `.syl` term yet, and the resolved tree it would target does not exist
yet either.

## Done when

- For a canon card, recovery reads the Packed Forest of its Oracle text and yields candidate
  resolved terms built from the builtin Templates and Definitions; candidates that discourse or
  the checker refuses drop out.
- Every canon card's recovery is run and reported: one surviving candidate that matches the
  card's `.syl` file, a mismatch, no candidate, or more than one candidate (reported, never stored
  as a choice).
- No check exists yet: recovery has no code, and the resolved tree it targets arrives with step 7.
  The landing names the command that runs recovery and reports its results.

## Pointers

`docs/decisions/architecture.md` ("Spelling is a construction grammar"),
`docs/decisions/english.md` §4, §8, `docs/decisions/semantics.md` §1, §3,
`docs/decisions/syl.md`, `crates/english/`, `plugins/canon/cards/`.

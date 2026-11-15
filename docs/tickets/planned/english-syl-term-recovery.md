---
needs: [syl-spelling-conventions, syl-spelling-optional-segments]
---

# Recover `.syl` terms from `sylex_english` Readings

## Why

A card's meaning is a `.syl` term written by hand today. Spelling is meant to be a
construction grammar: each Template or Definition with its printed wording pairs English structure
with semantic structure, so the same grammar recovers a surface term from a Reading and spells a
term back as English (`architecture.md`, "Spelling is a construction grammar"). Recovery reads
each Template's and Definition's `spelling`, so it needs the spelling conventions
(`syl-spelling-conventions`) and the optional and repeated segments
(`syl-spelling-optional-segments`) that fill the 168 `TODO(spelling)` markers. Nothing maps a
`sylex_english` Reading to a `.syl` term yet, and the Rust structures it would build do not
exist yet either.

Recovery does not wait for the rest of step 6 (`cleanup-step6-syl`). It reads whichever
declarations have a wording when it runs, so its report changes as the corpus is reshaped.

## Done when

- For a canon card, recovery reads the Packed Forest of its Oracle text and yields candidate
  surface terms built from the builtin Primitives, Templates and Definitions; candidates that
  semantics refuses drop out, and one it does not understand is reported as that
  (`docs/design/semantics.md` §7).
- Every canon card's recovery is run and reported: one surviving candidate that matches the
  card's `.syl` file, a mismatch, no candidate, or more than one candidate. A card with more than
  one is reported with each candidate; recovery chooses none, by rank or otherwise, and stores no
  choice.
- No check exists yet: recovery has no code, and the Rust structures it builds arrive with step 7.
  The landing names the command that runs recovery and reports its results.

## Not in scope

- Choosing among the candidates the checker and discourse leave: the pragmatics step
  (`docs/design/pragmatics.md`).

## Pointers

`docs/design/architecture.md` ("Spelling is a construction grammar"),
`docs/design/english.md` §4, §8, `docs/design/semantics.md` §1, §3,
`docs/design/syl.md`, `crates/english/`, `plugins/canon/cards/`.

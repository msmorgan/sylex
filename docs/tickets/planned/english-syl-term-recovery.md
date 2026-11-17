---
needs: [cleanup-step6-surface, syl-shared-phrase-written-once, syl-replacement-stated-as-clause, syl-token-records-to-templates]
---

# Recover `.syl` terms from `sylex_english` Readings

## Why

A card's `.syl` term is written by hand today. Spelling is meant to be a
construction grammar: each Template or Definition with its printed wording pairs English structure
with semantic structure, so the same grammar recovers a surface term from a Reading and spells a
term back as English (`architecture.md`, "Spelling is a construction grammar"). Recovery reads
each Template's and Definition's `spelling`, so it needs the spelling conventions
(`syl-spelling-conventions`) and the optional and repeated segments
(`syl-spelling-optional-segments`) that fill the 168 `TODO(spelling)` markers. Nothing maps a
`sylex_english` Reading to a `.syl` term yet, and the Rust structures it would build do not
exist yet either.

Recovery is written against the surface (`cleanup-step6-surface`), which has the nominal
descriptions, the spellings in the settled notation and the clause and voice words, and against
the three tickets after it that write shared phrases, clause-form replacements and tokens as
printed. It does not wait for the rest of step 6 (`cleanup-step6-syl`).

## Done when

- For a canon card, recovery reads the Packed Forest of its Oracle text and yields candidate
  surface terms built from the builtin Primitives, Templates and Definitions; a candidate
  `cargo xtask dsl check` refuses drops out.
- Every canon card's recovery is run and reported: one surviving candidate that matches the
  card's `.syl` file, a mismatch, no candidate, or more than one candidate. A card with more than
  one is reported with each candidate; recovery chooses none, by rank or otherwise, and stores no
  choice.
- No check exists yet: recovery has no code, and the Rust structures it builds arrive with step 7.
  The landing names the command that runs recovery and reports its results.

## Not in scope

- Pruning candidates by what they mean, which waits for the semantics crate
  (`docs/design/semantics.md` §5), and choosing among the candidates left: the pragmatics step
  (`docs/design/pragmatics.md`).

## Pointers

`docs/design/architecture.md` ("Spelling is a construction grammar"),
`docs/design/english.md` §4, §8, `docs/design/semantics.md` §1, §5, `docs/design/surface.md` §6,
`docs/design/syl.md`, `crates/english/`, `plugins/canon/cards/`.

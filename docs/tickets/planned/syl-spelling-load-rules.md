---
needs: [syl-spelling-optional-segments]
---

# `cargo xtask dsl check` refuses a `spelling` that breaks a load rule its declaration settles

## Why

`docs/design/spelling.md` §7 refuses a `spelling` at load for more than bad syntax, and nothing
makes those refusals. `syl-spelling-notation-crate` parses every `spelling`, so a wording that
parses and still cannot round-trip is accepted: a parameter that no Hole prints, two optional
parts one printed phrase could fill, a list with no join.

## Design

- The rules checked are the ones that need only the declaration and its parameters' sorts:
  1. every Hole names a parameter of its declaration;
  2. a Hole states a join exactly when its parameter is a `List`;
  3. every parameter with a Hole directly in an optional part is omissible;
  4. all the Holes of one parameter stand in one optional part, or all stand outside every one;
  5. an optional part directly holds a Hole, a Hole in its own Tail counting;
  6. outside every optional part there is text, a Hole, a Tail or a Gap;
  7. a parameter with no Hole is omissible;
  8. the skeleton is unambiguous;
  9. a `spelling` that is one list Hole whose element can stand at the result sort spells lists
     of two or more only.
- The rules that need the lexicon (a tie's affix is a bound form it declares) or the English
  grammar (every variant is English) are not checked here.

## Done when

- `cargo xtask dsl check` refuses a `spelling` that breaks any of the nine rules, with a
  negative test for each, and every declaration under `plugins/` passes.
- Each declaration that breaks a rule when this ticket is claimed is resolved against its
  card's Oracle text or its rule's text, by writing the Hole or by fixing the parameter; the
  landing lists each with the rule it broke and what was done. Four break rule 7 today
  (`colored`, `ofTheLastNotedType`, `faceDownAs`, `doUnlessKeywordCost`; re-run the scan when
  claimed).
- `docs/design/spelling.md` §7 states rule 1 among its refusals, says a list parameter with no
  Hole is fixed at the empty list, where it says an empty list is spelled only as the absent
  value of an optional part, and says how a Tail and a Gap enter the skeleton.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask` and clippy pass.

## Not in scope

- The form-label rules ("a label is refused where the position admits only one form"):
  `syl-spelling-forms-and-runs`.
- That every variant of a `spelling` is English: it waits on spellings being parsed through the
  English grammar (`english-syl-term-recovery`).

## Pointers

- `docs/design/spelling.md` §7 ("A `spelling` is refused at load")
- `crates/xtask/src/dsl_check.rs`, `crates/xtask/src/dsl_check/coercion.rs`, `crates/spelling/`

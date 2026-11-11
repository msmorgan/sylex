---
needs: [syl-at-turn-point, cleanup-step6-spine, syl-number-snapshot]
---

# Spellings with optional and repeated parts

## Why

A `spelling` string can't say that a part is printed only when an argument is given, or once per
list entry, so most of the 167 `TODO(spelling)` markers, all under `plugins/builtin/`, stand
where a spelling needs one (re-count when claimed; `putInto`'s "Omit the origin phrase when from
is anywhere" is typical). Without it,
wordings that differ only by such a part become near-duplicate Template families: `enters…`,
`activatedOnly…`, `prevent…`, `defines…`.

`syl-at-turn-point`, which lands first, settles `at`'s base spelling "At $point, $action"; `at.syl`'s remaining `TODO(spelling)`, for its limit and if clauses, is this ticket's.

Written against the spine (`cleanup-step6-spine`).

## Design

- The notation is settled in `docs/decisions/spelling.md` §7 (`$name` Holes, `~` ties, `$xs<join>`
  lists, `( … )` optional parts, `<~Category: …>` and `<~inner Category: …>` Tails) with its load
  rules; where this ticket's text differs, §7 governs.

## Done when

- `sylex_syl` parses and formats the notation `docs/decisions/spelling.md` §7 states.
- Every `TODO(spelling)` that needed an optional or repeated segment is written with it.
- The notation has no mark yet for the Holes the two closed rules of `spelling.md` link: a
  context's Subject and Action Holes under passive subject promotion (`does`, `would`, `did`,
  `cant`, `must`, `can`, `gatedBy`, `doesnt`), and a trigger's effect Hole that may hold several
  sentences (`when`, `whenever`, `at`, the delayed and reflexive triggers). Their optional parts
  are written like any other, no mark is invented for those Holes, and the landing lists these
  declarations as standing outside the check that a variant is English until `spelling.md`
  gives the mark.
- The near-duplicate Template families that differed only by such a segment are each one
  Template; the landing lists the merges.
- Declaration order stays the printed order (`syl.md` §5).
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p sylex_syl` and
  `cargo test -p xtask` pass.

## Not in scope

The other spelling conventions (verb inflection, the hole syntax): `syl-spelling-conventions`.

## Pointers

`docs/decisions/spelling.md` §7, `crates/syl/`, `rg 'TODO\(spelling\)' plugins/`.

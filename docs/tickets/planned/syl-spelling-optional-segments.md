---
needs: [syl-at-turn-point, cleanup-step6-spine, syl-number-snapshot]
---

# Spellings with optional and repeated parts

## Why

A `spelling` string can't say that a part is printed only when an argument is given, or once per
list entry, so 151 `TODO(spelling)` markers stand where a spelling needs one (re-count when
claimed; `putInto`'s "Omit the origin phrase when from is anywhere" is typical). Without it,
wordings that differ only by such a part become near-duplicate Template families: `enters…`,
`activatedOnly…`, `prevent…`, `defines…`.

`syl-at-turn-point`, which lands first, settles `at`'s base spelling "At <point>, <action>"; `at.syl`'s remaining `TODO(spelling)`, for its limit, if and joins clauses, is this ticket's.

Written against the spine (`cleanup-step6-spine`).

## Design

- The notation is settled in `docs/decisions/spelling.md` §7 (`$name` Holes, `$xs<join>` lists,
  `( … )` optional parts, `<Category: …>` Tails); where this ticket's text differs, §7 governs.

## Done when

- `sylex_syl` parses and formats the notation `docs/decisions/spelling.md` §7 states.
- Every `TODO(spelling)` that needed an optional or repeated segment is written with it.
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

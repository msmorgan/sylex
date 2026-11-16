---
needs: [syl-at-turn-point, cleanup-step6-spine, syl-number-snapshot]
---

# Spellings with optional and repeated parts

## Why

A `spelling` string can't say that a part is printed only when an argument is given, or once per
list entry, so most of the 168 `TODO(spelling)` markers, all under `plugins/builtin/`, stand
where a spelling needs one (re-count when claimed; `putInto`'s "Omit the origin phrase when from
is anywhere" is typical). Without it,
wordings that differ only by such a part become near-duplicate Template families: `enters…`,
`activatedOnly…`, `prevent…`, `defines…`.

`syl-at-turn-point`, which lands first, settles `at`'s base spelling "At $point, $action"; `at.syl`'s remaining `TODO(spelling)`, for its limit and if clauses, is this ticket's.

Written against the spine (`cleanup-step6-spine`).

## Design

- The notation is settled in `docs/design/spelling.md` §7 (`$name` Holes, `$name@Form` printed
  forms, `$name*` run Holes, `~` ties, `$xs<join>` lists (a join is nothing, `,`, `.`, `and`, `or`,
  `and/or` or `then`, never chosen by a parameter), `( … )` optional parts, `<~Category: …>` and
  `<~inner Category: …>` Tails, the `^` Gap mark) with its load rules; where this ticket's text
  differs, §7 governs.
- The notation is parsed by a new crate, `sylex_spelling` (`crates/spelling/`), whose first
  module it is. `sylex_syl` parses the `.syl` language and nothing else: a `spelling` stays a
  string to it. `cargo xtask dsl check` parses every `spelling` with it once no `<name>` Hole is
  left (`syl-spelling-conventions`).
- `cargo xtask dsl fmt` does not normalize a `spelling`: `sylex_syl`'s printer writes the string
  as it stands, and `sylex_spelling` parses and has no printer.

## Done when

- `sylex_spelling` exists and parses the notation `docs/design/spelling.md` §7 states, form
  labels, `*` and the `<.>` join included, and refuses a form outside §7's set, with a negative
  test. No code under `crates/syl/` reads inside a `spelling`, and `cargo xtask dsl fmt` leaves
  every `spelling` as written.
- Every `spelling` this ticket writes or merges parses with `sylex_spelling`: a one-off script,
  kept in `docs/evidence/`, parses each, and the landing reports the count. The spellings it
  leaves still write `<name>` Holes, so `cargo xtask dsl check` parsing every `spelling` under
  `plugins/` is `syl-spelling-conventions`'s.
- `docs/design/spelling.md` §8 says what the spelling crate holds, where it says no spelling
  crate exists, and `docs/design/architecture.md` says `sylex_spelling` parses the `spelling`
  notation, beside its sentence that `sylex_syl` parses `.syl`.
- Every `TODO(spelling)` that needed an optional or repeated segment is written with it.
- The notation has no mark for the Holes of the words that take a `Subject` and a verb phrase
  (`does`, `would`, `did`, `cant`, `must`, `can`, `gatedBy`, `doesnt`), whose `Subject` selects
  the voice the verb phrase prints. Their optional parts are written like any other, no mark is
  invented for those Holes, and the landing lists these declarations as standing outside the
  check that a variant is English until each takes its subject as a noun phrase
  (`syl-passive-participle-and-trigger-events` and the three tickets after it).
- The optional parts this ticket writes leave form labels, `*` and `<.>` joins unwritten; that
  is `syl-spelling-forms-and-runs`'s.
- The parser refuses a join chosen by a parameter (`$xs<$name>`), with a negative test.
- No `coord` primitive has a `spelling` (`plugins/builtin/primitives/{noun,predicate,zone}/coord.syl`
  have none today; each takes its `coordinator` as an argument), and the coordinator word is the
  Template's: the `and`, `or` and `andOr` Templates at `NounPhrase`, `Predicate` and `Location`,
  unspelled today, are spelled `$xs<and>`, `$xs<or>` and `$xs<and/or>` over their list, and the
  `or` at `TriggerCondition` `$conditions<or>`, replacing its `TODO(spelling)`. `isOutlaw`
  ("outlaw") keeps its own wording over an unspelled `coord(andOr, …)` in its body. The
  `Predicate` `coord` and its Templates are what `syl-nominal-descriptions` replaces in
  descriptions; whatever coordination at `Nominal` and `PreModifier` it adds follows the same
  rule.
- The near-duplicate Template families that differed only by such a segment are each one
  Template; the landing lists the merges.
- Declaration order stays the printed order (`syl.md` §5).
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p sylex_spelling` and
  `cargo test -p xtask` pass.

## Not in scope

The other spelling conventions (verb inflection, the hole syntax): `syl-spelling-conventions`.

## Pointers

`docs/design/spelling.md` §7, §8, `docs/design/architecture.md`, `crates/syl/src/tree.rs`
(`METADATA_FIELDS`), `crates/xtask/src/dsl_check.rs`, `rg 'TODO\(spelling\)' plugins/`.

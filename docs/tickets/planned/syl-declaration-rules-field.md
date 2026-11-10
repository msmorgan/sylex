---
needs: [cleanup-step6-surface]
specs: [confers-and-rules]
---

# A `rules` field for rules enabled once per scope match

## Why

A declaration's `confers` is enabled once per holding, so a counter or a keyword ability cannot
carry a rule that applies once however many of it an object has: the [CR#704.5q] annihilation on
`p1p1Counter`, or toxic's poison, which reads the sum of every toxic instance once
[CR#702.164b,702.164c] and so is written apart, as `rules/conferral/toxicPoison.syl`. `syl.md` §2
already lists a `rules` field among a declaration's meaning fields, but no kind in
`crates/xtask/src/dsl_check/schema.rs` has one. `semantics.md` §5 says counter state-based
actions are Conferrals on their definitions [CR#704.5q], and `p1p1Counter.syl` has none.
`engine.md` law 5 says data rows are evaluated "one object at a time", but rows are scoped to
whatever their `scope` matches, players included (`playerDamageLifeLoss`).

Written against the surface (`cleanup-step6-surface`).

## Design

- `card_type`, `subtype`, `supertype`, `counter`, `designation` and `ability` take a `rules` field,
  a list of `{scope, confer}` records (`ConferralRule`), each enabled once per entity matching its
  `scope` (spec §1).
- `cargo xtask dsl check` refuses a `$name` read inside `rules` (spec §8).
- An `ability` block takes a `number` field, the value `keywordNumber(<keyword>)` projects (spec
  §7). Toxic writes `number = $amount`, and its rule moves from `conferral toxicPoison` into
  `toxic.syl` under `rules` [CR#702.164b,702.164c]; a `rules` scope may name its own declaration.

## Done when

- The `rules` and `number` fields are in `crates/xtask/src/dsl_check/schema.rs`, with a negative
  test for a `$name` read inside `rules`.
- `toxic.syl` writes `number = $amount` and carries the poison rule under `rules`;
  `plugins/builtin/rules/conferral/toxicPoison.syl` is deleted, and `totalToxicValue` still reads
  `keywordNumber(toxic)`.
- `p1p1Counter` writes the [CR#704.5q] annihilation under `rules`, and `semantics.md` §5 is true
  of it.
- `syl.md` §2 states what `confers` and `rules` each mean, and `engine.md` law 5 says rows are
  evaluated once per scope match, not one object at a time.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale, after `cargo xtask cite bless` for newly cited rules.

## Not in scope

- Naming top-level rows (`lexical` and `spelling` on `sba` and `conferral`) and the legend rule
  row: `syl-supertype-declarations`.
- The other counter rules that go in `rules` (shield, stun, finality, rad, ten poison):
  `syl-counter-conferrals`.
- `doesntApply` (spec §5): no card in the corpus prints it yet.

## Pointers

- `docs/specs/confers-and-rules.md`
- `crates/xtask/src/dsl_check/schema.rs` (`KINDS`, `CONFERS`, `ConferralRule`),
  `crates/xtask/src/dsl_check.rs` (`Resolver::body`)
- `plugins/builtin/definitions/keyword_ability/toxic.syl`,
  `plugins/builtin/rules/conferral/toxicPoison.syl`,
  `plugins/builtin/templates/quantity/totalToxicValue.syl`,
  `plugins/builtin/definitions/counter/p1p1Counter.syl`
- `docs/decisions/syl.md` §2, `docs/decisions/semantics.md` §5, `docs/decisions/engine.md` law 5

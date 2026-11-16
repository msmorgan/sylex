---
needs: []
---
# Retire two over-policing negative judgments and drop `Number = Plural` on `FloatedObjectAll`

## Why

Two negative judgments refuse forms nothing needs refused (`docs/design/english.md` §3):

- `floated_each_quantifier.txt`: `* Remove it all.` CGEL (Ch. 5, §10.1.1, pp. 427–428) licenses
  *all* after personal pronouns including singular *it* (*it all*, *you all*, *them all*), and line
  30 (`* Remove the counters all.`) already pins the common-noun refusal through `head.Case =
  Accusative` on `FloatedObjectAll` (`head.Number = Plural` exists solely to refuse *it all*).
- `notation_coordination.txt`: `*NounPhrase two or one creatures`. No rule restricts descending
  numeral order (`+NounPhrase two or one creature` parses), while `proximate: [Number]` on
  `coordination DeterminerPhrase` implements CGEL Ch. 5, §3.4, p. 353 n. 13 (*one or two* / *one or
  more* -> plural, 1,926 corpus units) and is already pinned by line 17 (`*NounPhrase one or two
  creature`).

## Design

- Remove `* Remove it all.` from `crates/english/judgments/floated_each_quantifier.txt` and drop
  `head.Number = Plural` from `FloatedObjectAll` in `crates/english/grammar/english.fug` (keeping
  `head.Case = Accusative` to refuse `* Remove the counters all.`).
- Remove `*NounPhrase two or one creatures` from `crates/english/judgments/notation_coordination.txt`,
  keeping `proximate: [Number]` on `coordination DeterminerPhrase` and `*NounPhrase one or two
  creature`.

## Done when

- `Remove it all.` parses (`* Remove the counters all.` stays refused), `*NounPhrase two or one
  creatures` is retired (`*NounPhrase one or two creature` stays refused), and the census shows
  identical Readings and verdicts across the supported corpus.
- `cargo xtask english strata --check`, `cargo test -p sylex_english` and
  `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `crates/english/judgments/floated_each_quantifier.txt`,
  `crates/english/judgments/notation_coordination.txt`.
- `crates/english/grammar/english.fug`: `FloatedObjectAll`, `coordination DeterminerPhrase`.
- `docs/design/english.md` §3, §6.

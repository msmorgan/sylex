---
needs: [cleanup-step6-spine]
---

# `sylex_spelling` parses every `spelling`, and the corpus writes its Holes `$name`

## Why

A declaration's `spelling` is a string in the notation of `docs/design/spelling.md` §7, and
nothing reads it: `sylex_syl` keeps it as metadata and `cargo xtask dsl check` skips it. The
corpus still writes a Hole `<name>` (784 Holes on 601 of 1,790 `spelling` lines; re-count when
claimed), which in §7 is an optional part holding a literal word. So no later spelling ticket
has anything to check its work against, and the old Holes would be misread the moment anything
parsed them.

## Design

- The notation is parsed by a new crate, `sylex_spelling` (`crates/spelling/`), whose first
  module it is. `sylex_syl` parses the `.syl` language and nothing else: a `spelling` stays a
  string to it.
- The parser checks the notation's syntax as §7's grammar block states it. The load rules that
  need a declaration, the lexicon or the English grammar are not this ticket's.
- `cargo xtask dsl fmt` does not normalize a `spelling`: `sylex_syl`'s printer writes the string
  as it stands, and `sylex_spelling` parses and has no printer.

## Done when

- `sylex_spelling` exists and parses the notation §7 states: Holes, ties, form labels, `*`,
  `#<…>` joins with the `#<.>` join included, `< … >` optional parts, `%` Tails and the `^` Gap.
  It refuses, each with a negative test: a form outside §7's set; a join chosen by a parameter
  (`$xs#<$name>`); a second Tail in one `spelling` or optional part; a `^` that is
  not the whole `spelling`; and a Hole that abuts a letter or digit with no tie. A Tail's
  Category is read as one capitalized word; whether the English grammar has it is not checked
  here.
- Every `<name>` Hole under `plugins/` is written `$name` by a one-off script kept in
  `docs/evidence/`, which checks that each declaration's tree is unchanged apart from its
  `spelling`; the landing reports the counts it compared. A declared affix that abuts a Hole
  takes a `~` tie (`landwalk`, `typecycling`, `non`). An ordinal is a form, not a tie:
  `fromTop`'s and `fromBottom`'s "<n>th from …" become "$n@Ordinal from …", and no spelling
  under `plugins/` contains `~th`.
- `which` (`primitives/pronoun/which.syl`) has `spelling = "^"`.
- `cargo xtask dsl check` parses every `spelling` under `plugins/` with `sylex_spelling` and
  refuses one it cannot parse, with a negative test. A declaration with no `spelling` is not
  parsed.
- No code under `crates/syl/` reads inside a `spelling`, and `cargo xtask dsl fmt` leaves every
  `spelling` as written.
- `docs/design/spelling.md` no longer says the declarations still write a Hole `<name>`, that
  `which` has no `spelling` or that no declaration writes a form label (§7), or that no spelling
  crate exists (§8); `docs/design/syl.md` §2 no longer says the corpus still
  writes one; and `docs/design/architecture.md` says `sylex_spelling` parses the `spelling`
  notation, beside its sentence about `sylex_syl`.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p sylex_spelling`,
  `cargo test -p xtask` and clippy for both crates pass.

## Not in scope

- Writing optional parts and joins: `syl-spelling-optional-segments`.
- The load rules of §7 beyond syntax: `syl-spelling-load-rules`.
- The informal wording in a declaration's doc comment (`putInto`'s "put <subject> into
  <destination> [from <from>] [<riders>]"): it is a comment, not a `spelling`.
- `_`, the Gap a shared phrase fills, which `syl-shared-phrase-written-once` declares.

## Pointers

- `docs/design/spelling.md` §7, §8; `docs/design/syl.md` §2; `docs/design/architecture.md`
- `crates/syl/src/tree.rs` (`METADATA_FIELDS`), `crates/xtask/src/dsl_check.rs`
- `plugins/builtin/primitives/pronoun/which.syl`

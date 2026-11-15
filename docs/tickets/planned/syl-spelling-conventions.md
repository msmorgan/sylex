---
needs: [syl-parameter-sigil, lean-retirement, syl-spelling-optional-segments, syl-misnamed-declarations, syl-number-snapshot]
---

# Settle the `spelling` conventions and apply them to the corpus

## Why

`spelling` strings are to be parsed through the English grammar with typed holes, but
today they inflect verbs inconsistently, they still write a Hole `<name>` (784 Holes on 601
`spelling` lines), and 168 `TODO(spelling)` markers, all under `plugins/builtin/`, stand where a
spelling is missing, many carrying generated text instead of a reason.

Two narrower tickets rewrite spellings this ticket also normalizes, and land first:
`syl-spelling-optional-segments` (the merged `enters…`, `activatedOnly…`, `prevent…` and
`defines…` Templates) and `syl-misnamed-declarations` (three declarations' spellings).

## Design

The notation is settled in `docs/design/spelling.md` §7 (`$name` Holes, `$name@Form` printed
forms, `$name*` run Holes, `~` ties, `$xs<join>` lists (a fixed join, never one chosen by a
parameter), `( … )` optional parts, `<~Category: …>` and `<~inner Category: …>` Tails); where this
ticket's text differs, §7 governs. This ticket's corpus rewrite migrates every `<name>` Hole to
`$name`. An ordinal is a form, not a tie: `fromTop`'s and `fromBottom`'s "<n>th from …" become
"$n@Ordinal from …", and no `$n~th` is written.

- A verb is written uninflected wherever its agreement depends on a hole or on the performer;
  agreement is solved when the clause is rendered. Three cases:
  - The subject is outside the declaration: the spelling is the bare verb phrase. `die` is
    `"die"`, `sacrifice` is `"sacrifice $subject"`, `win` is `"win the game"`.
  - The grammatical subject is still a parameter: the hole stays and the verb is uninflected, as
    `phaseOutUntil`'s `"$subject phase out until $event"` and `becomeBlocked`'s `"$subject become
    blocked"`. Every such declaration gives its subject up to the clause it stands in
    (`syl-continuous-effect-subject`, `syl-passive-participle-and-trigger-events`), and none is
    restructured here.
  - The subject is fixed inside the declaration and no hole decides agreement: the verb is
    written as printed. `controllerSacrifices` stays `"$subject's controller sacrifices it"`.
- A spelling that parses in several ways is several Constructions (`spelling.md`, the notation's
  load checks). A wording difference with the same meaning lives in which Template a card calls,
  never in the AST.
- A spelling's Holes stand in its declaration's parameter order, which is printed order, as
  arguments are written in declaration order (`syl.md` §2, §5); a spelling whose Hole order
  differs is `syl-printed-read-order`'s to reorder, not this ticket's.

## Done when

- Every `<name>` Hole under `plugins/` is written `$name`, with a `~` tie where a declared affix
  abuts it (`landwalk`, `typecycling`, `non`), and `fromTop` and `fromBottom` write their ordinal
  `$n@Ordinal`; no spelling under `plugins/` contains `~th`.
- `cargo xtask dsl check` parses every `spelling` under `plugins/` with `sylex_spelling`
  (`syl-spelling-optional-segments`) and refuses one it cannot parse, with a negative test.
- In separate corpus commits: every spelling follows the three inflection cases, and every
  `TODO(spelling)` whose wording needs no optional or repeated segment is resolved, either written
  or replaced by the specific missing wording.
- No verb whose agreement depends on a hole or the performer is inflected, and no marker carries
  generated text.
- The corpus means the same before and after the rewrite: a one-off script, kept in
  `docs/evidence/`, checks that every declaration's tree is unchanged apart from its `spelling`
  and comments, and the landing reports the declaration count it compared.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

Writing the optional parts and list Holes `TODO(spelling)` markers need:
`syl-spelling-optional-segments`. Form labels other than the ordinal, `Counting`, the `one`
Template, `*` run Holes and `<.>` joins: `syl-spelling-forms-and-runs`. Recovering a term and
spelling one out, the rest of the spelling crate.

## Pointers

`docs/design/spelling.md` §1 and §7, `docs/design/syl.md` §2,
`docs/design/lexicon.md` §1, `crates/spelling/`, `crates/xtask/src/dsl_check.rs`.

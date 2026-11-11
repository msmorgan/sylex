---
needs: [syl-parameter-sigil, lean-retirement, syl-spelling-optional-segments, syl-misnamed-declarations, syl-number-snapshot]
---

# Settle the `spelling` conventions and apply them to the corpus

## Why

`spelling` strings are to be parsed through the English grammar with typed holes, but
today they inflect verbs inconsistently, they still write a Hole `<name>` (777 Holes on 594
`spelling` lines), and 167 `TODO(spelling)` markers, all under `plugins/builtin/`, stand where a
spelling is missing, many carrying generated text instead of a reason.

Two narrower tickets rewrite spellings this ticket also normalizes, and land first:
`syl-spelling-optional-segments` (the merged `enters…`, `activatedOnly…`, `prevent…` and
`defines…` Templates) and `syl-misnamed-declarations` (three declarations' spellings).

## Design

The notation is settled in `docs/decisions/spelling.md` §7 (`$name` Holes, `~` ties, `$xs<join>`
lists (a fixed join, never one chosen by a parameter), `( … )` optional parts, `<~Category: …>` and
`<~inner Category: …>` Tails); where this ticket's text differs, §7 governs. This ticket's corpus
rewrite migrates every `<name>` Hole to `$name`.

- A verb is written uninflected wherever its agreement depends on a hole or on the performer;
  agreement is solved when the clause is rendered. Three cases:
  - The performer is outside the declaration (`actor`): the spelling is the bare verb phrase.
    `die` is `"die"`, `sacrifice` is `"sacrifice $subject"`, `win` is `"win the game"`.
  - The grammatical Subject is a parameter: the hole stays and the verb is still uninflected,
    as `phaseOutUntil`'s `"$subject phase out until $event"` while it keeps its parameter (the
    open question below). `becomeBlocked` and `resolve` are not this case:
    `syl-shared-phrase-written-once` makes them subjectless, read like `die` (`"become blocked"`,
    `"resolve"`), their subject the context's.
  - The Subject is fixed inside the declaration and no hole decides agreement: the verb is
    written as printed. `controllerSacrifices` stays `"$subject's controller sacrifices it"`.
- A spelling that parses in several ways is several Constructions (`spelling.md`, the notation's
  load checks). A wording difference with the same meaning lives in which Template a card calls,
  never in the AST.
- A spelling's Holes stand in its declaration's parameter order, which is printed order, as
  arguments are written in declaration order (`syl.md` §2, §5); a spelling whose Hole order
  differs is `syl-printed-read-order`'s to reorder, not this ticket's.

## Open questions

- **`becomeBlocking`, `stopBlocking` and `phaseOutUntil`** ("$subject becomes blocking $blocked",
  "$subject stops blocking $blocked", "$subject phases out until $event") take their subject as a
  parameter, as `becomeBlocked` did, and nothing in `plugins/` calls them. Each prints its own
  subject in the active: the creature that begins or stops blocking, the permanent that phases
  out. Whether they become subjectless like `becomeBlocked`. Recommendation: yes, each when a
  card first calls it, so its subject is the context's.

## Done when

- Every `<name>` Hole under `plugins/` is written `$name`, with a `~` tie where letters abut
  it.
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
`syl-spelling-optional-segments`. The spelling crate itself.

## Pointers

`docs/decisions/spelling.md` §1 and §7, `docs/decisions/syl.md` §2,
`docs/decisions/lexicon.md` §1, `crates/syl/`.

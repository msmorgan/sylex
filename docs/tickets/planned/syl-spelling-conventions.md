---
needs: [syl-parameter-sigil, lean-retirement, syl-spelling-optional-segments, syl-misnamed-declarations, syl-number-snapshot]
---

# Settle the `spelling` conventions and apply them to the corpus

## Why

`spelling` strings are to be parsed through the English grammar with typed holes, but
today they inflect verbs inconsistently, and 151 `TODO(spelling)` markers stand where a spelling
is missing, many carrying generated text instead of a reason.

Two narrower tickets rewrite spellings this ticket also normalizes, and land first:
`syl-spelling-optional-segments` (the merged `enters…`, `activatedOnly…`, `prevent…` and
`defines…` Templates) and `syl-misnamed-declarations` (three declarations' spellings).

## Design

The notation is settled in `docs/decisions/spelling.md` §7 (`$name` Holes, `$xs<join>` lists,
`( … )` optional parts, `<Category: …>` Tails); where this ticket's text differs, §7 governs. This
ticket's corpus rewrite migrates every `<name>` Hole to `$name`.

- A verb is written uninflected wherever its agreement depends on a hole or on the performer;
  agreement is solved when the clause is rendered. Three cases:
  - The performer is outside the declaration (`actor`): the spelling is the bare verb phrase.
    `die` is `"die"`, `sacrifice` is `"sacrifice $subject"`, `win` is `"win the game"`.
  - The grammatical Subject is a parameter: the hole stays and the verb is still uninflected.
    `becomeBlocked(subject)` is `"$subject become blocked"`, likewise `becomeBlocking`,
    `stopBlocking` and `phaseOutUntil`. These keep `subject` because their Subject is the
    patient, not the performer (`does(any, becomeBlocked(this))`, `semantics.md` §4.2).
  - The Subject is fixed inside the declaration and no hole decides agreement: the verb is
    written as printed. `controllerSacrifices` stays `"$subject's controller sacrifices it"`.
- A spelling with several parses keeps every parse that recovers to its `body`, and fails to load
  if none does. A wording difference with the same meaning lives in which Template a card calls,
  never in the AST.

## Done when

- `docs/decisions/spelling.md` §1 states the inflection rule and §7 the notation, which
  `docs/decisions/syl.md` §2 points at, and the glossary has a **Hole** entry.
- Every `<name>` Hole under `plugins/` is written `$name`, or `$<name>` where a word character
  follows it.
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

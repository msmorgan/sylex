---
needs: [syl-parameter-sigil, lean-retirement, syl-spelling-optional-segments, syl-misnamed-declarations]
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

- A verb is written uninflected wherever its agreement depends on a hole or on the performer;
  agreement is solved when the clause is rendered. Three cases:
  - The performer is outside the declaration (`theActor`): the spelling is the bare verb phrase.
    `die` is `"die"`, `sacrifice` is `"sacrifice <subject>"`, `win` is `"win the game"`.
  - The grammatical Subject is a parameter: the hole stays and the verb is still uninflected.
    `becomeBlocked(subject)` is `"<subject> become blocked"`, likewise `becomeBlocking`,
    `stopBlocking` and `phaseOutUntil`. These keep `subject` because their Subject is the
    patient, not the performer (`does(any, becomeBlocked(this))`, `semantics.md` §4.2).
  - The Subject is fixed inside the declaration and no hole decides agreement: the verb is
    written as printed. `controllerSacrifices` stays `"<subject>'s controller sacrifices it"`.
- A hole is `<name>`; angle brackets never appear in Oracle text. Square brackets do (cleave), so
  they are never markup.
- A spelling with several parses keeps every parse that recovers to its `body`, and fails to load
  if none does. A wording difference with the same meaning lives in which Template a card calls,
  never in the AST.

## Done when

- `docs/decisions/syl.md` §2 states the conventions, and the glossary has an entry for a
  spelling's holes.
- In separate corpus commits: every spelling follows the three inflection cases, and every
  `TODO(spelling)` whose wording needs no optional or repeated segment is resolved, either written
  or replaced by the specific missing wording.
- No verb whose agreement depends on a hole or the performer is inflected, and no marker carries
  generated text.
- The corpus means the same before and after the rewrite: a one-off script, kept in
  `docs/evidence/`, checks that every declaration's tree is unchanged apart from its `spelling`
  and comments, and the landing reports the declaration count it compared.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

How a spelling expresses optional and repeated segments (a list hole, a
defaulted or `Option` parameter), which is open. The spelling crate itself.

## Pointers

`docs/decisions/syl.md` §2,
`docs/decisions/lexicon.md` §1, `crates/syl/`.

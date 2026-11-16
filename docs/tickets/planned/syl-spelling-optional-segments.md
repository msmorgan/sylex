---
needs: [syl-spelling-notation-crate, syl-at-turn-point, syl-number-snapshot]
---

# Spellings with optional and repeated parts

## Why

A `spelling` cannot yet say that a part is printed only when an argument is given, or once per
list entry, so most of the 168 `TODO(spelling)` markers, all under `plugins/builtin/`, stand
where a spelling needs one (re-count when claimed; `putInto`'s "Omit the origin phrase when from
is anywhere" is typical). Without it, wordings that differ only by such a part become
near-duplicate Template families: `enters…`, `activatedOnly…`, `prevent…`, `defines…`.

## Design

- The notation is `docs/design/spelling.md` §7's: an optional part is `< … >` and a list Hole
  states its join, `$xs#<and>`. Where this ticket's text differs, §7 governs.
- A coordinator word is its Template's, never a `coord` primitive's: the `and`, `or` and `andOr`
  Templates at `NounPhrase`, `Predicate` and `Location` are spelled `$xs#<and>`, `$xs#<or>` and
  `$xs#<and/or>` over their list, and the `or` at `TriggerCondition` `$conditions#<or>`.
- A move's riders are not one list under one join. "Onto the battlefield tapped and attacking"
  coordinates two with "and", "onto the battlefield tapped under their control" sets two side
  by side, and no card prints "tapped attacking". They are depictives, and their wording waits
  for that design (`docs/design/semantics.md` §6 item 13).

## Open questions

- **Modes printed one to a line.** `chooseModes` and `chooseSpree` ask for their modes as
  lines, and no join of §7 prints a line break. Options: a join for a line break; or the modes
  as a run of sentences, which `syl-spelling-forms-and-runs` writes. Recommendation: the owner
  rules which, and until then both markers stay, naming this question.

## Done when

- The open question is answered.
- Every `TODO(spelling)` whose wording needs an optional part or a list's join is written with
  it and checked against a card's Oracle text, except the ones below, whose markers stay, each
  rewritten to name what it waits on:
  - a move's riders: the markers on `create`, `enter`, `returnTo`, `put`, `putOntoBattlefield`,
    `zoneChange`, `returnToBattlefield`, `lookAndSortSlice` and `putInto` (re-run the search
    when claimed). `putInto`'s origin phrase is written, "< from $from>";
  - a word for an empty list (`creatureToken`, `creatureTokenOf` and `artifactCreatureToken`
    want "colorless"), which §7 does not have: `syl-token-records-to-templates`;
  - a part printed as a following sentence (`withCharacteristicEdits`):
    `syl-spelling-forms-and-runs`.
- Every list Hole states its join, the one its wording prints, checked against a card. Twelve
  spellings write a list Hole with no join when this ticket is claimed (`abilityGrant`,
  `abilityLoss`, `alsoHas`, `byLabel`, `cantSpendOn`, `spendOnly`, both `symbols`,
  `distinctChosenKinds`, and `enters`, `entersUnless` and `mayHaveEnter`; re-run the scan when
  claimed). The last three hold a move's riders: their riders Hole is taken out and a marker
  names the depictives design, so a term with riders has no wording by them.
- The coordinator Templates are spelled as Design states, the `or` at `TriggerCondition`
  replacing its `TODO(spelling)`, and no `coord` primitive has a `spelling`. `isOutlaw` keeps
  its own wording over an unspelled `coord(andOr, …)` in its body.
- The near-duplicate Template families that differed only by an optional or repeated part are
  each one Template; the landing lists the merges.
- The optional parts this ticket writes carry no form label, `*` or `#<.>` join.
- `docs/design/spelling.md` §7 no longer says the coordinator Templates have no spelling.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- The parser and the migration of the old Holes: `syl-spelling-notation-crate`.
- The load rules a declaration settles: `syl-spelling-load-rules`.
- Verb inflection and the markers that need no optional or repeated part:
  `syl-spelling-conventions`.
- Form labels, runs and `#<.>` joins: `syl-spelling-forms-and-runs`.

## Pointers

- `docs/design/spelling.md` §3.1, §7; `docs/design/semantics.md` §6 item 13
- `rg 'TODO\(spelling\)' plugins/`; `plugins/builtin/templates/action/putInto.syl`;
  `plugins/builtin/primitives/{noun,predicate,zone}/coord.syl`

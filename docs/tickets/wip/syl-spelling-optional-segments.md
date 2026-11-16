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
  `$xs#<and/or>` over their list.
- A move's riders are left unspelled. They are depictives, not one list under one join ("tapped
  and attacking" beside "tapped under their control"), and their wording waits for that design
  (`docs/design/semantics.md` §6 item 13).
- A list of modes takes the line-break join, written `$modes#<\n>` with the `.syl` string escape
  for a line break (the printer writes every line break in a string that way), as the grammar's
  `Modes` joins its `Mode`s, and a mode's bullet is the mode's own wording, with its cost clause an
  optional part.

## Done when

- `sylex_syl`'s lexer reads `\n` in a string as a line break, still reads a typed one and still
  refuses any other new escape, and its printer writes every line break in a string as `\n`,
  with a test for each. `cargo xtask dsl fmt` has rewritten `chooseOne`'s `spelling`, and
  `docs/design/syl.md` §2 no longer says the lexer lacks the escape.
- `chooseModes` and `chooseSpree` are spelled over their list of modes with the line-break join,
  and a mode's wording prints its bullet and, where it has one, its cost; each is checked
  against a modal card (*Cryptic Command*) and a spree card. The landing says whether
  `chooseOne` still needs a wording of its own.
- Every `TODO(spelling)` whose wording needs an optional part or a list's join is written with
  it and checked against a card's Oracle text, except the ones below, whose markers stay, each
  rewritten to name what it waits on:
  - a move's riders: the markers on `create`, `enter`, `returnTo`, `put`, `putOntoBattlefield`,
    `zoneChange`, `returnToBattlefield`, `lookAndSortSlice` and `putInto`, and on the rider
    words themselves (`entersAttacking`, `inOrder`, `withCounters`); re-run the search when
    claimed. `putInto`'s origin phrase is written, "< from $from>";
  - a copy's exceptions (`asCopyOf`, `copy`, `copyOnStack`): `syl-copy-exceptions`;
  - a word for an empty list (`creatureToken`, `creatureTokenOf` and `artifactCreatureToken`
    want "colorless"), which §7 does not have: `syl-token-records-to-templates`;
  - a part printed as a sentence after the clause (`withCharacteristicEdits`, `addMana`'s
    riders): `syl-spelling-forms-and-runs`.
- Every list Hole states its join, the one its wording prints, checked against a card. Thirteen
  spellings write a list Hole with no join when this ticket is claimed (`abilityGrant`,
  `abilityLoss`, `alsoHas`, `byLabel`, `cantSpendOn`, `spendOnly`, both `symbols`,
  `distinctChosenKinds`, `getEmblem`, and `enters`, `entersUnless` and `mayHaveEnter`; re-run the
  scan when claimed). The last three hold a move's riders: their riders Hole is taken out and a
  marker names the depictives design.
- The coordinator Templates are spelled as Design states, and no `coord` primitive has a
  `spelling`. `isOutlaw` keeps its own `"outlaw"` wording over `andOr([…])` in its body.
- The near-duplicate Template families that differed only by an optional or repeated part are
  each one Template; the landing lists the merges.
- No spelling this ticket writes carries a form label, `*` or `#<.>` join.
- `docs/design/spelling.md` §7 no longer says the coordinator Templates have no spelling.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p sylex_syl`, `cargo test
  -p xtask` and clippy for both crates pass.

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

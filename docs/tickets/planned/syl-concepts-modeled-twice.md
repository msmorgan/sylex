---
needs: [syl-single-face-card-call, cleanup-step6-spine, syl-number-snapshot]
---

# Each concept modeled one way

## Why

Some concepts have two or more `.syl` models side by side: "X'd this way" in five forms, copying a
spell, coordination, "whose", and an optional entry with counters. Two models of one concept give
two readings of one printed phrase, and a card's meaning depends on which its author picked.

The optional entry with counters, "You may have this permanent enter with an additional +1/+1
counter on it" (*Unleash* [CR#702.98a], and *Riot* [CR#702.136a] with "If you don't, it gains
haste"), has two models. `unleash.syl` and `riot.syl` write it as `alongside(may(putCounters(…)))`
inside `replace`, and `mayHaveEnter` (`optionally(withRiders(…))`) can write it as a `withCounters`
Entry Rider. Under both, counters put on a permanent as it enters are either `alongside(putCounters)`
or `withRiders([withCounters(…)])`. `optionally` has no branch for when you decline, so it cannot
write Riot's.

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

Written against the spine (`cleanup-step6-spine`).

## Design

- NounPhrase-level `coord(and, …)` is kept wherever a card coordinates noun phrases: the plural
  group referent comes from it, as in "Untap target creature and each other creature that shares
  a color with it. Those creatures get +2/+0 until end of turn." (*Rally the Righteous*).

## Done when

- For each of the five, the landing names its forms, and one model remains, the others deleted or
  made Templates over it, with every caller in `plugins/` updated; a form kept separate states why
  in its doc comment.
- Unleash, Riot and every other optional entry with counters write one model, and that model
  says Riot's "If you don't" branch.
- Every card that coordinates noun phrases still writes NounPhrase-level `coord(and, …)` there.
- Declaration order stays the printed order (`syl.md` §5).
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

The numeric readbacks ("that much", "that many"): `syl-numeric-anaphors`.

## Pointers

`plugins/builtin/primitives/timing/thisWay.syl` and its callers; `docs/decisions/semantics.md`
§1 item 5(b); `plugins/builtin/definitions/keyword_ability/{unleash,riot}.syl`,
`plugins/builtin/primitives/static/{alongside,optionally,withRiders}.syl`,
`plugins/builtin/templates/ability/mayHaveEnter.syl`.

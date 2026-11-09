---
needs: [cleanup-step6-surface]
---

# Author Balance and its family in `.syl`

## Why

Balance (Restricted in Vintage) and its family (Magus of the Balance, Restore Balance, Balancing
Act; all Legal) have no `.syl` card. Each needs "each player chooses N of what they control,
where N is the count controlled by the player who controls the fewest, then sacrifices (or
discards) the rest", repeated per category. The pieces this needs may already exist:
`instruct(eachPlayer, choose(…))` with `notChosen` (Consuming Tide, Disciple of Caelus Nin),
`choose` followed by `theRest` (Duneblast), and the minimum over players through
`aggregateOver`/`superlative`. Nothing yet shows they compose into "choose exactly that many, act
on the rest" per player, nor how "the same way" repeats the instruction for each category.

Written against the surface (`cleanup-step6-surface`).

## Done when

- `plugins/canon/cards/` holds Balance, Magus of the Balance, Restore Balance and Balancing Act,
  each following its Oracle text, with the categories in printed order and every player's choice
  made in APNAP order before the actions happen [CR#101.4].
- Any construct the cards cannot be written with is added as a new primitive, with the reason in
  its declaration comment.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Running Balance in the engine. The engine reads core, which `.syl` reaches only through the
  step-9 lowering.

## Pointers

- `plugins/canon/cards/Consuming Tide.syl`, `Disciple of Caelus Nin.syl`, `Duneblast.syl`
- `plugins/builtin/templates/noun/eachPlayer.syl`, `plugins/builtin/primitives/predicate/notChosen.syl`,
  `plugins/builtin/primitives/pronoun/theRest.syl`, `plugins/builtin/primitives/quantity/aggregateOver.syl`,
  `plugins/builtin/primitives/predicate/superlative.syl`

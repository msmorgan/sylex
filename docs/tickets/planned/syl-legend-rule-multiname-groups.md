---
needs: [cleanup-step6-surface]
---

# The legend rule groups a permanent with two names correctly

## Why

`legendRule` (`plugins/builtin/rules/sba/legendRule.syl`) makes one group per name, the player
chooses one permanent per group with `doForEachKind(name, …, choose(…))`, and every legendary
permanent that shares a name and was not chosen in any group is put into its owner's graveyard.
A permanent with more than one name is in more than one group, and [CR#704.5j] says the player
chooses one of them and the rest are put into their owners' graveyards. Two cases reach it:

- Rooms, only under an effect that makes Rooms legendary, such as *Leyline of Singularity* ("All
  nonland permanents are legendary."). Two fully unlocked copies of one Room each have both names
  [CR#709.4,709.5], so a player can choose one copy for the left name and the other for the right,
  and neither is unchosen.
- *Spy Kit* ("Equipped creature gets +1/+1 and has all names of nonlegendary creature cards in
  addition to its name."), one on each of two legendary creatures, gives them thousands of names
  in common. A row that chooses once per shared name asks for thousands of choices and, if the
  player picks different permanents for different names, leaves both.

The priority is low.

## Design

- The row keeps the rule's own sentence inside its loop: the player chooses one of them, then
  `putInto(theRest, graveyard)` [CR#704.5j].
- The loop runs over the distinct groups of legendary permanents the player controls that have a
  name in common [CR#201.2a], each group counted once however many names its members share, not
  over names.

## Open questions

- The overlapping case [CR#704.5j] leaves unstated, with no ruling behind any reading: one fully
  unlocked Room, a copy of it with only its left half unlocked and a copy with only its right half
  unlocked [CR#709.5], all legendary. The Room shares a name with each copy and the two copies
  share none. Is that one group or two, and which permanents may stay?

## Done when

- `legendRule` loops over groups as the Design says, putting each group's rest into its owners'
  graveyards, and its doc comment no longer names the multi-name case as open.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/rules/sba/legendRule.syl`, `plugins/builtin/primitives/action/doForEachKind.syl`,
`plugins/builtin/primitives/predicate/notChosen.syl`, `plugins/builtin/primitives/kindAxis/name.syl`,
`plugins/builtin/primitives/pronoun/theRest.syl`.

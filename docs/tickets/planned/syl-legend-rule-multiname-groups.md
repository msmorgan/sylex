---
needs: [cleanup-step6-surface]
---

# The legend rule groups a permanent with two names correctly

## Why

`legendRule` (`plugins/builtin/rules/sba/legendRule.syl`) makes one group per name, the player
chooses one permanent per group with `doForEachKind(name, …, choose(…))`, and every legendary
permanent that shares a name and was not chosen in any group is put into its owner's graveyard.
A permanent with two names is in two groups. Under *Leyline of Singularity* ("All nonland
permanents are legendary."), two fully unlocked copies of one Room each have both names
[CR#709.4,709.5], so a player can choose one copy for the left name and the other for the right,
and neither is unchosen. [CR#704.5j] says the player chooses one of them and the rest are put into
their owners' graveyards, so one copy must go.

## Open questions

- Which reading fixes it: a per-group "not chosen" (each group's rest goes, whatever was chosen
  in other groups), or one group-level choice over the permanents that share a name?
- The overlapping case [CR#704.5j] leaves unstated: one fully unlocked Room, a copy of it with only
  its left half unlocked and a copy with only its right half unlocked [CR#709.5], all legendary.
  The Room shares a name with each copy and the two copies share none. Is that one group or two,
  and which permanents may stay?

## Done when

- `legendRule` puts the rest of each group into its owners' graveyards for permanents with two
  names, by the reading the owner picks, and its doc comment no longer names the two-name case as
  open.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/rules/sba/legendRule.syl`, `plugins/builtin/primitives/action/doForEachKind.syl`,
`plugins/builtin/primitives/predicate/notChosen.syl`, `plugins/builtin/primitives/kindAxis/name.syl`.

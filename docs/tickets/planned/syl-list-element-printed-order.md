---
needs: [cleanup-step6-spine]
---

# Put every list's elements in printed order

## Why

Read order is printed order for the elements of a list as for parameters (`syl.md` §5), but the
corpus often writes a conjunction's head noun before a modifier the card prints first. "Basic
land" is `and([isLand, hasSupertype(basic)])` in *Celebrate the Harvest*, *Gauntlet of Power*,
*Land Tax*, *Old-Growth Dryads*, *Thawing Glaciers* and `landerToken.syl`, and *Arcum's
Weathervane* writes "snow land" and "nonsnow basic land" the same way: 8 lists put `isLand`
before a supertype, where *Blizzard Brawl* and *Skred* write `and([hasSupertype(snow), …])` in
printed order. Counting only an `and([…])` whose first conjunct is a card-type head (`isLand`,
`isCreature`, `isPermanent`, `isSpell` and the like) and whose second is a modifier usually
printed before the noun (`hasSupertype`, `non(…)`, `hasColor`, `isTapped`, `isUntapped`,
`isAttacking`, `isBlocking`, `isToken`) finds about 60, as `and([isPermanent, non(isLand)])` for
"nonland permanent" (*Abrupt Decay*) and `and([isSpell, hasColor(white)])` for "white spell"
(*Gloom*). That count misses a modifier in a later position (`isOther`, `isNontoken`) or written
`not(…)`, and counts some postmodifiers, so the sweep surveys every list.

Written against the spine (`cleanup-step6-spine`).

## Done when

- Every list in `plugins/` whose element order differs from the order its card or declaration
  prints them is reordered to match: the conjuncts of `and([…])`, `or([…])` and `andOr([…])`, and
  the steps of `sequentially([…])`. A list whose printed order is not determined by the text (a
  conjunct no word prints) is left as it is, and the landing lists those.
- The reordered corpus means the same: a one-off script, kept in `docs/evidence/`, checks that
  every reordered list holds the same elements before and after, and that nothing else in the
  tree changed; the landing reports the list counts it compared.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- A declaration's parameter order: `syl-printed-read-order`.

## Pointers

- `docs/decisions/syl.md` §5
- `plugins/canon/cards/Celebrate the Harvest.syl`, `plugins/canon/cards/Arcum's Weathervane.syl`,
  `plugins/builtin/definitions/token/landerToken.syl`

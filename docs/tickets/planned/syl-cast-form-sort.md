---
needs: [cleanup-step6-surface]
---

# Cast characteristics are their own sort

## Why

`cast`'s and `play`'s `payment` (sort `PlayPayment`) holds the characteristics a spell is cast
with: `faceDown(spec, cost)` (morph, megamorph, disguise), `prototyped` [CR#718.3],
`fused` [CR#702.102a], `transformedPaying(cost)` and its Template `convertedPaying` (disturb,
more than meets the eye) [CR#712.8c]. Two of them also carry the alternative cost they are paid
with, which `cast`'s `altCost` slot holds for every other cast, so a cast characteristic and an
alternative cost are written two ways. Other cast characteristics stand elsewhere or nowhere:
a Siege's "cast it transformed" is `resulting = isTransformed`, and casting a bestow card bestowed,
which makes it an Aura spell with enchant creature [CR#702.103b], has no word.

## Done when

- A cast characteristic is a value of its own sort, in its own slot of `cast` and `play`, and
  carries no cost: morph's {3} [CR#702.37c] and disturb's cost [CR#702.146a] are `altCost`
  values.
- `itsOwnCost` and `PlayPayment` are gone, or the ticket's landing says what each still means.
- Bestow casts the card bestowed [CR#702.103a,702.103b], and a Siege's cast transformed uses the
  transformed characteristic [CR#310.12b].
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/cost/{faceDown,prototyped,fused,transformedPaying,itsOwnCost}.syl`,
`plugins/builtin/templates/cost/convertedPaying.syl`,
`plugins/builtin/definitions/keyword_action/{cast,play,activate}.syl`,
`plugins/builtin/primitives/action/procedure.syl`,
`plugins/builtin/definitions/keyword_ability/bestow.syl`,
`plugins/builtin/definitions/subtype/battle/siege.syl`.

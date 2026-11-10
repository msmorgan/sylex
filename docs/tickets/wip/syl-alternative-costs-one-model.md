---
needs: [syl-boolean-flags]
---

# Alternative and additional costs modeled one way

## Why

An alternative or additional cost is written two ways: as `altCost`/`addedCost` static specs, and
as payment options (`payingInstead`, `withoutPaying`, `plusAdditional`, `usingKeyword`) in a
`can(…, cast(…))` permission. The CR keeps alternative and additional costs apart
[CR#118.8,118.9], but not two models of each. Nor is it checked that the keywords written with
`altCost` function where their rules say: evoke's and bestow's static abilities function in any
zone the card can be cast from [CR#702.74a,702.103a], and dash's while the card is on the stack
[CR#702.109a]. And `foretoldCardLook.syl` leaves a foretold card's cast
for its foretell cost [CR#702.143d] unwritten, saying no cost term names a keyword's cost, though
`usingKeyword` exists.

## Design

- An alternative cost is a value of sort `AltCost`: `payingInstead(cost)`, `withoutPaying` or
  `usingKeyword(label)` [CR#118.9]. An additional cost is a value of sort `AddedCost`:
  `plusAdditional(cost, times = once)` [CR#118.8]. No other declaration builds either.
- The printed sentence decides where the value stands. "You may pay C rather than pay …" and "As
  an additional cost to cast …" are the statics `altCost(subject, cost)` and
  `addedCost(subject, optionality, cost)`. "You may cast X [by paying C | without paying its mana
  cost]", in a permission or an instructed cast, fills the `altCost` or `addedCost` slot of
  `cast`, `play` or `activate`. A keyword stands where its rule's sentence puts it: evoke ("You
  may cast this card by paying [cost] rather than paying its mana cost") is a permission, mutate
  ("You may pay [cost] rather than pay this spell's mana cost") a static.
- A cast permission widens only the coordinates it states (`docs/decisions/semantics.md` §4.7),
  so a permission with no `from` writes *Omniscience*'s and evoke's alternative cost without
  granting a zone.
- Cast characteristics (`faceDown`, `prototyped`, `fused`, `transformedPaying`) stay in `cast`'s
  `payment`, sort `PlayPayment`, whose default `itsOwnCost` is the ordinary cast;
  `syl-cast-form-sort` splits them out.

## Done when

- Each of alternative and additional cost has one model, the other form deleted or a Template
  over it, with every caller in `plugins/` updated.
- Evoke, bestow and dash function exactly where their rules say [CR#702.74a,702.103a,702.109a];
  the landing quotes each rule beside its body.
- `plugins/builtin/rules/conferral/foretoldCardLook.syl` writes the foretell-cost cast
  [CR#702.143d].
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/static/{altCost,addedCost}.syl`,
`plugins/builtin/definitions/keyword_ability/{evoke,dash,bestow,foretell}.syl`,
`plugins/builtin/definitions/keyword_action/cast.syl` (`PlayPayment`),
`plugins/builtin/rules/conferral/foretoldCardLook.syl`,
`docs/decisions/semantics.md` §1 item 5(a).

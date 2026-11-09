---
needs: []
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

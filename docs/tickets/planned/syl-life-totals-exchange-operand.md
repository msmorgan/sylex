---
needs: []
---

# Decide whether `.syl`'s `lifeTotals` exchange operand folds into `values`

## Why

The `exchange` primitive takes one `Exchanged` operand, with five constructors:
`controlOf` [CR#701.12b], `cardsAcross` [CR#701.12d], `zones` [CR#701.12f], `values`
[CR#701.12g] and `lifeTotals` [CR#701.12c]. Exchanging life totals is exchanging two numerical
values, so `exchangeValues` over two `lifeTotalOf` amounts could say what `exchangeLifeTotals`
says, and `semantics.md` §1 item 5(b) prefers composing over minting. `lifeTotals` exists because
it takes one group of players, so "Two target players exchange life totals" (Soul Conduit, Axis of
Mortality, Profane Transfusion) names both parties as one plural target group; `values` takes two
separate amounts. Canon's one user today, Mirror Universe, writes
`exchangeLifeTotals(youAnd(target(isOpponent)))`.

## Open questions

- Whether `lifeTotals` folds into `values` (or a pairwise form over one group of players), or stays
  its own operand.

## Done when

- Either `lifeTotals` is retired, every life-total exchange is written through `values` (or a
  pairwise form over one group of players), and "two target players" stays a single plural target
  group; or `lifeTotals` stays and `exchange.syl`'s doc comment states why.
- The life-total exchange cards in canon (Mirror Universe today) keep their printed wording.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/action/exchange.syl`, `lifeTotals.syl`, `values.syl`;
`plugins/builtin/definitions/keyword_action/exchangeLifeTotals.syl`, `exchangeValues.syl`;
`plugins/builtin/templates/quantity/lifeTotalOf.syl`; `plugins/canon/cards/Mirror Universe.syl`;
`docs/decisions/semantics.md` §1.

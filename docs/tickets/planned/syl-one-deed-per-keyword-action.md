---
needs: [syl-explicit-deeds, syl-single-face-card-call, syl-life-totals-exchange-operand]
---

# One declaration and one Deed per keyword action

## Why

Several keyword actions are declared once per printed variant, each variant with its own Deed
under the label of the keyword action it is, so "exiled this way", "can't be exiled" or "whenever
you cast" can match them only by that label string:

- Exchange [CR#701.12a]: `exchangeCards`, `exchangeControl`, `exchangeLifeTotals`,
  `exchangeTextBoxes`, `exchangeValues` and `exchangeZones`, all `"exchange"`, beside the
  `exchange` primitive and `exchangeWith`.
- Exile [CR#701.13a]: `exile`, `exileFrom`, `exileFaceDown`, `exileFaceDownFrom` and
  `exileWithCounters`, all `"exile"`.
- Double [CR#701.10]: `doublePower`, `doubleToughness`, `doublePowerAndToughness`,
  `doubleLifeTotal` and `doubleCounters`, all `"double"`, with no plain `double` declaration.
- Triple [CR#701.11]: `triplePower`, `tripleToughness` and `triplePowerAndToughness`, all
  `"triple"`.
- `millCards` beside `mill` [CR#701.17],
  `revealHand` beside `reveal` [CR#701.20], `searchZonesOf` beside `search` [CR#701.23],
  `shuffleInto` beside `shuffle` [CR#701.24] and `voteStartingWith` beside `vote` [CR#701.38].

Merging a family's declarations merges their Deeds, which is `syl-explicit-deeds`' open question
on one shared exile Deed; the same answer settles every family here, so this ticket builds it for
all of them.

`syl-life-totals-exchange-operand` decides whether `lifeTotals` folds into `values`, rewriting
`exchange.syl` and `exchangeLifeTotals.syl`, which this ticket also rewrites, and lands first.

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

## Done when

- Each keyword action above is one Definition with one Deed, as `syl-explicit-deeds` settles; its
  variants are Templates or arguments over it, keeping their printed wording, and every caller in
  `plugins/` is updated.
- No `deed = "…"` label under `plugins/builtin/definitions/keyword_action/` is shared by two
  declarations.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- The keyword abilities declared many times (`hexproof`, `partner` and the label-joined
  keyword-ability variants): `syl-one-declaration-per-keyword`.
- Whether `lifeTotals` folds into `values`: `syl-life-totals-exchange-operand`.

## Pointers

`plugins/builtin/definitions/keyword_action/{exchange,exile,double,triple}*.syl`,
`plugins/builtin/primitives/action/exchange.syl`, `plugins/builtin/primitives/static/exchangeWith.syl`, the six variant files and
their bases (`cast`, `mill`, `reveal`, `search`, `shuffle`, `vote`) in the same folder,
`docs/decisions/semantics.md` §3 and §3.1.

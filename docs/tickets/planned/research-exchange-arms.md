---
needs: []
---

# Research: which exchanges do Vintage-legal cards perform?

## Why

"Exchange" has several arms in the rules: control of two permanents [CR#701.12b], life
totals [CR#701.12c], cards between zones [CR#701.12d], whole zones [CR#701.12f], numerical values
[CR#701.12g] and text boxes [CR#701.12h]. The text-box and numerical-value arms are rare, and
whether the engine models them depends on which cards use them.

## Done when

- Every Vintage-legal card whose text contains an "exchange" instruction is listed with its
  Oracle text and the arm it uses; a card that fits no arm is called out.
- The cards that exchange text boxes or numerical values are called out.
- The findings are written to `docs/evidence/` and reported to the owner as input to the decision
  whether to model those two arms.

## Pointers

`data/mtgjson/AllPrintings.sqlite`, `data/rules/cr.txt` [CR#701.12].

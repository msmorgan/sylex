---
needs: [syl-active-player-noun-phrase, syl-balance-family, syl-card-comment-modeling-choices, syl-card-head-noun-sweep, syl-cast-permission-spell-anaphor, syl-colored-mana-spent-count, syl-cross-ability-self-replacement, syl-engineered-explosives-sunburst, syl-ice-cauldron-noted-mana, syl-later-ability-acts-before-earlier, syl-life-totals-exchange-operand, syl-lose-control-event, syl-mode-frame-cataphora, syl-replacement-except-nth-event, syl-round-each-time, syl-turn-face-up-spend-cost, syl-unwritable-card-shapes]
---

# Step 6 middle: constructs and cards

## Why

This stage adds the constructs that specific Vintage-legal cards need and cannot yet write, and
writes those cards. Later stages write against it.

## Done when

- Every ticket in its `needs:` is in `done/`.
- A checkpoint pass over `plugins/canon/cards/` and the constructs this stage adds finds card
  text still bent to fit Lean: a printed ability or keyword written out, copied, reordered or
  dropped instead of written as printed, a frame copied into each Mode, an anaphor resolved by
  hand, or a card comment that describes a deleted Lean check instead of the card's modeling
  choice. Each one found is fixed in the checkpoint's own landing or ticketed, and the landing
  lists them.

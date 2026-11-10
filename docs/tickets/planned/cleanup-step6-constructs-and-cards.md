---
needs: [syl-active-player-noun-phrase, syl-already-attached, syl-balance-family, syl-card-comment-modeling-choices, syl-card-head-noun-sweep, syl-cast-form-sort, syl-cast-permission-spell-anaphor, syl-catalog-name-once, syl-cleave-bracketed-text, syl-clone-any-on-the-battlefield, syl-colored-mana-spent-count, syl-cross-ability-self-replacement, syl-cycling-cost-substitute, syl-engineered-explosives-sunburst, syl-ice-cauldron-noted-mana, syl-kind-axis-one-word, syl-later-ability-acts-before-earlier, syl-life-totals-exchange-operand, syl-lose-control-event, syl-mode-frame-cataphora, syl-replacement-except-nth-event, syl-round-each-time, syl-shared-phrase-written-once, syl-single-value-sorts, syl-turn-face-up-spend-cost, syl-turn-point-printed-wordings, syl-unwritable-card-shapes]
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
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

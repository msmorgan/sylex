---
needs: []
---
# Track flipped status and flip cards on the battlefield

## Why

Flipped/unflipped is one of the four status categories every permanent has [CR#110.5], and the
engine does not store it. `Status::Flipped` and `Status::Unflipped` trip a `todo!` in
`crates/engine/src/target.rs` and its snapshot twin in `crates/engine/src/trigger.rs`. A filter
over an unstored status must keep failing loudly: reading a default would change targeting
legality. The card model already has flip cards (`Card::Flip` in `crates/card/src/card.rs`, one
Card Face plus Alternative Characteristics), and the `.syl` `flip` template
(`plugins/builtin/templates/action/flip.syl`) sets the status, but the engine always reads the
normal characteristics. 20 Vintage-legal cards have the flip layout.

## Done when

- A permanent's flipped status is stored; it enters unflipped [CR#110.5b], flipping is one-way,
  and a permanent that leaves the battlefield keeps no memory of it [CR#710.4].
- A flipped `Card::Flip` permanent uses its alternative name, text box, type line, power and
  toughness, keeping its color and mana cost [CR#710.1b,710.1c,710.2]; in every other zone and
  before it flips, only its normal characteristics [CR#710.2].
- The `Status` predicate reads flipped status in `target.rs` and `trigger.rs`.
- Tests use real cards: Kitsune Mystic flipping into Autumn-Tail, Kitsune Sage at the end step
  with two Auras attached, and a flipped permanent's name and power after it returns to hand.

## Pointers

- `crates/core/src/status.rs`: `Status`.
- `crates/card/src/card.rs`: `Card::Flip`.
- `crates/engine/src/target.rs` and `crates/engine/src/trigger.rs`: the `Status` arms.
- `crates/engine/src/object.rs` and `crates/engine/src/derive.rs`: where a permanent's
  characteristics come from its card.
- `plugins/builtin/templates/action/flip.syl`,
  `plugins/canon/cards/Kitsune Mystic {slash}{slash} Autumn-Tail, Kitsune Sage.syl`.
- Flip cards [CR#710] in `data/rules/cr.txt`.

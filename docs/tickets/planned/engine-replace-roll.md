---
needs: [engine-randomness]
---
# Replace coin flips and die rolls (Krark's Thumb family)

## Why

`StaticSpec::ReplaceRoll` (`crates/core/src/continuous.rs`: "if you would flip/roll, instead
flip/roll that many plus N and ignore …") is authorable, but the replacement collector in
`crates/engine/src/replace_registry.rs` does not gather it. A Krark's Thumb or Pixie Guide on the
battlefield does nothing. `FlipCoins`/`RollDice` already emit `CoinFlipped`/`DieRolled` through
the replacement pipeline, so there is a live event to intercept. Die rolls set
`result = natural` because no modifier pipeline exists (`agenda.rs`).

## Done when

- The collector gathers `ReplaceRoll`, and flip and roll batches pass through it.
- The ignore selection is a decision when the player chooses (`IgnoreChosen`, and ties for lowest
  under `IgnoreLowest`). An ignored roll never happened: it fires no triggers and no effects
  apply to it [CR#706.6]. An ignored flip is likewise not the flip's result.
- Effects that fix a flip's result or winner replace the actual result [CR#705.3].
- Die-roll modifiers change the result while the natural result stays recorded [CR#706.2]. When
  several apply, the roller chooses which to apply, rerolls before increases or decreases
  [CR#706.2b], and a modifier with a mana cost gives a mana-ability window [CR#706.2a].
- Tests use Krark's Thumb and Pixie Guide.

## Pointers

- `crates/core/src/continuous.rs` (`StaticSpec::ReplaceRoll`, `IgnoreRule`)
- `crates/engine/src/replace_registry.rs` (the note after the collector),
  `crates/engine/src/agenda.rs` (`RollDice`), `crates/engine/src/event.rs` (`natural`)

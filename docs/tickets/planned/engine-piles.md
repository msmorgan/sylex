---
needs: [core-regions-piles]
---
# Resolve pile separation and pile choice

## Why

Core spells each temporary pile as a `Kind::Pile` register [CR#700.3a,700.3b]: `SeparatePiles`
writes its `dests`, `ChoosePile` reads pile registers in `from` and writes the chosen pile to
`dest`. The engine interprets neither: both fall into the `todo!` seam at the end of
`run_effect` in `crates/engine/src/resolve/effect.rs`. `Action::Shuffle` over anything but a
library returns no work through a catch-all `_ => vec![]` in
`crates/engine/src/resolve/player_action.rs`, so shuffling a pile silently does nothing. Fact or
Fiction, Do or Die, Death or Glory, Fortune's Favor, Steam Augury and Atris, Oracle of
Half-Truths in `plugins/canon/cards/` all separate piles.

## Done when

- `SeparatePiles` surfaces a decision for `by` to put each object of `group` into exactly one of
  `dests.len()` piles, any of them possibly empty [CR#700.3a,700.3d]. Objects stay in their zone,
  and a graveyard keeps its order [CR#700.3c]. Every destination register is written.
- `ChoosePile` surfaces a pick over `from` (or picks at random when `random`), writes the chosen
  pile to `dest`, and runs `then`.
- Shuffling a pile-valued register shuffles it [CR#701.24a]. Any other unsupported shuffle shape
  fails loudly instead of returning no work.
- Tests use Fact or Fiction and Do or Die, built as core values.

## Not in scope

- Whims of the Fates' per-player, cross-region piles. They need a label-free authored and core
  design first; the deleted label-bearing core pile variants are not restored.
- `ArrangePile` (`crates/engine/src/state.rs`) orders cards within one known pile landing in a
  library. It is not partition-and-choose.

## Pointers

- `crates/core/src/effect.rs` (`SeparatePiles`, `ChoosePile`), `crates/core/src/region.rs`
- `crates/engine/src/resolve/effect.rs`, `crates/engine/src/resolve/player_action.rs`,
  `crates/engine/src/activation.rs` (`Value::Pile`)
- `plugins/builtin/primitives/action/separateIntoPiles.syl`, `plugins/builtin/primitives/pile/`

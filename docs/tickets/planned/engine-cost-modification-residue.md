---
needs: []
---
**Engine: fold non-mana components and non-simple symbols in cost increases and reductions.**

## Why

The total-cost pipeline in `crates/engine/src/cast.rs` [CR#601.2f] handles only mana. Three arms
stop with a loud `todo!` that names this ticket:

- `add_mana_components`: an increase that reaches a non-mana `CostComponent`;
- `reduce_mana_components`: a reduction that reaches a non-mana `CostComponent`;
- `reduce_symbol`: a reduction symbol other than generic or a single colored/{C} pip (hybrid,
  Phyrexian, snow, {X}). A hybrid reduction has the payer choose a half when the reduction
  applies [CR#118.7e].

Each component kind needs its own ruling, because folding is not uniform. "Spells cost {1} less
to cast" does not reduce an additional sacrifice, for example. So each arm is widened when a real
card reaches it.

## Done when

- Each component kind or symbol kind that a card in `plugins/canon/` reaches has its fold
  implemented, with a test grounded in that card. The `todo!` stays for every kind no card
  reaches.

## Not in scope

- Choosing an alternative cost at announcement: `engine-alt-costs`.
- An unmatched colored reduction falling through to generic [CR#118.7b]:
  `engine-cost-and-search-rule-gaps`.

## Pointers

- `crates/engine/src/cast.rs` (`add_mana_components`, `reduce_mana_components`, `reduce_symbol`)
- `crates/core/src/continuous.rs` (`CostChange`), `crates/core/src/cost.rs` (`CostComponent`)

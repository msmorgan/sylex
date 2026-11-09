---
needs: []
---

# Core and engine: a punisher whose escape is a choice between two costs

## Why

Perforating Artist: "each opponent loses 3 life unless that player sacrifices a nonland
permanent of their choice or discards a card." `.syl` can write the escape as one cost with a
choice in it (the `or` cost primitive and its `either` Template, used by Earthen Goo's cumulative
upkeep), but core cannot: `sylex_core::Cost` is a list of `CostComponent`s that must all be paid,
and `May` with `Pay(cost)` takes one such list. Two nested `May(Pay(…))`s would ask two yes/no
questions in a row instead of offering one choice among three outcomes, so they are not a faithful
substitute. Whatever shape core gets must keep the [CR#118.12a] reading (the escape is something
the affected player may do) and make the payer the loop's player, not the ability's controller.

## Open questions

- The shape core gives a cost that is a choice among costs, and how the engine offers the paying
  player that one decision.

## Done when

- Core can express a cost that is a choice among costs, as `.syl`'s `or` does, and the engine
  offers the paying player one decision among the alternatives (or declining).
- `crates/engine/tests/region_witnesses.rs`'s
  `perforating_artist_taxes_each_opponent_who_will_not_pay` loses its `#[ignore]` and passes with
  the punisher spelled, alongside the Raid intervening "if" and the per-opponent loop already in
  that fixture, and without new syntax in `v1/`.

## Pointers

`crates/core/src/cost.rs` (`Cost`, `CostComponent`), `crates/core/src/effect.rs`
(`May`), `crates/engine/tests/region_witnesses.rs` (Perforating Artist, and Painful Quandary for
the single-cost punisher), `plugins/builtin/primitives/cost/or.syl`,
`plugins/builtin/templates/cost/either.syl`, `plugins/builtin/templates/action/doUnless.syl`,
`docs/decisions/core.md`.

---
needs: []
---
# "This combat" and "this step" history windows

## Why

History reads choose their entries by a `Lookback` window. `ThisTurn`, `ThisGame`, `LastTurn`
and `SinceYour(step)` (echo's "since the beginning of your last upkeep" [CR#702.30a]) work;
`ThisCombat` and `ThisStep` reach `unreachable!` in `window_contains`
(`crates/engine/src/eval.rs`). Cards read them: Clockwork Beast ("if this creature attacked or
blocked this combat"), Cathedral Membrane ("each creature it blocked this combat", read as it
dies) and Harsh Justice ("only if you've been attacked this step").

How `ThisCombat` and `ThisStep` anchor when read outside that combat phase or step, for example
from a delayed trigger, is open and needs a ruling per consumer before the build.

## Done when

- `ThisCombat` selects the entries since the current combat phase began [CR#506.1] and `ThisStep`
  those since the current step began, anchored by log position on the phase's or step's onset,
  the way `in_window_for` anchors `SinceYour`.
- Their meaning when read outside that combat phase or step is ruled for each consumer and
  tested.
- `SinceYour` can no longer reach `window_contains`: the type rules it out instead of an
  `unreachable!`.
- The `in_window` doc comment no longer calls the sub-turn windows unemittable.
- Tests use the real cards above.

## Pointers

- `crates/engine/src/history.rs`: `in_window`, `in_window_for`, `position_in_window_for`.
- `crates/engine/src/eval.rs`: `window_contains`.
- `crates/core/src/temporal.rs`: `Lookback`.

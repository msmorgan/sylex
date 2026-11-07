---
needs: [engine-history-tallies]
---
# Cached counters for hot history queries

## Why

Every history query scans the full-game log in `crates/engine/src/history.rs`:
`lands_played_this_turn` (read on every land-play legality check) and the
`Count::EventCount`/`Count::EventSum` readers that count spells cast (storm) or cards drawn. Hot queries could read
constant-time counters kept beside the log, with the log staying the one source of truth. This
is an optimization, so it waits until the engine is functionally complete.

## Done when

- The hot queries, chosen by profiling a full game in `crates/engine/tests/full_game.rs`, read
  counters derived from the log, updated in `History::record`.
- A test checks, for each cached query, that the counter equals a scan of the log after a game
  with lands played, cards drawn and spells cast across several turns.

## Pointers

- `crates/engine/src/history.rs`: `History`, `record`, `scan`, `in_window`.
- `crates/engine/src/resolve/count.rs`: `lands_played_this_turn`, `Count::EventCount`,
  `Count::EventSum`.
- `crates/engine/src/legal.rs`: the land-play check.

---
needs: [tui-polish]
---

# A game-log pane in the TUI

## Why

The hotseat client shows the board and the pending decision, but not what has happened:
triggers, combat damage, deaths, draws and life changes that resolve between a player's decisions
pass unseen. A scrolling log of the game's events lets a player follow the sequence.

## Open questions

- The log's source:
  - **the engine's history**: render `GameState.history`, the engine's append-only `GameEvent`
    log. This needs a public read accessor on `History` (`entries` is `pub(crate)`) and curation of
    which events a player sees (dropping bookkeeping such as `StepBegan`, `Untapped` or per-unit
    `ManaAdded`), but records everything that happened; or
  - **the client's submitted decisions**: a line per decision the human submits, from the same
    `describe_action` data the footer uses. No engine change, but it misses everything the
    engine resolves between decisions.

## Done when

- The log reads the settled source.
- The right-hand column splits into Detail (top) and Log (bottom); the log shows the most recent
  entries, newest at the bottom, and scrolls.
- Each logged entry is a human-readable line, with tests over the formatting.

## Pointers

`crates/engine/src/history.rs`, `crates/engine/src/event.rs` (`GameEvent`),
`crates/tui/src/ui/`, `crates/tui/src/ui/format.rs`.

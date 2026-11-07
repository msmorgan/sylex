---
needs: [tui-shortcuts]
---

# Configurable pass stops in the TUI

**Why.** The TUI's pass modes (`PassMode::Yield` and `PassMode::Turn` in
`crates/tui/src/shortcuts.rs`) stop at a fixed set of conditions: the stack grew or emptied,
combat began, your main phase, your next turn. Real clients (MTGO's stops, Arena) let a player
pick which turn boundaries auto-pass stops at (the opponent's upkeep, beginning of combat, each
end step, before attackers are declared) and pass priority everywhere else.

**Done when.**
- Each player has a set of stop points, phases and steps on their own or the opponent's turn,
  that auto-pass stops at, replacing the hard-coded conditions.
- A small UI toggles the stop points.
- `keep_passing`'s unit tests cover a custom stop set, including a stop on the opponent's turn.

**Pointers.** `crates/tui/src/shortcuts.rs` (`PassMode`, `keep_passing`), `crates/tui/src/app.rs`.

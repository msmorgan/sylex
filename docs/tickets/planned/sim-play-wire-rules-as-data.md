---
needs: [engine-planeswalkers]
---

# The headless simulator runs the rules-as-data rows

**Why.** `sim::play` builds its `GameConfig` with empty `sba_rules`, `conferral_rules`,
`damage_result_rules`, `counter_decls`, `subtypes` and `types`, so no rules-as-data fire in a
simulated game: a planeswalker takes no loyalty loss from damage and does not die at zero
loyalty, and type conferrals never apply. The TUI's game path loads them from the plugins
(`crates/tui/src/game.rs`). Any simulated game that touches those rules silently diverges from
the engine's real behavior.

**Done when.**
- `sim::play` loads every rules-as-data field of its `GameConfig` from the same plugins the TUI
  uses, through one shared loader rather than a copy of the TUI's code.
- The comment calling the empty rules acceptable is gone.
- A sim test with a real planeswalker shows damage removing loyalty and the planeswalker dying at
  zero loyalty.
- The matchups suite passes.

**Pointers.** `crates/engine/src/sim.rs` (`play`), `crates/tui/src/game.rs`.

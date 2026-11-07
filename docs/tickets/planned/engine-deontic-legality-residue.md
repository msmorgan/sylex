---
needs: []
---
**Engine: evaluate the deontic rows the legality checks still refuse, and guard every row they
ignore.**

## Why

A "can't" effect beats a permission [CR#101.2], and a player can begin casting only if no rule or
effect prohibits it [CR#601.3]. So a deontic row present in the derived view must either be
evaluated or stop the engine loudly. `guard_deontic_seam` (`crates/engine/src/legal.rs`) does the
loud stop, naming this ticket. Its three call sites leave these shapes unevaluated:

- `legal_actions`: `Cast` rows other than a slotless `Cant(Cast)` and the flash
  `May(Cast(window: InstantSpeed))`. That covers zone permissions (`from`), alternative costs
  (`cost`), `Must`/`Gate(Cast)`, and a slotted `Cant(Cast)`. Also `Play` rows other than the
  land-play marker `May(Play(from: None))`, `Must`/`Gate(Attach)`, and `May`/`Gate(Target)`.
- `legal_attackers` and `legal_blockers`: `Gate(Attack)` and `Gate(Block)`, the costs to attack
  or block.

The `legal_actions` guard's closure ends in `_ => false`, so rows over `Activate`, `Regenerate`,
`Counter` and `Untap` are never guarded there. Their `Cant` polarity is evaluated elsewhere in
`legal.rs` and in `step/mod.rs`, but a `May`, `Must` or `Gate` row over these actions is silently
ignored.

## Done when

- Every `DeonticAction` × polarity combination is either evaluated where its legality is decided or
  stops loudly. No catch-all arm drops a combination silently.
- Each combination that a card in `plugins/canon/` produces is evaluated, with a test grounded in
  that card. Combinations no card produces keep the loud guard.

## Not in scope

- Arbitration that maximizes obeyed requirements without violating restrictions.
- `May(Cast)`/`May(Play)` rows with a `from` zone: `engine-cast-from-zones`. `Cast` rows with a
  `cost`: `engine-alt-costs`.

## Pointers

- `crates/engine/src/legal.rs` (`guard_deontic_seam` and its callers)
- `crates/core/src/deontic.rs` (`Deontic`, `DeonticAction`)

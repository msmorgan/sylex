---
needs: [engine-planeswalkers]
---

# Choose planeswalker attack targets in the AI and the TUI

**Why.** The engine already carries an attack target per attacker: the declare-attackers decision
surfaces `legal_targets`, the defending player's proxy plus each planeswalker they control
[CR#506.3,508.1b]. But the AI strategies and the TUI always attack the defending player's proxy,
so no planeswalker is ever attacked. And the `Must(Attack)` requirement's `on` match still tests
only the defender's proxy (`legal.rs`), so "attacks if able" toward a specific planeswalker is not
enforced; that is unreachable today only because no consumer passes another target.

**Done when.**
- The AI strategies have a policy for when to attack a planeswalker instead of the player.
- The TUI's declare-attackers flow lets a human choose each attacker's target from
  `legal_targets`.
- The `Must(Attack)` match checks each attacker's chosen target, with an engine test grounded in a
  real card that a non-proxy target exercises.

**Pointers.** `crates/engine/src/decide/pending/combat.rs` (`DeclareAttackers`),
`crates/engine/src/legal.rs` (`legal_attack_targets`, the `Must(Attack)` match),
`crates/engine/src/strategy.rs`, `crates/tui/src/interact.rs`.

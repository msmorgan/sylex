---
needs: []
---
**Costs to attack and to block.**

## Why

Attack and block requirements (must attack, must block, goad) are evaluated at declaration, but a
`Gate(Attack)` or `Gate(Block)` row, a toll on attacking or blocking, is still a loud
`guard_deontic_seam` in `legal_attackers` and `legal_blockers`. Propaganda and Ghostly Prison
("Creatures can't attack you unless their controller pays {2} for each creature they control
that's attacking you") cannot be played.

## Done when

- After attackers are chosen, the active player chooses any optional attack costs, the total
  cost to attack is determined and locked in, they may activate mana abilities, and they pay it
  [CR#508.1g..508.1j]; the defending player does the same for blocks [CR#509.1d..509.1f]. A
  declaration whose cost is not paid is illegal.
- Requirement counting does not oblige a player to pay a cost to obey a requirement
  [CR#508.1d,509.1c].
- The seam guards no longer cover the `Gate` polarity of `Attack` and `Block`.
- Tests with Propaganda and Ghostly Prison, including an "attacks each combat if able" creature
  under Propaganda that need not attack, by core fixture where the cards are not authored.

## Pointers

- `crates/engine/src/legal.rs`: `legal_attackers`, `legal_blockers`, the `Must(Attack)` and
  `Must(Block)` row readers.
- `crates/engine/src/decide/pending/combat.rs`: declaration checks.
- `crates/core/src/deontic.rs`: `Deontic::Gate`.

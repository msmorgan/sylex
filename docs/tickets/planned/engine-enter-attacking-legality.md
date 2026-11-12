---
needs: []
---
# A permanent put onto the battlefield attacking is attacking only when it can be

## Why

A permanent an effect puts onto the battlefield attacking enters but is never an attacking
creature when it is a noncreature [CR#506.3a], when it enters under the control of a player other
than an attacking player [CR#506.3b] (outside a combat phase there is no attacking player
[CR#506.2]), when what it would attack is gone or no longer attackable [CR#506.3c,508.4a], or when
it is also a battle [CR#506.3f]. The zone-change apply in `crates/engine/src/step/mod.rs` declares
any entering object with an `EnterRider::Attacking` target an attacker through
`CombatState::declare_attacker`, checking only that the target still exists. A creature put onto
the battlefield attacking in a main phase therefore joins `CombatState` and sets its per-combat
record that a creature attacked [CR#508.8], which nothing clears until the next end of combat, so
that combat's declare blockers and combat damage steps are not skipped even when nothing attacks.
The tests `move_attacking_rider_declares_the_entering_creature_an_attacker` and
`move_attacking_rider_with_no_target_defaults_to_the_sole_opponent` in
`crates/engine/src/resolve/player_action.rs` pin the out-of-combat declaration.

## Done when

- An entering permanent becomes an attacking creature only when none of
  [CR#506.3a,506.3b,506.3c,506.3f,508.4a] applies; otherwise it enters and is never attacking,
  and `CombatState` is unchanged.
- Each case has a test grounded in a real card (Ninjutsu's "tapped and attacking" for the legal
  case), and the two pinned tests above assert the corrected behavior.

## Pointers

- `crates/engine/src/step/mod.rs`: the zone-change apply's `attacking_target`.
- `crates/engine/src/copy.rs`: where the `EnterRider::Attacking` target is resolved.
- `crates/engine/src/combat.rs`: `CombatState::declare_attacker`, `had_attackers`.

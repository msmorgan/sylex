---
needs: [engine-counters-api, engine-transform, engine-dfc-cast-faces, engine-loyalty-defense-zone-reads]
---
**Battles play in the engine: defense, the protector, attacking a battle, and a Siege's defeat.**

## Why

The engine has the battle card type and one battle rule: the defense-zero state-based action,
which today also moves a Siege whose defeat trigger is still waiting, against [CR#704.5v]. A
battle does not enter with defense counters, damage does not remove them, nothing chooses or
tracks a protector, and `legal_attack_targets` offers only the defending player and their
planeswalkers. The `.syl` battle and Siege definitions declare the full rule set the engine is
missing.

## Done when

- Damage dealt to a battle removes that many defense counters from it [CR#120.3h].
- As a battle enters, its controller chooses its protector: a Siege's from among their opponents
  [CR#310.9a,310.12a]. The protector is tracked per battle and survives the battle stopping
  being a battle or becoming a copy [CR#310.9g].
- A battle is a legal attack target for an attacking player whose defending player is its
  protector, and never for its protector [CR#310.9b,508.1b]; a Siege can be attacked by its own
  controller. Only its protector may block creatures attacking it [CR#310.9c], and "defending
  player" for those attackers means the protector [CR#310.9d,508.5].
- The protector state-based actions run [CR#704.5x,704.5y], and an attached battle becomes
  unattached [CR#310.10].
- The defense-zero action spares a Siege while one of its triggered abilities waits to leave the
  stack [CR#704.5v], and still moves a non-Siege [CR#704.5w].
- When a Siege's last defense counter is removed it is exiled and its controller may cast it
  transformed without paying its mana cost [CR#310.12b].
- Tests use real Sieges (e.g. Invasion of Zendikar // Awakened Skyclave), by core fixture where
  the cards are not authored.

## Not in scope

- The TUI/AI picker that chooses to attack a battle (`planeswalker-attack-target-ui`).

## Pointers

- `plugins/builtin/definitions/card_type/battle.syl`, `plugins/builtin/definitions/subtype/battle/siege.syl`:
  the declared battle and Siege rules.
- `crates/engine/src/legal.rs`: `legal_attack_targets`.
- `crates/engine/src/combat.rs`: `CombatState::attack_target`.
- `crates/engine/src/sba.rs`: `defense_zero_battle_is_put_into_graveyard`.
- `docs/tickets/planned/engine-dfc-cast-faces.md`: casting a back face.

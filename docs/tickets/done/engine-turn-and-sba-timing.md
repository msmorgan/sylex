---
needs: []
---
# Cleanup order, the no-attackers skip and the deathtouch window

## Why

Three turn-timing departures in the engine, all reachable with current cards.

**Cleanup.** The active player first discards to hand size [CR#514.1]; second, marked damage is
removed and "until end of turn" and "this turn" effects end, simultaneously [CR#514.2]. If any
state-based action is then performed or any trigger is waiting, they are performed and put on
the stack, the active player gets priority, and another cleanup step follows [CR#514.3a]. The
engine's Cleanup (`turn_based_actions` in `step/mod.rs`) clears damage and expires effects before
`CheckHandSize`, and `step_tail` gives Cleanup `CheckSbas` and `PlaceTriggers` with no
`OpenPriority` and no repeat; its comment says the sweep "can never act in the skeleton". It can:
Falkenrath Gorger grants madness, so a cleanup discard puts a madness trigger on the stack and the
turn moves on with no priority.

**No attackers.** If no creatures are declared as attackers or put onto the battlefield
attacking, the declare blockers and combat damage steps are skipped [CR#508.8]. `begin_step`
skips only the first-strike damage step; with no attackers, declare blockers and combat damage
still emit `StepBegan` and open priority, and only their turn-based actions are suppressed (keyed
on `combat.attackers().is_empty()`, which removal from combat also empties). The comment in
`legal.rs` that the declare blockers step "is reached only when an attacker exists" is false.

**Deathtouch.** A creature is destroyed if it has been dealt damage by a source with deathtouch
"since the last time state-based actions were checked" [CR#704.5h]. `DamageMark` (`object.rs`)
has no check epoch, and the lethal-damage SBA data reads `DealtDamageBy(This, Has(Deathtouch))`
over every mark, so a creature that survives one check (for example while indestructible) is
destroyed at a later check that turn. Damage stays marked even on a permanent that stops being a
creature [CR#120.6], so the same happens to one that becomes a creature again.

## Done when

- Cleanup discards first, then removes damage and ends the effects together; after the SBA check
  and trigger placement, if any SBA was performed or any trigger placed, the active player gets
  priority and another cleanup step begins. The stale comments are gone.
- A combat in which no creature was declared as an attacker or put onto the battlefield attacking
  goes from declare attackers to end of combat, with no `StepBegan` or priority for the skipped
  steps. The decision rests on a per-combat record of whether any creature was declared or put
  onto the battlefield attacking, not on the current attackers. The `legal.rs` comment is
  corrected.
- The deathtouch clause matches only damage dealt since the last state-based action check, while
  the [CR#704.5g] clause still counts all marked damage. The comment in `check_sbas` that the mark's
  lifetime is the window is corrected.
- Tests, with real cards:
  - a madness card discarded in cleanup with Falkenrath Gorger: its trigger goes on the stack, the
    active player gets priority, and a second cleanup step follows;
  - a "this turn" graveyard-to-exile replacement still applies to a cleanup discard;
  - a turn with no attackers has no declare blockers or combat damage `StepBegan`
    (`declare_blockers_skipped_when_no_attackers` in `tests/combat.rs` asserts this, not only the
    missing decision);
  - a creature dealt damage by Typhoid Rats while indestructible, which loses indestructible later
    that turn with toughness above its marked damage, survives. (Darksteel Myr under Humility
    does not test this: as a 1/1 with 1 damage marked it dies to [CR#704.5g] anyway.)

## Pointers

- `crates/engine/src/step/mod.rs`: `begin_step`, `turn_based_actions`, `step_tail`,
  `check_sbas`
- `crates/engine/src/object.rs` (`DamageMark`), `crates/engine/src/sba.rs`,
  `crates/engine/src/legal.rs`
- `v1/plugins/builtin/rules/sba/lethal-damage.ron` (the SBA data the engine loads)
- `crates/engine/tests/combat.rs`

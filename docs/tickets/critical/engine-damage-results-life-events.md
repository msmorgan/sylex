---
needs: [engine-lki-robustness]
---

# Engine: damage results are events, so damage-caused life loss and lifelink life gain are `LifeLost`/`LifeGained` events that triggers and replacements see

## Why

`impl EventApply for DamageDealt` (`crates/engine/src/step/combat.rs`, used for all damage, not
only combat damage) subtracts damage to a player straight from the life total and adds lifelink
life straight to the controller's. No `LifeLost` or `LifeGained` event exists for either, and
`EventFilter::LifeLost`/`LifeGained` match only those events. So "whenever an opponent loses
life" never sees damage, Ajani's Pridemate never sees lifelink, and Boon Reflection ("If you
would gain life, you gain twice that much life instead") never doubles it.

Infect and wither are modeled in the core the engine runs as replacements of the damage event
itself, so the damage is never dealt: damage-dealt triggers miss it and lifelink gains nothing.
Flensermite (infect and lifelink) gains no life. The -1/-1 and poison counters also become an
ordinary replacement the affected player can order against a damage doubler.

A departed source loses its keywords: the lifelink check reads the source only while it is still
in the object store, and `DamageMark.source_abilities` is empty when the source has left. So a
lifelink or deathtouch creature's "when this dies, it deals 2 damage" trigger neither gains life
nor destroys, contrary to [CR#702.15c,702.2e].

## Done when

- Damage is dealt first, then processed into its results [CR#120.4b,120.4c]. Each result is an
  event in the damage's simultaneous batch, routed through replacements and the trigger scan:
  damage to a player from a source without infect is `LifeLost { player, amount, cause }`, and
  each lifelink source makes its own `LifeGained { player, amount, cause }` [CR#702.15e], with
  `cause` the damage source [CR#119.9,119.10]. `DamageDealt::apply` no longer writes life totals.
- The lifelink payee is the source's controller, or its owner if it has no controller
  [CR#702.15b], read from the layered view (or the snapshot), not the base
  `GameObject.controller`.
- Wither and infect replace the marking or life-loss result, not the damage
  [CR#120.3b,120.3d,702.80a,702.90b,702.90c]; a source with both puts the -1/-1 counters once.
  The engine's `Instead(would: Damage …)` statics for them are gone.
- A departed source's lifelink, deathtouch, wither and infect come from its last-known
  information [CR#702.15c,702.2e,702.80b,702.90d], and `DamageMark.source_abilities` is filled
  from the same read, so it is never empty only because the source left.
- Tests:
  - combat damage to a player fires a "whenever an opponent loses life" trigger;
  - two lifelink creatures dealing combat damage at once trigger Ajani's Pridemate twice
    [CR#702.15e];
  - Boon Reflection doubles lifelink life gain;
  - a wither and lifelink creature deals damage: the recipient gets -1/-1 counters, its
    controller gains life, and a damage-dealt trigger fires (the example under [CR#120.4d]);
  - Flensermite deals combat damage to a player: the player gets poison counters, loses no
    life, and Flensermite's controller gains life;
  - a lifelink creature's dies trigger that deals 2 damage gains its controller 2 life, and a
    deathtouch one destroys the creature it damages.

## Pointers

- `crates/engine/src/step/combat.rs` (`DamageDealt::apply`)
- `crates/engine/src/eval.rs` (`EventFilter::LifeLost`/`LifeGained`)
- `crates/engine/src/replace_registry.rs` (how an `Instead` replacement removes the event)
- `crates/core/src/damage_result_rule.rs` (the existing data-driven damage-result rows)
- `plugins/builtin/definitions/keyword_ability/{infect,wither,lifelink}.syl`
- `engine-derived-controller-reads` for the payee read

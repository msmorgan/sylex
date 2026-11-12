---
needs: []
---
# A resolved effect's `This` names the source of the spell or ability that created it

## Why

Battle cry ("Whenever this creature attacks, each other attacking creature gets +1/+0 until end
of turn.", [CR#702.91a]) pumps the creature that has it. A resolved `Each(SelectAll(filter), …)`
keeps its filter floating in the layer pass, which matches `Ref(This)` against the effect's
carrier, and a resolved effect's carrier is its controller's player proxy, so `Not(Ref(This))`
excludes nothing. Accorder Paladin (3/1, battle cry) attacking beside Grizzly Bears ends up 4/1;
it should stay 3/1. Every loaded resolved `Each` filter that names `Ref(This)` has the same
defect.

## Done when

- A resolved effect's filter reads `This` as the source of the spell or ability that created the
  effect, as the resolution frame bound it, while the effect lasts.
- An engine test drives Accorder Paladin and Grizzly Bears into an attack through real turns:
  after battle cry resolves, Grizzly Bears is 3/2 and Accorder Paladin stays 3/1.

## Pointers

- `crates/engine/src/layer.rs`: `gather` (the registry loop's `watcher`), `static_reference_matches`.
- `crates/engine/src/resolve/effect.rs`: `Instruction::Continuously`, where the resolved scope is
  minted.
- `v1/plugins/builtin/macros/keyword/BattleCry.ron`.
- `crates/engine/tests/combat.rs`: `exalted_lone_attacker_pumps_via_that_object` drives an attack
  trigger through resolution.

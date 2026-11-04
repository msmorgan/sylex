---
needs: []
---
**An Aura's attach host is read positionally from target slot 0 through a
card-shape predicate, not from a declared register.**

## Why

Spell resolution attaches an entering Aura to "slot 0's single member" when
`enters_attached_self` recognizes the card's self-attach replacement. That is
an instruction reading outside its operands
([Engine](../../decisions/engine.md) law 3): any targeted
permanent spell whose attach host is not slot 0, or whose slot 0 is plural,
attaches to the wrong object.

## Done when

- The self-attach replacement's `Attach(This, to)` names its host as a register
  read (`Reference::Reg` of the region's `Provenance::AnnouncedTarget(n)`
  parameter), and the engine resolves the host through that ordinary read.
- The slot-0 lookup in spell resolution is gone, and `enters_attached_self`
  no longer chooses the host.
- A test attaches an Aura to its declared slot when that slot is not slot 0;
  the existing single-slot Auras (Enchant Creature canon) attach unchanged.
- `cargo test -p sylex_engine` and `cargo clippy -p sylex_engine --tests -- -D
  warnings` pass.

## Pointers

- `crates/engine/src/resolve/mod.rs`: the `enters_attached_self` /
  `entry.targets.first()` branch in spell resolution.
- `crates/engine/src/replace.rs`: `enters_attached_self`, and the `Attach`
  fold into `EnterStatus`.
- `crates/core/src/region.rs`: `Provenance::AnnouncedTarget`,
  `Region::reference_for`.

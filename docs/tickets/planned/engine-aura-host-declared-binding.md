---
needs: []
---
**An Aura spell's attach host is read positionally from target slot 0, not from a declared
register.**

## Why

Spell resolution attaches an entering Aura to "slot 0's single member" whenever
`enters_attached_self` recognizes the card's self-attach replacement. That instruction reads
outside its operands ([Engine](../../decisions/engine.md) law 3): a targeted permanent spell
whose attach host is not slot 0, or whose slot 0 holds several objects, attaches to the wrong
object.

## Done when

- The self-attach replacement's `Attach(This, to)` names its host as a register read
  (`Reference::Reg` of the region's `Provenance::AnnouncedTarget(n)` parameter), and the engine
  resolves the host through that ordinary read.
- The slot-0 lookup in spell resolution is gone, and `enters_attached_self` no longer chooses the
  host.
- A test attaches an Aura to its declared slot when that slot is not slot 0 (a minimal repro card
  derived from a named real Aura is fine); the existing Aura tests attach unchanged.

## Pointers

- `crates/engine/src/resolve/mod.rs`: the `enters_attached_self` / `entry.targets.first()` branch
  in spell resolution.
- `crates/engine/src/replace.rs`: `enters_attached_self`, and the `Attach` fold in
  `apply_as_enters`.
- `crates/core/src/region.rs`: `Provenance::AnnouncedTarget`, `Region::reference_for`.

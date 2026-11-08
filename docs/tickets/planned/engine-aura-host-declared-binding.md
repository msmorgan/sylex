---
needs: []
---
**A resolving Aura spell's host is read positionally from target slot 0, not from the target its
enchant ability declares.**

## Why

An Aura spell's target is defined by its enchant ability [CR#303.4a], and on resolution it enters
attached to what it was targeting [CR#608.3c]. Spell resolution instead attaches it to
`entry.targets.first()`'s single member whenever `enters_attached_self` recognizes the card's
self-attach replacement. That instruction reads outside its operands
([Engine](../../decisions/engine.md) law 3): a targeted permanent spell whose enchant target is
not slot 0, or whose slot 0 holds several objects, attaches to the wrong object.

## Done when

- Spell resolution takes a resolving Aura spell's host from the target slot its enchant ability
  declares, and the slot-0 lookup is gone.
- The self-attach replacement and its `Attach(This, to)` are unchanged, so an Aura entering by any
  other means still chooses its host as it enters [CR#303.4f]; `also_is_self_attach` still
  recognizes it.
- A test attaches an Aura spell to its declared slot when that slot is not slot 0 (a minimal
  repro card derived from a named real Aura is fine); the existing Aura tests attach unchanged.

## Not in scope

- Rewriting the self-attach replacement's `to` as a `Provenance::AnnouncedTarget(n)` register
  read: an Aura entering other than as a resolving spell has no announced target.

## Pointers

- `crates/engine/src/resolve/mod.rs`: the `enters_attached_self` / `entry.targets.first()` branch
  in spell resolution.
- `crates/engine/src/replace.rs`: `enters_attached_self`, `also_is_self_attach`, and the `Attach`
  fold in `apply_as_enters`.

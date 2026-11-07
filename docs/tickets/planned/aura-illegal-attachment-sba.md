---
needs: []
---

# The Aura state-based action covers an illegal host, not only no host

## Why

[CR#704.5m] puts an Aura into its owner's graveyard if it is attached to an illegal object or
player, or is not attached. `aura.syl` declares only the "not attached" half
(`not(isAttached(thisAura))`), so in `.syl` an illegally attached Aura is never graveyarded by
its own row. The engine still runs on the v1 Aura row, whose condition is
`Not(LegallyAttached(This))` and covers both halves, so pass 1 of `attachment_sbas` graveyards
it and pass 2 (the [CR#704.5n,704.5p] unattach cleanup) skips it. No engine test covers the
illegal-host case.

## Done when

- The `aura.syl` state-based row fires when the Aura is not attached or is attached to something
  that is not a legal host, reusing `legalHostOf`.
- An engine test with a real card (Pacifism on a permanent that stops being a creature) shows the
  Aura goes to its owner's graveyard in a single state-based check, with no `Unattached` event.

## Pointers

- `plugins/builtin/definitions/subtype/enchantment/aura.syl`
- `plugins/builtin/templates/predicate/legalHostOf.syl`, `plugins/builtin/templates/condition/isAttached.syl`
- `crates/engine/src/sba.rs`: `attachment_sbas` and its tests; `crates/engine/src/legal.rs`: `attachment_legal`.
- `v1/plugins/builtin/macros/types/enchantment/Aura.ron`: the row the engine runs today.

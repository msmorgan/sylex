---
needs: []
---

# Designation grants: single holders, eviction and object scope

## Why

`Action::GetDesignation` in core is the generic designation grant, but the engine handles only
the player-scope flag: it records the player and emits `GotDesignation`, and does nothing for a
designation only one player can hold or one that a permanent holds. Five mechanics need what is
missing:

- **Single holder with eviction**: only one player is the monarch [CR#725.3] or has the initiative
  [CR#726.3]; as a player gains it, the current holder loses it.
- **Object scope**: goaded [CR#701.15b] and suspected [CR#701.60b] are designations a permanent
  has. The engine's `DesignationStore.objects` already stores per-object instances, and
  `Designated(name)` filter reads are live against it, but nothing grants into it.
- **Both at once**: Ring-bearer is an object designation, one per player, that ends when another
  creature becomes that player's Ring-bearer or another player gains control of it [CR#701.54a].

Core's `DesignationDef::Stored { scope, shape, uniqueness, persistence, payload }` already
expresses each of these (object scope with `PerPlayer` uniqueness included); this is verb and
runtime work, not a taxonomy change.

## Done when

- Granting a `PerGame`-unique player designation moves it off the prior holder, with the event a
  trigger on losing it can see.
- Granting an object-scope designation adds an instance to `DesignationStore.objects`, with the
  grantor and duration the instance carries.
- Granting a `PerPlayer`-unique object designation evicts that player's prior holder, and a control
  change of the holder ends it.
- Engine tests, grounded in real cards (Archon of Coronation for the monarch; a goad, a suspect and
  a Ring-tempts-you card), cover each case.

## Not in scope

- The conferred abilities of these designations (the monarch's draw, goad's attack requirement)
  and the Ring's tempt progression (`macro-ring-emblem`).
- Where `.syl` declares uniqueness (`semantics-v3-designation-storage-columns`).

## Pointers

- `crates/core/src/action.rs` (`GetDesignation`), `crates/core/src/designation.rs`
- `crates/engine/src/resolve/player_action.rs` (the `GetDesignation` arm),
  `crates/engine/src/state.rs` (`DesignationStore`, `DesignationInstance`)
- `plugins/builtin/definitions/designation/` (`monarch.syl`, `initiative.syl`, `goaded.syl`,
  `suspected.syl`, `ringBearer.syl`)

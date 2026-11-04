---
needs: []
---
**An Aura attached to an illegal object or player takes two state-based-action
checks to reach the graveyard.**

## Why

[CR#704.5m] puts an Aura into its owner's graveyard if it is attached to an
illegal object or player, or is not attached. `aura.syl` declares only the
"not attached" half. An illegally attached Aura is first unattached by the
engine's generic unattach pass, then moved by the data row at the next check.
That pass also has no business touching an Aura: [CR#704.5n,704.5p] unattach
Equipment, Fortifications, battles, creatures and other permanents, never an
Aura.

## Done when

- The `aura.syl` state-based row covers both halves of [CR#704.5m]: not
  attached, or attached to something that is not a legal host, reusing
  `legalHostOf` from `enchant.syl`.
- The generic unattach pass in `attachment_sbas` skips Auras.
- A test shows an Aura attached to an illegal host goes to its owner's
  graveyard in a single check, with no `Unattached` event.
- `cargo test -p sylex_engine`, `cargo clippy -p sylex_engine --tests -- -D
  warnings` and `cargo xtask lean check` pass.

## Pointers

- `plugins/builtin/definitions/subtype/enchantment/aura.syl`
- `plugins/builtin/definitions/keyword_ability/enchant.syl`: `legalHostOf`.
- `crates/engine/src/sba.rs`: `attachment_sbas`.

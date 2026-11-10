---
needs: [cleanup-step6-surface]
specs: [confers-and-rules]
---

# The Role rule as `role`'s state-based conferral

## Why

"If a permanent has more than one Role controlled by the same player attached to it, each of
those Roles except the one with the most recent timestamp is put into its owner's graveyard"
[CR#704.5z,303.7a]. `plugins/builtin/definitions/subtype/enchantment/role.syl` states that rule
only in its header comment and has no `confers`, and no word reads an object's timestamp.

Written against the surface (`cleanup-step6-surface`).

## Design

- `role`'s `confers` writes the rule as a per-holder `stateBased` entry: a Role is put into its
  owner's graveyard if another Role controlled by the same player is attached to the same
  permanent and has a more recent timestamp (spec §6).
- It reads a new measure, `timestampOf(<object>)`, the object's timestamp; a Role is an Aura
  [CR#303.7] and gets a new one each time it becomes attached [CR#613.7e].

## Done when

- `role.syl` confers the rule, with [CR#704.5z,303.7a] cited, and its comment no longer states a
  rule the declaration does not write.
- `timestampOf` exists.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale, after `cargo xtask cite bless` for newly cited rules.

## Pointers

- `plugins/builtin/definitions/subtype/enchantment/role.syl`
- `plugins/builtin/definitions/subtype/enchantment/aura.syl` (a subtype's `stateBased` entry)
- `docs/specs/confers-and-rules.md` §6

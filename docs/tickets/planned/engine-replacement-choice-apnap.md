---
needs: []
---

# Engine: simultaneous replacement choices follow APNAP order, not member order

## Why

When two or more players must choose replacement effects at the same time, they choose in APNAP
order [CR#616.1,101.4], and a player with several choices at once picks their order
[CR#101.4c]. The replacement pass of one occurrence walks its members in member order and
surfaces each member's choice as it reaches it (`replace_members`, `resume_replacements` and
`surface_choice` in `crates/engine/src/replace_registry.rs`). So in a destroy-all over two
players' creatures that each hold two regeneration shields, the chooser order is the members'
order, which law 7 of `docs/decisions/engine.md` says no rule may read.

## Done when

- Within one occurrence, the active player makes every replacement choice they have before the
  nonactive players make theirs, in turn order [CR#101.4]; a choice that gives an earlier player
  a new choice restarts that order [CR#101.4d].
- A player with several members to choose for picks which member to choose for first
  [CR#101.4c].
- Test: a destroy-all whose first member is the nonactive player's creature with two
  regeneration shields and whose later member is the active player's creature with two shields
  surfaces the active player's choice first.

## Pointers

- `crates/engine/src/replace_registry.rs` (`replace_members`, `resume_replacements`,
  `surface_choice`)
- `crates/engine/src/state.rs` (`ReplaceState`)
- `crates/engine/tests/replace_registry.rs`
  (`destroy_all_over_two_double_shielded_creatures_stays_one_event`)

---
needs: []
---
# Share the Fight-subject resolution between the single and batched Fight paths

## Why

Two places resolve a fight's subjects with the same block: read both fighters off the body with
`sylex_core::fight_body_fighters`, collapse a creature fighting itself to one subject (it is one
creature dealing damage to itself [CR#701.14c]), and fizzle if a fighter is gone or the body's
guard fails [CR#701.14b]. One is the `"Fight"` arm of `composite_items` in
`crates/engine/src/resolve/action.rs`; the other is the `"Fight"` arm of `batch_act_head` in
`crates/engine/src/resolve/effect.rs`. They differ only in how they fizzle (`return vec![]`
against `return None`), so a change to one silently diverges from the other.

## Done when

- One `GameState` method returns the deduplicated, all-live subject list for a Fight body, or
  `None` to fizzle, and both arms call it.
- No behavior changes; the existing Fight tests in `resolve/action.rs` and `resolve/effect.rs`
  pass unchanged.

## Pointers

- `crates/engine/src/resolve/action.rs`: `composite_items`, the `"Fight"` arm.
- `crates/engine/src/resolve/effect.rs`: `batch_act_head`, the `"Fight"` arm.
- `crates/core/src/action.rs`: `fight_body_fighters`.

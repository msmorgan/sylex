---
needs: []
---
# Copying an activated ability whose source has left the battlefield makes no copy

## Why

An activated ability on the stack exists independently of its source
[CR#113.7a], and copying it puts a copy onto the stack [CR#707.10]. The
`Copied` apply instead returns nothing when the original's source object is
gone. Rings of Brighthearth copying a fetchland's ability, whose source was
sacrificed as a cost, gets no copy.

## Done when

- Copying an activated ability whose source has left creates the copy, with
  the original's announced decisions, and the copy reads its source through
  last known information [CR#113.7a].
- `copy_activated_ability_with_gone_source_fizzles` is rewritten to assert the
  copy exists and resolves.
- A test with Rings of Brighthearth copying Polluted Delta's ability passes, by core fixture
  where the cards are not authored.

## Pointers

- `crates/engine/src/step/stack.rs`: `impl EventApply for Copied`, the
  `StackObject::Activated` arm.
- `crates/engine/src/step/mod.rs`: `copy_activated_ability_with_gone_source_fizzles`.

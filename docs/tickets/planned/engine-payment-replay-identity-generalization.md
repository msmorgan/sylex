---
needs: [engine-payment-obligation-window]
---
# Stable replay identities for replacements, stack copies and emblems

## Why

Payment replay names objects by `LogicalObject` (`payment/replay.rs`): a card, a player, or an
object created by a payment record, with `GameState.payment_logical_objects` keyed by
`ObjectSource`. Three allocator families have no stable identity yet. A floating replacement
instance is recorded by its rollback-local key (`ReplacementKey::Floating` passes through replay
unchanged); a stack copy minted by `Copied` has no creation `ZoneChange` and may share its
`ObjectSource` with the original; an emblem is minted outside the card-zone identity path. A
replay in which an earlier record shifts one of these allocators can bind a later decision or
fact to the wrong object.

## Done when

- One generation-aware logical identity space covers replacement instances, stack copies and
  emblems as well as today's objects, without conflating coexisting objects that share an
  `ObjectSource`.
- Transcript decisions, fact comparison and reconstruction all use it.
- Reversal tests: for each of the three families, an earlier record shifts the allocator and a
  later retained record chooses or uses the newly allocated replacement, stack copy or emblem,
  and replay binds it correctly.

## Not in scope

- Keying stack copies by `ObjectSource` in the existing map: source identity is not instance
  identity.

## Pointers

- `crates/engine/src/payment/replay.rs` (`LogicalObject`, `ReplayMap`, decision rebinding)
- `crates/engine/src/state.rs` (`payment_logical_objects`)
- `crates/engine/src/step/stack.rs` (`Copied`)
- `crates/engine/src/replace_registry.rs` (`ReplacementKey`)
- `crates/engine/tests/payment_replay.rs`

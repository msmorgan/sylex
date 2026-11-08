---
needs: [engine-payment-obligation-window]
---
# Payment frames fork the game image cheaply instead of cloning it whole

## Why

Each `PaymentFrame` holds up to three full `GameImage` clones (`working`, `proposal_base`,
`payment_base`), and nested payments stack frames, so a filter-land or Krark-Clan Ironworks
payment copies every object, zone, history entry and the card table several times. Whether that
cost matters is not yet measured.

## Done when

- A benchmark covers nested filter-land and Krark-Clan Ironworks payments and selective
  rescind/decline replay, and its numbers on the full-clone implementation are recorded before
  any change.
- A forked frame keeps the whole runnable surface of a full clone: every engine query and
  mutation, replacements, triggers, layers, history, RNG, pending decisions, zone-change
  reminting and deterministic replay.
- The object store keeps `SlotMap`'s generational keys and allocator behavior: a stale
  `ObjectId` never resolves again, and never to a different object.
- Differential tests run the payment-window acceptance scenarios on the forked image and on
  full clones and require identical results.

## Pointers

- `crates/engine/src/state.rs`: `GameImage`. Its doc comment calls `cards` "fixed at game start,
  never mutated after"; the card table is append-only (`Cards::push_token` adds an entry per
  created token).
- `crates/engine/src/payment.rs`: `PaymentFrame`; `crates/engine/src/payment/replay.rs`.
- `crates/engine/src/object.rs`: `ObjectStore`, `Cards`.
- `crates/engine/tests/payment.rs`, `payment_mana.rs`, `payment_replay.rs`;
  `crates/engine/benches/full_game.rs`.

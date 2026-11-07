---
needs: []
---
# Make `Condition::Crossed` correct by construction

## Why

`Condition::Crossed` ("if the number of lore counters on it was less than N and became at least N",
[CR#714.2b]) reads the before/after counts that the firing counter-placement fact puts in the
frame. Only a Saga chapter's intervening-if condition supplies that channel, at both the printed
and the delayed trigger gate and again at resolution, and that path works. Every other
`condition_holds` caller builds a frame with no channel: an activated ability's "Activate only if"
gate (`activate.rs`), the state-based statics (`sba.rs`) and the `ForAsLongAs` sweep
(`state.rs`), and the `EventFilter::When` refinement when it falls back to a watcher frame
(`eval.rs`); the derived-condition pass in
`layer.rs` short-circuits `Crossed` to false. A `Crossed` that reached any of them would hit the
`todo!` in `condition.rs` at runtime with no earlier signal. No producer emits that shape today, so
this is hardening.

Two fixes close it, and the owner picks one:

1. **Restrict where it can appear.** Lowering (or core's types) admits `Crossed` only as a
   triggered ability's intervening-if condition, so any other position is refused before play.
2. **Make it self-sufficient.** `Crossed` reads the firing counter-placement fact from history, as
   `Condition::Happened` does, so it needs no frame channel. This also serves per-member firing
   when one doubled placement crosses two chapters of a range at once
   (`docs/tickets/planned/engine-sagas.md`).

## Done when

- The chosen fix is in, and the `todo!` in `Condition::Crossed`'s arm is unreachable by
  construction (option 1) or gone (option 2).
- A test shows the chosen behavior for a `Crossed` outside a chapter trigger: refused before play,
  or evaluated correctly.
- The existing Saga chapter tests pass unchanged.

## Pointers

- `crates/core/src/condition.rs` (`Crossed`), `crates/engine/src/condition.rs`
- `crates/engine/src/trigger.rs` (`scan_event`, `TriggerBindings`), `crates/engine/src/layer.rs`
- `plugins/builtin/primitives/event/chapterMark.syl`,
  `plugins/builtin/definitions/subtype/enchantment/saga.syl`

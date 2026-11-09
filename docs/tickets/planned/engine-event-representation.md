---
needs: []
---
# Make engine events cheap to move, and drop the `large_enum_variant` allows

## Why

`GameEvent` is about 288 bytes, and four engine enums carry one inline next to small variants, so
each silences `clippy::large_enum_variant` with an `#[allow]`: `WorkItem` (`agenda.rs`),
`Occurrence` (`event.rs`), and `StepOutcome` and `Progress` (`step/mod.rs`). The fix is to make
`GameEvent` cheap to move, consistent with "shared pointers are fine, clones are cheap".

Wrapping `GameEvent` in `Arc` only at one variant does not work. `Occurrence::Single(Arc<GameEvent>)`
beside `Batch(Vec<GameEvent>)` breaks every site that flattens an `Occurrence` into one
`Vec<GameEvent>`, and an `Arc` cannot be pattern-matched through, so every
`if let Single(GameEvent::…)` becomes bind-then-`match ev.as_ref()`. Making `Batch` hold
`Vec<Arc<GameEvent>>` too pushes `Arc<GameEvent>` through the whole event-producing pipeline
(combat, mana, cast, choice, every handler that builds a batch), with return-type changes cascading
to callers. The engine both produces and consumes events, so this needs one whole-pipeline choice
of representation, made before any code changes.

## Open questions

- The event representation: a uniform `Arc<GameEvent>` end to end, or a `Box`/`Arc` only at the
  enum boundary with `as_ref()` matching.

## Done when

- The chosen event representation is in, and stated in the doc comment on `GameEvent` or
  `Occurrence`.
- The `large_enum_variant` allows on `WorkItem`, `Occurrence`, `StepOutcome` and `Progress` are
  gone, with no new ones added.
- `cargo clippy --workspace --all-targets -- -D warnings` passes.

## Pointers

- `crates/engine/src/event.rs` (`Occurrence`, `GameEvent`, and the note above `Occurrence`)
- `crates/engine/src/agenda.rs` (`WorkItem`), `crates/engine/src/step/mod.rs` (`StepOutcome`,
  `Progress`)
- `crates/engine/src/replace_registry.rs` (`ReplaceOutcome::Pass`, `ResumeOutcome::Fact`: they
  also carry a `GameEvent` inline)

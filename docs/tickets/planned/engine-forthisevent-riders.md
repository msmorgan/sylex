---
needs: []
---
# Event-scoped riders beyond "can't be regenerated"

## Why

A `Duration::ForThisEvent` rider rides the instruction that carries it: Terror's "Destroy target
nonartifact, nonblack creature. It can't be regenerated." makes regeneration shields not apply to
that destruction [CR#701.19c]. `resolve_no_regen_riders` in `crates/engine/src/resolve/effect.rs`
wires only `Cant(Regenerate(on: <bare object Ref>))`. Three shapes trip a `todo!`: a rider part
that is not `Cant`, a `Cant` rider whose action is not `Regenerate`, and `Cant(Regenerate(on:))`
whose subject is not a bare object `Ref`.

## Done when

- The Vintage-legal card texts that carry an event-scoped rider are inventoried by shape, and
  each shape they need is wired, with a test grounded in a real card that has it. Wrath of God's
  "They can't be regenerated", whose subject is every destroyed creature, is one text to check.
- A shape no card needs keeps failing loudly, naming the shape.

## Pointers

- `crates/engine/src/resolve/effect.rs`: `resolve_no_regen_riders`, the `Sequentially`
  pre-scan for `ForThisEvent` riders, `WorkItem::InstallRiders`.
- `crates/core/src/continuous.rs`: `Duration::ForThisEvent`.
- `crates/engine/src/state.rs`: `duration_sweepable`, `no_regen_subjects`.

---
needs: []
---
# Prevention effects created by a resolving spell or ability

## Why

A spell or ability that creates a prevention effect for a duration, such as Fog ("Prevent all
combat damage that would be dealt this turn.") or Healing Salve's "Prevent the next 3 damage that
would be dealt to any target this turn.", resolves to `Continuously(Prevention(_))`, and the
`Instruction::Continuously` arm in `crates/engine/src/resolve/effect.rs` trips a `todo!` on it.
Prevention works today only as a static ability of a permanent: `prevention_watches` in
`crates/engine/src/replace_registry.rs` reads `StaticSpec::Prevention` off live objects. Such an
effect lasts until it is used up or its duration expires [CR#615.3], and a "next N damage" one
works like a shield [CR#615.7].

## Open questions

- The representation: reuse the floating `ReplacementInstance` shields in `shields` (which carry
  a duration but a single `subject` object, while Fog names no object), or add a prevention row
  kind on `ContinuousEffect.rows`.

## Done when

- A resolved `Continuously(Prevention(_))` creates a prevention effect that applies to damage
  events as they happen [CR#615.1] and ends when used up or when its duration expires [CR#615.3].
- `PreventNext` decrements per damage prevented and is used up at zero [CR#615.7].
- The stale comment on the `StaticSpec::Prevention` arm, which says the prevention macros wait
  on another ticket, is gone.
- Tests use real cards: Fog against a combat-damage step, and Healing Salve's prevention mode
  against a larger hit and across end of turn.

## Pointers

- `crates/engine/src/resolve/effect.rs`: the `Instruction::Continuously` arm, the
  `StaticSpec::Prevention` match.
- `crates/engine/src/replace_registry.rs`: `ReplacementInstance`, `prevention_watches`,
  `floating_watches`, the prevention tests.
- `crates/core/src/replacement.rs`: `Prevention`.
- Prevention effects [CR#615] in `data/rules/cr.txt`.

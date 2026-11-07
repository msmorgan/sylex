---
needs: []
---
# Generalize core's `UseLimit` to N times per window

## Why

`.syl` already states a use limit as `usageLimit(times, per)` (`oncePerTurn` and `oncePerGame`
are templates over it), but core's `UseLimit` has only `OncePerTurn` and `OncePerGame`. Lowering
therefore cannot carry "Activate no more than twice each turn" (Roterothopter, Vampire Bats). The
engine already enforces the two variants as `ability_used_count(object, index, window) >= 1`, so a
`{times, per}` form is a small step there.

The open question is where the limit lives. Kept as a field on the ability, it is checked against
the engine's history fact, which already keys on object and ability index together. Expressed
instead as a general condition (`EventCount` over `EventFilter::Used`), it cannot be right yet:
`Used { of: Reference }` is object-scoped, but an acquired restriction applies only to that ability
as acquired from that object, not to identically worded siblings [CR#602.5c], and core has no
way to address one ability.

## Done when

- The owner has chosen between an ability-attached `{times, per}` limit and an ability-addressable
  reference or filter.
- Core's `UseLimit` carries a count and a window in place of `OncePerTurn`/`OncePerGame`, and the
  engine checks `ability_used_count(...) >= times` at every site that enforces a limit today
  (activation legality, mana-ability payment, trigger firing).
- A test with Roterothopter shows the third activation in a turn is refused and that the count
  resets next turn.

## Not in scope

- `UseLimit::LoyaltyOncePerTurn`, which becomes an engine rule instead
  (`docs/tickets/planned/engine-loyalty-limit-intrinsic.md`).

## Pointers

- `crates/core/src/ability.rs` (`UseLimit`), `crates/core/src/event.rs` (`EventFilter::Used`)
- `crates/engine/src/resolve/count.rs` (`ability_used_count`), `crates/engine/src/activate.rs`,
  `crates/engine/src/payment/mana.rs`, `crates/engine/src/step/trigger.rs`
- `plugins/builtin/primitives/timing/usageLimit.syl`, `plugins/builtin/templates/timing/`

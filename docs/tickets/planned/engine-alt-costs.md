---
needs: [core-alt-costs]
---
# Choose and record alternative and optional costs as a spell is cast

## Why

Core can say that a cast permission carries an alternative cost (`DeonticAction::Cast { cost,
tag }`, `AlternativeCost::{Free, Components}`) and that a spell has optional additional costs, but
the engine never offers either at announcement [CR#601.2b] and keeps no record of which was
paid. So "without paying its mana cost", flashback, evoke and the like can't be cast through a
permission, and every reader of the record is a seam that names this ticket:

- `may_cast_rows` (`legal.rs`) collects a row's `cost` and `tag`, and `castable_cost_ignoring_mana`
  (`cast.rs`) skips rows that carry a cost.
- `Condition::CastWith` (`condition.rs`) answers a silent `false`, so Flashback's exile and
  Evoke's sacrifice never fire.
- `StatePredicate::WasPaidWith` and `WasCastWith` (`target.rs`) are `todo!`s.
- Entwine and escalate riders on a `Modal` return early without effect (`resolve/effect.rs`), and
  an optional-cost component other than mana or an action is a `todo!` (`cast.rs`).

## Done when

- At announcement the caster may choose one alternative cost from the permission rows that apply
  to the spell, only one per spell [CR#118.9a], and pays it rather than the mana cost [CR#118.9];
  `Free` pays no mana cost. Additional costs, increases and reductions still apply on top.
- The chosen alternative cost's tag and the paid optional costs are recorded on the spell and
  carried with it as it resolves; `CastWith`, `WasCastWith` and `WasPaidWith` read that record.
  None of them answers by default.
- Entwine and escalate add their costs during casting [CR#702.42a,702.120a], so the modal node
  neither returns early nor reaches a `todo!` for them.
- Tests on real cards: Think Twice cast by flashback from the graveyard and exiled as it
  leaves the stack [CR#702.34a]; Mulldrifter evoked and sacrificed [CR#702.74a]; a spell cast
  through a "without paying its mana cost" permission; Gatekeeper of Malakir kicked
  [CR#702.33d].

## Not in scope

An effect that casts a spell as it resolves (`Action::Cast` with its own alternative-cost slot,
madness) already works and is unchanged, except that it records its cost the same way.

## Pointers

- `crates/core/src/deontic.rs` (`AlternativeCost`, `DeonticAction::Cast`)
- `crates/engine/src/legal.rs` (`MayCastRow`, `may_cast_rows`), `crates/engine/src/cast.rs`,
  `crates/engine/src/condition.rs`, `crates/engine/src/target.rs`,
  `crates/engine/src/resolve/effect.rs`
- `plugins/builtin/definitions/keyword_ability/` (`flashback.syl`, `evoke.syl`, `kicker.syl`)

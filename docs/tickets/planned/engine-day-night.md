---
needs: [engine-transform, engine-trigger-text-survives-transform, engine-transform-current-face-name-loyalty]
---
# Engine: day/night, and daybound/nightbound

## Why

Day and night are designations the game itself holds. The game starts with neither, and once one
is held it always holds exactly one [CR#731.1]. About 50 Vintage-legal cards read or change day
and night; 35 of them have daybound. Core can already change the designation and watch it:
`Action::SetGameDesignation("DayNight", "Day")` writes `DesignationStore.game` as
`DesignationValue::Mode`, and the `EventFilter::DesignationChanged { name, to }` trigger filter
matches the change. The `.syl` side declares `day` and `night` (`plugins/builtin/definitions/designation/`),
`gameIs` and `gameBecomes`, and the `daybound` and `nightbound` keywords. The engine is missing
the rest:

- **A game-scope read.** `StatePredicate::Designated` evaluates only object and player
  designations (`crates/engine/src/target.rs`), so "if it's night" cannot be read.
- **The untap-step check** [CR#502.2,731.2]: if it's day and the previous turn's active player
  cast no spells, it becomes night. If it's night and they cast two or more, it becomes day. If
  it's neither, no check happens [CR#731.2c]. This is a turn-based action the engine performs.
- **Daybound and nightbound.** Their statics [CR#702.145b,702.145e]: a daybound permanent enters
  transformed at night; "as it becomes night/day", a permanent with the matching face up
  transforms; and it can't transform except through its own keyword. The keywords also have
  any-time rules that apply immediately and are not state-based actions: a front-face-up daybound
  permanent at night transforms [CR#702.145c]; a back-face-up nightbound permanent during the day
  transforms [CR#702.145f]; with a daybound permanent and neither designation, it becomes day
  [CR#702.145d]; with a nightbound permanent, neither designation and no daybound permanent, it
  becomes night [CR#702.145g].

These are the first cards that put back-face-up double-faced permanents into a real game, so the
two transform hardening tickets in `needs` must land first.

## Done when

- A game-scope designation read is evaluable, and "if it's day/night" conditions read the store.
- The untap step performs the [CR#502.2] check, binding the previous turn's active player and
  counting their spells. It never runs while the game has neither designation, and the store never
  returns to neither once set.
- Daybound and nightbound behave as their statics say, and the any-time rules run immediately
  whenever their conditions can become true: on a designation change, when a permanent enters, and
  when control changes.
- Tests grounded in a real daybound card (Brutal Cathar // Moonrage Brute) cover entering at night,
  the flip at day/night changes, the untap-step check after zero and after two spells, the
  neither-to-day rule when the first daybound permanent appears, and the can't-transform-otherwise
  restriction.

## Pointers

- `crates/core/src/action.rs` (`SetGameDesignation`), `crates/core/src/event.rs`
  (`DesignationChanged`)
- `crates/engine/src/state.rs` (`DesignationStore`, `DesignationValue`)
- `crates/engine/src/target.rs` (`StatePredicate::Designated`)
- `crates/engine/src/transform.rs`
- `plugins/builtin/definitions/designation/day.syl`, `night.syl`;
  `plugins/builtin/definitions/keyword_ability/daybound.syl`, `nightbound.syl`;
  `plugins/builtin/primitives/condition/gameIs.syl`, `plugins/builtin/templates/event/gameBecomes.syl`

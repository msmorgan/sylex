---
needs: [syl-event-disjunction]
---

# Joined trigger conditions are one disjunction of trigger conditions

## Why

`triggered` takes a first `TriggerHeader` and a list of joined ones, so "When this enchantment
enters and whenever you cast a spell with mana value 5 or greater" (*Up the Beanstalk*) is two
headers. A triggered ability may have more than one trigger condition [CR#603.1b], and each prints
its own trigger word, in any order and with any of "when", "whenever" and "at": "At the beginning
of your upkeep and whenever you cast a black spell" (*Shrine of Limitless Power*), "When Minsc &
Boo enters and at the beginning of your upkeep". A header list with one Template per joined
wording says that by enumeration, and a trigger word written over an event would stand wherever
an event does.

## Design

- A `TriggerCondition` sort. `triggerEvent(event: GameEvent)` is its one constructor from an
  event; the Templates `when(event)`, `whenever(event)` and `at(point)` (body
  `triggerEvent(reached($point))`) print the trigger words, so a trigger word stands only where a
  trigger condition does.
- Joined trigger conditions are one disjunction, in printed order: the primitive
  `or(conditions: List(TriggerCondition))` prints "or" (*Tenuous Truce*'s "… or when they attack
  you …"), and the Template `and(conditions)` prints "and", which nearly every joining card
  prints, with body `or($conditions)`. Each word is named for what it prints. Their list joins are
  `TODO(spelling)`.
- `triggered(condition: TriggerCondition, intervening = none, action, limit = none)`. The Ability
  Templates `when(event, action, …)`, `whenever(…)` and `at(point, action, …)` stay as the
  one-condition wording over `triggered(when($event), …)`; the same word at Ability and at
  `TriggerCondition` resolves by expected sort, and no coercion reaches both. A joined trigger is
  written `triggered(and([…]), action = …)`.
- A concurrent condition or timing window restricts one event: `timed(event, timing): GameEvent`,
  spelled "<event> <timing>", where the Timing prints its own "while" or "during". Each card prints
  the Timing right after its event, so the header's `while` and `timing` fields become `timed`
  and the `TriggerHeader` record is deleted.
- The look-back derivation reads an arm's event through the trigger-condition words and `timed`,
  and a `timed` arm's Timing is read at the same point as its event.

## Done when

- `TriggerHeader`, `joinedHead`, `joinedHeadWhile`, `atHead` and the `joins`, `while` and `during`
  parameters are gone; `TriggerCondition` with `triggerEvent`, `when`, `whenever`, `at`, `or` and
  `and`, and `timed` exist, and every caller in `plugins/` is respelled, keeping its card's
  printed order.
- A dsl_check test resolves "At … and whenever …" (*Shrine of Limitless Power*), "When … and at
  …" (*Minsc & Boo, Timeless Heroes*), "and when" (*Bartered Cow*) and "or when" (*Tenuous
  Truce*), and refuses an event written as a trigger condition without its trigger word.
- `docs/decisions/semantics.md` §4.6 states trigger conditions and their disjunction, and the
  Observation Point bullet reads through them and `timed`; the glossary defines Trigger
  Condition.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask` and
  `cargo clippy -p xtask --tests -- -D warnings` pass.

## Not in scope

- A referent bound in more than one arm of an event `or` and read after it.

## Pointers

`plugins/builtin/primitives/ability/triggered.syl`, `plugins/builtin/primitives/triggerCondition/`,
`plugins/builtin/templates/triggerCondition/`, `plugins/builtin/primitives/event/timed.syl`,
`plugins/builtin/templates/ability/{when,whenever,at}.syl`,
`crates/xtask/src/dsl_check/schema.rs` (`RECORDS`).

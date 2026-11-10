---
needs: [syl-single-face-card-call]
---

# Event disjunction, so `triggered` loses its extra slots

## Why

`.syl` has no "or" over events. Each place that needs one carries its own workaround:
`triggered` has eight parameters, among them `alternatives: List(GameEvent)` and
`joins: List(JoinedHeader)`; `delay` takes `alternatives` too; a would-clause uses `wouldAny`. A
triggered ability may have more than one trigger condition [CR#603.1b], and one construct should
say so everywhere an event stands.

Written against the start of step 6 (`syl-single-face-card-call`).

## Design

- An event disjunction is `or(events: List(GameEvent)): GameEvent`, beside the other per-sort
  `or`s. It stands wherever an event does (`when`, `whenever`, `at`, `delay`, `delayWithin`), so
  `triggered`'s `alternatives`, the trigger Templates' `or` and the `alternatives` of `delay` and
  `delayWithin` are deleted.
- The disjunction is over events, never over Actions: `does(actor(x), or([attack, block]))` is
  not written. "Blocks or becomes blocked" joins two events whose subjects differ
  (`does(actor(x), block)`, `does(any, becomeBlocked(x))`), *Syr Konrad, the Grim* joins three
  events with three subjects, and the Event Gate refuses a compound action as an event
  (`semantics.md` §4.6).
- Joined headers are not a disjunction. "When this creature enters and whenever it attacks while
  saddled" (*Autarch Mammoth*) prints a trigger word per header and carries a `while` on one
  header only. `triggered` takes its first header and a list of joined headers, each a
  `TriggerHeader` record `{event, while, timing}`; an event disjunction lives inside `event`.
  The first header is its own parameter because `.syl` has no list concatenation, so
  `when(e, joins = […])` cannot build one list of headers from its own header and its caller's.
  `JoinedHeader` is deleted.
- A header's trigger word is spelling, written by the Template that builds the header (`when`,
  `whenever`, `at`, `joinedHead`); the record carries none. A joined header may therefore be a
  turn-point header beside an event header ("When … enters and at the beginning of your upkeep",
  `syl-at-turn-point`).
- `triggered`'s parameters are in printed order: header, joins, intervening, action, limit.
- `wouldAny` had no caller and is deleted, not folded in: a would-clause reads before its event
  (`semantics.md` §4.5), so it is no event.
- The event `or`'s list formatting is a `TODO(spelling)`. Arms that share a subject print it
  once ("attacks or blocks"), even across two semantic roles ("blocks or becomes blocked", "dies
  or is exiled"), which a spelling string cannot say yet; no per-wording Template stands in.

## Done when

- `or` at `GameEvent` exists. `triggered`'s `alternatives`, the trigger Templates' `or`, the
  `alternatives` of `delay` and `delayWithin`, `JoinedHeader` and `wouldAny` are gone, with every
  caller in `plugins/` respelled.
- `triggered` takes a first `TriggerHeader`, a list of joined ones, the intervening condition, the
  action and the usage limit.
- Negative tests pin that a `TriggerHeader` without its event, or with the old `alternatives`
  field, is refused.
- `syl-at-turn-point`'s Done when says a joined header may be a turn-point header beside an event
  header.
- `docs/decisions/semantics.md` §4.6 states the event disjunction and joined headers.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Where the look-back point of each arm is decided (`syl-lookback-point-from-trigger`).
- A referent bound in more than one arm of an event `or`, read after it: *Chub Toad*'s "it gets
  +2/+2", *Aisling Leprechaun*'s "that creature".
- Reordering `when`, `whenever` and `at` to printed order (`syl-printed-read-order`).

## Pointers

`plugins/builtin/primitives/ability/triggered.syl`, `plugins/builtin/primitives/action/delay.syl`,
`plugins/builtin/templates/ability/{when,whenever,at,joinedHead,joinedHeadWhile}.syl`,
`plugins/builtin/templates/action/delayWithin.syl`, the `or.syl` primitives and Templates under
`plugins/builtin/`, `crates/xtask/src/dsl_check/schema.rs` (`RECORDS`).

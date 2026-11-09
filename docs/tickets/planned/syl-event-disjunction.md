---
needs: []
---

# Event disjunction, so `triggered` loses its extra slots

## Why

`.syl` has no "or" over events. Each place that needs one carries its own workaround:
`triggered` has eight parameters, among them `alternatives: List(GameEvent)` and
`joins: List(JoinedHeader)`; `delay` takes `alternatives` too; a would-clause uses `wouldAny`. A
triggered ability may have more than one trigger condition [CR#603.1b], and one construct should
say so everywhere an event stands.

## Open questions

- The shape of an event disjunction: an `or` at `GameEvent` beside the other `or`s, or something
  else; and whether `joins` and `wouldAny` fold into it or stay distinct.

## Done when

- An event disjunction exists, and `triggered`'s `alternatives`, `delay`'s `alternatives` and
  whatever else the settled shape covers are gone, with every caller in `plugins/` respelled.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/ability/triggered.syl`, `plugins/builtin/primitives/action/delay.syl`,
`plugins/builtin/primitives/event/wouldAny.syl`, the `or.syl` primitives and Templates under
`plugins/builtin/`.

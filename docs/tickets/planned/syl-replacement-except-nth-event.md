---
needs: [lean-retirement]
---

# A replacement that excepts the nth event

## Why

*Notion Thief*: "If an opponent would draw a card except the first one they draw in each
of their draw steps, instead that player skips that draw and you draw a card." A would-clause can
select the nth event in a period (`would/nthOccurrence.syl`) but cannot exclude it, so the card
cannot be written.

## Done when

- A would-clause can exclude the nth occurrence in a period, either as its own primitive or as a
  general "except" on an event pattern; the landing says which and why.
- *Notion Thief* is in `plugins/canon/cards/`.
- That the first draw in an opponent's draw step is not replaced and a second draw in that step
  is, is not checkable until the engine reads `.syl`.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/would/nthOccurrence.syl`,
`plugins/builtin/primitives/event/nthOccurrence.syl`, `docs/decisions/semantics.md` §4.5.

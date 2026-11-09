---
needs: []
---

# Each concept modeled one way

## Why

Some concepts have two or more `.syl` models side by side: "X'd this way" in five forms, copying a
spell, coordination, and "whose". Two models of one concept give two readings of one printed
phrase, and a card's meaning depends on which its author picked.

## Done when

- For each of the four, the landing names its forms, and one model remains, the others deleted or
  made Templates over it, with every caller in `plugins/` updated; a form kept separate states why
  in its doc comment.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

The numeric readbacks ("that much", "that many"): `syl-numeric-anaphors`.

## Pointers

`plugins/builtin/primitives/timing/thisWay.syl` and its callers; `docs/decisions/semantics.md`
§1 item 5(b).

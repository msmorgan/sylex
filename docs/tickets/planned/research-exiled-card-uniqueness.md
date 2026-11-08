---
needs: []
---

# Research: is "the exiled card" always one card?

## Why

A Participial Description ("the exiled card", "the card exiled with this artifact")
requires the unique referent the deed stamped, and reaches objects a linked ability published
[CR#607.2a] (`docs/decisions/semantics.md` §2.4). Whether every such read refers to exactly one
card, or a linked exile can stamp several, decides how reference resolution treats it.

## Done when

- Every Vintage-legal card whose text reads "the exiled card" or "the card exiled with …" is
  listed with its Oracle text, the quantity its exiling ability can exile (one card, or possibly
  several), and the rule that links the read to the exile.
- The cards where the exiling ability can exile more than one card while the read is singular
  are called out.
- The findings are reported to the owner as input to the reference-resolution design.

## Pointers

`data/mtgjson/AllPrintings.sqlite`, `docs/decisions/semantics.md` §2.4.

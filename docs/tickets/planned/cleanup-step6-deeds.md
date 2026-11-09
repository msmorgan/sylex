---
needs: [syl-fight-both-performers, syl-one-exile-declaration, syl-ring-tempts-you-raw-scope, syl-single-use-as-though]
---

# Step 6 middle: Deeds

## Why

This stage makes Deeds explicit in `.syl` and rewrites the declarations and cards that name one.
It runs in parallel with the other two middle stages. Later stages write against it.

## Done when

- Every ticket in its `needs:` is in `done/`.
- A checkpoint pass over the `action` declarations under
  `plugins/builtin/definitions/keyword_action/` and the keyword abilities that perform a Deed
  finds Deed handling still shaped by Lean: a Deed named or matched by a `deed = "…"` label string,
  one keyword action declared once per printed variant under a shared label, a bodiless primitive
  standing in for a Deed, a scope written by hand, or a doc comment that names a generated
  wrapper. Each one found is fixed in the checkpoint's own landing or ticketed, and the landing
  lists them.

## Pointers

- Keyword actions that share a Deed label with another declaration, outside `exile` and
  `exchange`: `double{Power,Toughness,PowerAndToughness,LifeTotal,Counters}` (`"double"`),
  `triple{Power,Toughness,PowerAndToughness}` (`"triple"`), and `castWithoutPaying`, `millCards`,
  `revealHand`, `searchZonesOf`, `shuffleInto` and `voteStartingWith` beside `cast`, `mill`,
  `reveal`, `search`, `shuffle` and `vote`, all in `plugins/builtin/definitions/keyword_action/`.
- `plugins/builtin/definitions/keyword_action/amass.syl`'s doc comment, written in Lean syntax
  around `ownScope`.

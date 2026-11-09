---
needs: []
---

# Write `it` wherever the card prints "it"

## Why

About 99 mentions in `plugins/canon/cards/` (re-count when claimed) are written as a
self-reference such as `thisCreature` or `thisEnchantment` where the Oracle text prints "it" for
the card itself: *Megatherium*'s "sacrifice it unless you pay …" is
`sacrifice(thisCreature)`; *Charnel Troll* and *Blood Spatter Analysis* do the same. The author
resolved the pronoun by hand, so it never reaches discourse and can't be checked, which breaks
surface-faithfulness (`docs/decisions/semantics.md` §1 item 1).

## Done when

- Every canon mention where the Oracle text prints "it" is written `it`
  (`plugins/builtin/templates/pronoun/it.syl`), and every self-reference left is printed as one
  ("this creature", the card's name).
- A one-off script, kept in `docs/evidence/`, lists every self-reference in `plugins/canon/cards/`
  beside the Oracle text it stands for (`data/mtgjson/AllPrintings.sqlite`), and the landing
  reports the count changed and any site where the printed word is ambiguous.
- Whether each `it` resolves to the card is not checkable until reference resolution exists.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

Replacing the `this<Head>` Templates with one Template: `syl-this-head-template`.

## Pointers

`plugins/builtin/templates/noun/this*.syl`, `plugins/builtin/templates/pronoun/it.syl`,
`plugins/canon/cards/` (*Megatherium*, *Charnel Troll*, *Blood Spatter Analysis*),
`docs/decisions/semantics.md` §1 item 1.

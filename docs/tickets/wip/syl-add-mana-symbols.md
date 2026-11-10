---
needs: [syl-single-face-card-call]
---

# "Add {G}" without an unlabeled list of lists

## Why

Mana a card adds by its symbols is written `addMana(amount, manaOf(runs))`, where
`manaOf(runs: List(List(ColorOrColorless)))` (`plugins/builtin/primitives/mana/manaOf.syl`) gives
the two list levels meanings nothing names: the outer list is "or", each inner list a run of
symbols. Every "Add {G}" is `addMana(lit(1), manaOf([[colored(green)]]))`; *Adarkar Unicorn*'s
"Add {U} or {C}{U}" is `manaOf([[colored(blue)], [colorless, colored(blue)]])`, and *Coal
Stoker*'s "add {R}{R}{R}" a run of three under `lit(1)`, while *The Flux*'s "Add six {R}" is
`lit(6)` over a run of one. A reader can't tell from the call which list is the choice and which
the sequence, the elements are colors rather than the mana symbols the card prints, and the
amount multiplies whichever run is chosen. `manaOf` is also spelled `"Add <runs>"`, though it is
an argument of `addMana` and the verb belongs to the action; the other `ProducedMana` primitives
(`ofAnyColor`, `ofTheChosenColor`, …) describe only the mana. `manaOf` is used by 32 canon cards and
`firebending.syl`.

## Open questions

- The shape of symbol-written mana production: how a run of symbols, a choice between runs and a
  counted amount ("six {R}", "{G} for each creature you control") are each written and named,
  and whether a run is written in mana symbols.
- Which declaration spells "Add": `addMana`, or the `ProducedMana` word inside it.

## Done when

- Symbol-written mana production is written in the settled shape: no position whose meaning
  comes from list nesting alone, and "Add" spelled by the declaration the settled answer names;
  every caller in `plugins/` is updated.
- The rewritten cards mean the same mana: the landing lists each card's old and new call.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/mana/manaOf.syl`, `plugins/builtin/primitives/action/addMana.syl`,
`plugins/builtin/primitives/mana/` (the other `ProducedMana` primitives),
`plugins/canon/cards/{Adarkar Unicorn,Coal Stoker,The Flux,Gaea's Cradle}.syl`.

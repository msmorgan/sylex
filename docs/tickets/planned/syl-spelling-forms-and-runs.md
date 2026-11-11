---
needs: [syl-spelling-conventions]
---

# Write printed forms, run Holes and sentence joins into the corpus

## Why

`docs/decisions/spelling.md` §7 states a Hole's printed form (`$name@Form`), a Hole a run of
sentences may fill (`$name*`) and a list whose elements are each a run (`<.>`), and no spelling
under `plugins/` writes any of them. Without them a numeric Hole prints in no stated notation
("draw $amount cards" admits "draw 2 cards" and "draw a card" alike), `does` and `did` have the
same spelling, and every `Action` Hole reads as one phrase, so no trigger's wording holds the
several sentences its effect prints. About 90 declarations under `plugins/` have a numeric
parameter and a `spelling`, and about 40 an `Action` parameter and a `spelling` (re-count when
claimed). Oracle prints "one" where `Counting` prints the article (*Melira, the Living Cure*'s
"you get one poison counter"), and no declaration says it.

## Done when

- Every Hole whose head word's form neither agreement nor a governing word fixes carries the
  `@Form` §7 gives it: each numeric Hole its slot's notation, and each ungoverned `Action` Hole
  its verb form (`does` `@Present`, `did` `@Preterite`), each checked against the Oracle text of
  a card that calls the declaration. No Hole carries a label its position forces, and `lit` and
  `plus` leave theirs unlabeled. The landing lists every label with the card line that fixes it.
- A Hole that counts objects is `@Counting` where the cards print 1 as the article or as no word,
  and the landing names a card line for each of §7's three places that the corpus reaches.
- The Template `one` at `Amount`, spelled "one", exists, and *Melira, the Living Cure* or
  *Noxious Vapors* is authored and calls it.
- Every Hole a run of sentences may fill is `$name*`, and no other Hole is: `when`, `whenever`,
  `at`, the delayed and reflexive triggers, `replaceEvent`'s `replacement`, `instead`'s and
  `activatedOnlyDuring`'s `action`, and any other the run rule (`spelling.md` §2) covers. The
  landing lists each, with a card whose text fills it with more than one sentence or the reason
  none does.
- A list Hole whose elements each print as a run is written with the `<.>` join and no `*`; the
  landing lists them.
- The corpus means the same before and after: a one-off script, kept in `docs/evidence/`, checks
  that every declaration's tree is unchanged apart from its `spelling`, comments and the added
  `one` Template and card, and the landing reports the declaration count it compared.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

A mark for passive subject promotion or for Factoring (`spelling.md` §10). The ordinal of
`fromTop` and `fromBottom`, which `syl-spelling-conventions` writes `$n@Ordinal`.

## Pointers

`docs/decisions/spelling.md` §2 and §7, `crates/lexicon/src/types.rs` (`Numeral`, `WordForm`),
`plugins/builtin/primitives/quantity/lit.syl`, `plugins/builtin/templates/quantity/plus.syl`,
`plugins/builtin/primitives/action/sequentially.syl`.

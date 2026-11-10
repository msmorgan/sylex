---
needs: [cleanup-step6-spine]
---

# One sort for a power/toughness change

## Why

"Gets +P/+T" takes two separate `Delta(Amount)` arguments, so nothing stands for the change as
a whole. *Johtull Wurm* prints "gets -2/-1 until end of turn for each creature blocking it
beyond the first": one "for each" scaling both numbers, which an `Amount`-level `forEach` can
only write twice. *Mutagen Connoisseur*'s `getsPt(thisCreature, up(forEach(1, …)), up(0))` for
"+1/+0 for each" recovers a second way, as a flat 1 beside a scaled 0. `getsBoth` goes the
other way and writes one delta for the two numbers of "+N/+N", which the text prints as two.

## Design

- A power/toughness change is one value of its own sort, built from its two deltas by one
  primitive, and `get`, `getsPt` and what they expand to take it whole
  (`docs/decisions/spelling.md` §3.2).
- `forEach` is declared at that sort: it takes a change and a Predicate and scales the whole
  change.
- A bare "+N/+N" is two printed values and is written as two. `getsBoth` is deleted at `Action`
  and at `StaticSpec`, and its callers write both deltas.

## Open questions

- The sort's name and its word. The glossary has **Delta** for one number's change and nothing
  for the pair. Recommendation: sort `PtChange`, glossary entry "Power/Toughness Change", built
  by `pt(power, toughness)`, matching the `Pt` already in `getsPt` and `definesPt`.
- Whether "has base power and toughness 3/3" (`getsBase`, two `set` deltas) takes the same
  sort. It prints no sign and is not a change the text scales. Recommendation: leave `getsBase`
  with its two amounts, and decide separately whether `set` stays a `Delta`.

## Done when

- The glossary has the sort's entry, and `docs/decisions/syl.md` §2 no longer gives `getsBoth`'s
  one delta as its example of a value written once.
- `get`, `getsPt` and the keyword bodies that call them (`battleCry`, `bushido`, `exalted`,
  `flanking`, `frenzy`, `melee`, `prowess`, `rampage`, the doubling and tripling keyword
  actions, the Role tokens) take the change; `getsBoth` is gone. The landing reports how many
  calls in `plugins/canon/` it rewrote (126 files call one of the three today).
- *Johtull Wurm* is in `plugins/canon/cards/` with one `forEach`, and every "+P/+T for each"
  in `plugins/` scales the whole change (*Adelbert Steiner*, *Mutagen Connoisseur*, *Nissa,
  Ascended Animist*, *Coat of Arms*, `rampage`).
- The rewritten corpus means the same: a one-off script, kept in `docs/evidence/`, checks that
  every rewritten call carries the same subject, deltas and duration before and after.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Where the "for each" Tail stands relative to a duration: `syl-tail-placement`.

## Pointers

`docs/decisions/spelling.md` §3.2; `plugins/builtin/templates/action/{get,getsBoth}.syl`,
`plugins/builtin/templates/ability/{getsPt,getsBoth,getsBase}.syl`,
`plugins/builtin/templates/amount/forEach.syl`; glossary **Delta**
(`docs/contexts/game-model/CONTEXT.md`), **Measure Noun Phrase**
(`docs/contexts/oracle-english/CONTEXT.md`).

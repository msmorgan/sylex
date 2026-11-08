---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---

# Which cost a turn-face-up spend purpose pays

**Why.** The spend purpose `toTurnFaceUp(subject: Predicate)` says only that the mana is spent to
turn a permanent `subject` describes face up, a special action [CR#116.2b]; it does not say which
cost the mana pays. A manifested card is turned face up by paying its mana cost [CR#701.40b], or,
if it has morph or disguise, by paying that cost instead [CR#701.40c,701.40d]. *Qarsi Deceiver*
prints "pay a mana cost to turn a manifested creature face up", so its mana cannot pay a manifested
card's disguise cost, but written with `toTurnFaceUp` alone it can. No spend purpose or named cost
says "its mana cost".

**Done when.**
- `toTurnFaceUp` takes an optional cost qualifier naming the permanent's mana cost, as opposed to
  a morph or disguise cost.
- *Qarsi Deceiver* is in `plugins/canon/cards/`, its second purpose written with the mana-cost
  qualifier, and a test shows that purpose names the mana cost.
- *Overgrown Zealot* ("to turn permanents face up") stays unqualified.
- Everything passes the `.syl` checks.

**Pointers.** `plugins/builtin/primitives/mana/toTurnFaceUp.syl`,
`plugins/canon/cards/Overgrown Zealot.syl`.

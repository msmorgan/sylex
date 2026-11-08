---
needs: [lean-retirement]
---

# Both creatures fight: `fight`'s Deed has two performers

## Why

When one creature fights another, both fight: "Each of those creatures deals damage
equal to its power to the other creature" [CR#701.14a], and if either is gone, "neither of them
fights" [CR#701.14b]. `keyword_action/fight.syl` declares `permanent = performs`, so its Deed
names one performer, the actor, and the creature it fights is only a patient of the body. A
trigger or lookback on the fighting creature ("whenever this creature fights", "if it fought")
then sees only the actor's side. `fightEachOther.syl` shares the `fight` Deed with the same one
performer.

## Done when

- The `fight` Deed names both creatures as performers, whether through a Deed header that names
  two performers or by writing `fight` over `fightEachOther`'s pair; the landing says which.
- Events and lookbacks match a Deed's performers, so "Whenever this creature fights" triggers both
  when this creature fights another and when another creature fights it, and "fought this way"
  reads back both creatures after `fight` and after `fightEachOther`.
- Reference resolution is unchanged: a clause after the fight still takes the fighting creature
  as its subject, so "that creature" after "this creature fights target creature" is the target
  (*Faunsbane Troll*, *Mawloc*).
- A creature fighting itself still takes twice its power [CR#701.14c].
- Each of these has a test once events and reference resolution are checkable.

## Pointers

`plugins/builtin/definitions/keyword_action/{fight,fightEachOther}.syl`,
`docs/decisions/semantics.md` §3.1.

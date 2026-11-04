---
needs: [lean-retirement]
---
# Both creatures fight: `fight`'s Deed has two performers

Standard constraints apply. Done in `.syl` and Rust (`deckmaste_semantics_v3`,
`deckmaste_discourse`) after `lean-retirement`.

When one creature fights another, both creatures fight: "Each of those creatures deals damage
equal to its power to the other creature" [CR#701.14a], and "If one or both creatures instructed
to fight are no longer on the battlefield or are no longer creatures, neither of them fights"
[CR#701.14b]. `plugins/builtin/definitions/keyword_action/fight.syl` declares
`permanent = performs`, so the `fight` Deed names one performer, the actor; the creature it
fights only receives the Deed's stamp as a patient of the body (`semantics.md`, "The header's
deed is the action's one deed"). A trigger or lookback on the fighting creature ("whenever this
creature fights", "if it fought") then sees only the actor's side. Not probed in Lean's event
matching; confirm before changing.

## Change

- Give the `fight` Deed both creatures as performers: either a Deed header that names two
  performers, or `fight` written over `fightEachOther`'s pair (`fightEachOther.syl` already says
  "<pair> fight each other" with the same Deed).
- A Deed's performers stay what events and lookbacks match. Reference resolution does not
  change: a clause after the fight still ranks the fighting creature as its own clause's subject,
  so "that creature" after "this creature fights target creature" is the target (*Faunsbane
  Troll*, *Mawloc*).

## Tests

- "Whenever this creature fights" triggers both when this creature fights another creature and
  when another creature fights this creature.
- "Fought this way" reads back both creatures after `fight` and after `fightEachOther`.
- A creature fighting itself still takes twice its power [CR#701.14c].

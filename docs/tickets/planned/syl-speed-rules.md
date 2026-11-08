---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---

# Speed: the start your engines! state-based action and the speed trigger

## Why

Start your engines! has an empty body (`startYourEngines.syl`): its rule is a state-based
action, "If a player controls a permanent with start your engines! and that player has no speed,
their speed becomes 1" [CR#702.179a,704.5aa], which belongs in a rules row under
`plugins/builtin/rules/sba/`. The row cannot be written yet. No Action sets or raises a player's
speed (`speedOf` only reads it), and "has no speed" is not "speed 0": players have no speed until
a rule or effect sets it [CR#702.179b], and 0 is only what an effect reading speed sees
[CR#702.179f].

## Done when

- An Action sets a player's speed and one raises it by an amount; raising from no speed sets it to
  that amount [CR#702.179c].
- A Condition tests whether a player has a speed, distinct from comparing `speedOf` with 0.
- An `sba` rule row writes [CR#704.5aa], and `startYourEngines` is read by it.
- The inherent triggered ability of every player with speed 1 or more is written as a rules row:
  sourceless and controlled by that player [CR#702.179d], the way designation definitions write
  theirs.
- Everything passes the `.syl` checks.

## Not in scope

The engine's speed state (`engine-speed`).

## Pointers

`plugins/builtin/definitions/keyword_ability/{startYourEngines,maxSpeed}.syl`,
`plugins/builtin/templates/quantity/speedOf.syl`,
`plugins/builtin/rules/sba/deathtouchDestruction.syl` (a rule row),
`plugins/builtin/definitions/designation/monarch.syl`.

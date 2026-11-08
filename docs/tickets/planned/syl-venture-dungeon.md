---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---

# Venture into the dungeon, dungeon cards and the initiative's venture triggers

## Why

38 Vintage-legal cards venture into the dungeon, and the initiative has two inherent
triggers that venture into Undercity [CR#726.2]. `.syl` has no venture action and no dungeon card,
so `designation/initiative.syl` carries only its third ability.

## Done when

- A dungeon card is a card frame of its own: rooms, each a room ability, joined by arrows
  [CR#309.4,309.4c]. `Card` has it beside the other frames, with the checks the frame needs.
- Each room ability's trigger is "When you move your venture marker into this room" [CR#309.4c].
- The venture marker's position is engine state. `.syl` reads it through Condition atoms (the
  marker is on the bottommost room, or in a room with a given arrow), and never simulates it
  [CR#309.5a,309.5b,701.49b,701.49c].
- `ventureIntoTheDungeon` and its variant "venture into [quality]" are keyword-action Definitions
  under `plugins/builtin/definitions/keyword_action/` [CR#701.49a..701.49d]. The dungeon card comes
  from outside the game, described with `outsideTheGame` [CR#701.49a,400.11], and is put into the
  command zone [CR#309.2b].
- The bottommost-room state-based action is a rules row [CR#309.6], and completing a dungeon is
  that card's removal from the game [CR#309.7].
- `initiative.syl` carries all three inherent abilities [CR#726.2], and its comment about the
  missing two is gone.
- Everything passes the `.syl` checks, with at least one venturing card in canon.

## Not in scope

The engine's dungeon state (`engine-dungeons`).

## Pointers

`plugins/builtin/definitions/designation/initiative.syl`,
`plugins/builtin/templates/predicate/outsideTheGame.syl`, `docs/decisions/semantics.md` §1 items 4
and 5.

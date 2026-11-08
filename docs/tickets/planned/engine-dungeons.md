---
needs: []
---
# Dungeons and venture into the dungeon in the engine

## Why

Venture into the dungeon [CR#701.49], the initiative [CR#726.2] and "if you've completed a
dungeon" all need engine state that does not exist: no venture-marker position, no dungeon cards
outside the game, no room abilities, no record of completed dungeons. `Type::Dungeon` exists in
core and `plugins/builtin/definitions/card_type/dungeon.syl` declares the card type, but
nothing in `crates/engine` models a dungeon. 68 Vintage-legal cards in the MTGJSON snapshot
mention venturing, the initiative or a dungeon.

## Done when

- Venture has no mechanic-specific engine action: it resolves as a named `Action::Composite`
  whose body composes the dungeon primitives, the way other keyword actions resolve through
  `composite_items`.
- A dungeon card is per-card data: its rooms joined by arrows, a directed graph with one topmost
  and one bottommost room [CR#309.4,309.5a].
- Each player owns a pool of dungeon cards outside the game. A dungeon card leaves the command
  zone only by leaving the game [CR#309.2c].
- Each player has one venture-marker position (none, or a room of the dungeon card they own in
  the command zone [CR#309.3,309.4]), and venturing follows [CR#701.49a..701.49d]: with no
  dungeon in the command zone, the player chooses one they own from outside the game, puts it
  there and the marker on its topmost room [CR#309.4a]; otherwise the marker moves along an arrow
  of the player's choice [CR#309.5a]; from the bottommost room the dungeon is removed and a new
  one is chosen [CR#309.5b].
- An effect other than venturing that would bring a dungeon card into the game does nothing
  [CR#309.2d].
- Moving the marker into a room triggers that room's ability, controlled by the dungeon's owner
  [CR#309.4c].
- The bottommost-room state-based action removes the dungeon unless one of its room abilities is
  still on the stack [CR#309.6].
- Completing a dungeon is removing it from the game [CR#309.7], whether by venturing from the
  bottommost room or by the state-based action. It is an event a trigger watches (Dungeon
  Crawler) and is recorded in history, so a condition reads it like any other history fact
  (Gloom Stalker; Acererak the Archlich reads which dungeon).
- Tests use real cards: Nadaar, Selfless Paladin venturing through one complete dungeon, and a
  choice at a branching arrow.

## Not in scope

- The `.syl` venture action, the dungeon card frame and the initiative's venture triggers:
  `syl-venture-dungeon`.

## Pointers

- `crates/core/src/type.rs`: `Type::Dungeon`.
- `crates/engine/src/resolve/action.rs`: `composite_items`.
- `crates/engine/src/history.rs`.
- `plugins/builtin/definitions/designation/initiative.syl`,
  `plugins/builtin/definitions/subtype/dungeon/undercity.syl`.
- Dungeons [CR#309] and venture into the dungeon [CR#701.49] in `data/rules/cr.txt`.

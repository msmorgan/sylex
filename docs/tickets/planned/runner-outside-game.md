---
needs: []
---

# Cards outside the game: sideboards for companion, learn and wishes

**Why.** Several mechanics reach cards a player owns outside the game [CR#400.11]: companion
reveals one before the game begins [CR#702.139a], learn may put a Lesson into hand
[CR#701.48a], venturing brings in a dungeon card [CR#701.49a], and wish-style effects fetch a
card. The engine has no pool of outside-the-game cards and the runner no point at which a player
chooses from it.

**Done when.**
- A game is set up with each player's outside-the-game cards (their sideboard), which the engine
  keeps apart from every zone.
- The runner exposes a decision point where a player chooses from those cards, and the engine
  moves the chosen card into the game as the effect says.
- Engine tests grounded in real cards cover learn putting a Lesson into hand and a wish effect;
  companion's pregame reveal and dungeons are covered here or by their own tickets
  (`engine-pregame-procedure`, `engine-dungeons`).

**Pointers.** `crates/engine/src/runner.rs`, `crates/engine/src/state.rs`.

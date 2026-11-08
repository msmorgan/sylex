---
needs: []
---
# The pre-game procedure

## Why

`GameState::new` (`crates/engine/src/state.rs`) picks the starting player from the config
(fixed or seeded random), shuffles and draws opening hands with no mulligans and no
opening-hand actions. The `PreGame` decision (`crates/engine/src/decide/pending/choice.rs`) is
never surfaced and its `resolve` is a `todo!`. `PreGameKind` already names `FirstTurn`,
`Mulligan`, `Companion` and `OpeningHand`. Starting a game is [CR#103]: who goes first
[CR#103.1], the mulligan loop with London bottoming [CR#103.5], and opening-hand actions such as
a Leyline beginning on the battlefield [CR#103.6a].

## Done when

- The owner has chosen whether the simulator needs the interactive procedure (surfaced `PreGame`
  decisions) or a deterministic seeded opener, and that choice is built.
- If interactive: `PreGame::resolve` applies first-turn, mulligan (with bottoming) and
  opening-hand answers, and the `todo!` is gone. Tests cover a mulligan to six with one card
  bottomed and Leyline of the Void starting on the battlefield.

## Not in scope

- Companion reveal and the outside-the-game card pool (`syl-companion-pregame-rule`,
  `runner-outside-game`).

## Pointers

- `crates/engine/src/state.rs` (`GameState::new`, `StartingPlayer`)
- `crates/engine/src/decide/mod.rs` (`PreGameKind`), `crates/engine/src/decide/pending/choice.rs`

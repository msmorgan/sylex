---
needs: [engine-face-down, engine-look-grants-lifecycle]
---

# A per-player redacted view of the game

## Why

The engine is full-information, and its consumers (the demo strategies, the TUI) read
hidden zones and printed faces straight from `GameState`, so an AI can cheat and a client could
leak hidden cards. A per-player view that redacts what the viewer may not see makes hidden
information real without changing the engine's model.

## Design

- `PlayerView` is a borrowed, stateless read facade over `GameState` in the engine crate, beside
  the layer view; nothing is copied.
- Visibility is decided by zone, not by whether the viewer owns or controls the object: off the
  battlefield and the stack an object's controller is its owner [CR#108.4a], so an owns-or-controls
  test would show a player their own library.
- A `face(id)` chokepoint, separate from the zone check, returns the known face or "hidden".
- A read of what the viewer can't see collapses to a count: an opponent's hand is a count, and
  library order is unknown beyond any granted top cards.

## Done when

- `PlayerView` exists in `sylex_engine`, with `can_see` and `face` accessors.
- `can_see` follows the zone rules: every object in a public zone is visible [CR#400.2], subject to
  `face`; a hand is visible to its player [CR#402.3], except a card drawn face down while a spell
  is being cast, hidden until that spell becomes cast [CR#121.8]; a library is visible to no one
  [CR#401.2], except a card a live effect or an open look window in `look_grants` shows
  [CR#401.5,701.20e,701.23a].
- `face` shows a face-down spell or permanent's face to its controller [CR#708.5] and to no other
  player without a grant. A face-down exiled card is visible to no one, its owner included, except
  under the separate [CR#406.3] ledger, not `look_grants`.
- The demo strategies and the TUI's hidden-zone and printed-face reads go through it.
- Tests cover: own versus opponent hand; public zones; a granted scry top card, then cleared by a
  shuffle; a reveal to all; an owner can't see their own library; an owner can't see a card an
  opponent exiled face down; a face-down permanent seen by its controller, an opponent and a
  grantee; the owner of a face-down permanent an opponent controls can't see its face.

## Not in scope

Changes to core, the RNG or zone storage. If a consumer needs non-cheating play
before `engine-face-down` lands, the zone half (`can_see`) may be built ahead of `face`.

## Pointers

`crates/engine/src/state.rs` (`look_grants`), `crates/engine/src/strategy.rs`,
`crates/tui/src/`.

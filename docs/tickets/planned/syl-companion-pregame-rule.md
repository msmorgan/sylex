---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement, cleanup-step6-surface]
---
# Companion's pregame reveal as a start-of-game rule

## Why

"Before the game begins, you may reveal one card you own from outside the game with a companion
ability whose condition is fulfilled by your starting deck" [CR#702.139a,103.2b]. That reveal is a
start-of-game rule, so it is a rule declaration (`semantics.md` §1 item 4), not part of the
keyword's in-game body. `companion.syl` writes only the in-game special action [CR#116.2g], and
gates it on nothing but the card being outside the game, though only "a player who has chosen a
companion" may take it [CR#116.2g]. No rule declaration under `plugins/builtin/rules/` covers the
start of the game.

Written against the surface (`cleanup-step6-surface`).

## Design

- The reveal is a top-level rules row scoped to players (`scope = isPlayer`), enabled once per
  player, as `playerDamageLifeLoss` is; each player reveals at most one card
  [CR#103.2b].

## Done when

- A rule declaration writes the reveal: at most one card [CR#103.2b], its condition read through
  the keyword's `DeckCondition` against the starting deck after sideboard cards are set aside
  [CR#702.139b], and a mark that the card was revealed this way.
- `companion.syl`'s special action is gated on that mark, and its header comment no longer points
  at this ticket.
- A Vintage-legal companion card (*Jegantha, the Wellspring* or *Lurrus of the Dream-Den*) is in
  `plugins/canon/cards/`.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

The order of the start-of-game procedure [CR#103] is the engine's.

## Pointers

- `plugins/builtin/definitions/keyword_ability/companion.syl`
- `plugins/builtin/primitives/condition/` (the `DeckCondition` primitives)
- `plugins/builtin/rules/` (the existing `sba` and `conferral` rule declarations)
- `docs/decisions/semantics.md` §1 item 4; `docs/decisions/syl.md` §1

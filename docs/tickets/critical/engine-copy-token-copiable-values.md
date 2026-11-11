---
needs: []
---

# Engine: a token made as a copy keeps the copied mana cost and loyalty, so its color and mana value come from that cost

## Why

A copy acquires the original's copiable values, mana cost and loyalty among them [CR#707.2],
and a token has a mana cost when the effect that creates it says so [CR#202.1b]. The engine
drops both: `token_from_copiable` (`crates/engine/src/copy.rs`) builds a `Token`, which has no
mana-cost or loyalty slot, and `push_token` (`crates/engine/src/object.rs`) mints every token
with an empty mana cost and no loyalty. An Offspring token of Agate Instigator ({1}{R}) is
therefore colorless with mana value 0; Populate and every other token-copy path are affected
the same way. Cackling Counterpart's ruling ("If the copied creature has {X} in its mana cost,
X is considered to be 0") presupposes the cost carries over.

Base color is also computed wrongly: `base_colors` (`crates/engine/src/layer.rs`) uses the
color indicator only when the mana cost has no colors, but an object is the colors of its mana
cost [CR#202.2] and also each color its color indicator denotes [CR#202.2e].

Embalm and Eternalize copy "except … it has no mana cost" [CR#702.128a,702.129a]. Today they
come out right only because the cost is always dropped, with their color set through the color
indicator.

## Done when

- A token created as a copy has the copied mana cost and loyalty; its color and mana value
  derive from that cost.
- `base_colors` is the union of the mana cost's colors and the color indicator's.
- Core can say "it has no mana cost" as a copy exception. Embalm and Eternalize use it, and their
  color exception wins over the color the copied cost would give.
- The docs that say a token never has a mana cost are corrected: `token_from_copiable`'s
  doc in `copy.rs`, and the `Token` doc in `crates/core/src/token.rs` (which also cites
  [CR#202.2e] for that claim; the rule is [CR#202.1b]).
- Tests: an Offspring token of Agate Instigator is red with mana value 2; a copy of a creature
  with {X} in its cost has mana value with X as 0; an embalmed token is white with mana value 0.

## Not in scope

- Defense, which [CR#707.2] also lists among the copiable values.

## Pointers

- `crates/engine/src/copy.rs` (`token_from_copiable`, `apply_modification`)
- `crates/engine/src/object.rs` (`push_token`)
- `crates/engine/src/layer.rs` (`base_colors`)
- `crates/core/src/token.rs` (`Token`), `crates/core/src/continuous.rs` (`Modification`)
- `plugins/builtin/definitions/keyword_ability/{embalm,eternalize,offspring}.syl`
- `crates/engine/src/resolve/player_action.rs` (the existing Embalm/Eternalize/Offspring
  token-copy tests)

---
needs: [engine-copy-token-copiable-values]
---
# A copy exception that sets a copy's color wins over the color its kept mana cost would give

## Why

A copy exception that modifies a characteristic makes the final value part of the copiable values
[CR#707.9b], so Croaking Counterpart's "except it's a 1/1 green Frog" on a copy of Agate
Instigator ({1}{R}) makes a green token that keeps the mana cost {1}{R} [CR#707.2]. The engine
spells an object's base color only as its mana cost's colors united with its color indicator's
[CR#202.2,202.2e], and a `Colors` exception folds into the indicator alone, so that token would be
red and green. `apply_exceptions` (`crates/engine/src/copy.rs`) crashes on any copy whose
`Colors` exception sets or removes a color its kept mana cost still gives
(`assert_color_spellable`). Embalm and Eternalize do not reach it: their "it has no mana cost"
leaves no cost colors.

## Done when

- A copy can carry a color that a `Colors` exception set or removed, apart from its mana cost,
  through copiable values, the minted token's face and every reader of an object's base colors
  (`layer::base_colors` and the readers built on it); its mana cost and mana value still come
  from the kept cost.
- `assert_color_spellable` is gone.
- Tests, each failing today: Croaking Counterpart copying Agate Instigator is a green 1/1 Frog
  with mana value 2; Benthic Anomaly's copy ("it's a colorless Eldrazi creature") of Grizzly Bears
  ({1}{G}) is colorless with mana value 2.
- Regression guard, passing today: a copy of a red creature "except it's blue in addition to its
  other colors" stays red and blue.
- Core fixtures where the cards are not authored.

## Pointers

- `crates/engine/src/copy.rs`: `apply_exceptions`, `assert_color_spellable`, `apply_modification`.
- `crates/engine/src/layer.rs`: `base_colors`, `base_values`.
- `crates/engine/src/object.rs`: `push_token`.
- `crates/core/src/copy.rs` (`CopiableValues`), `crates/core/src/token.rs` (`Token`),
  `crates/card/src/card.rs` (`Characteristics`).

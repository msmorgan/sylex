---
needs: [engine-payment-obligation-window]
---
# Plural color-union mana production (Mox Amber)

## Why

`ManaSpec::AmongColorsOf` (`crates/core/src/mana.rs`) takes one `Reference`, and its engine arm
(`resolve/player_action.rs`) reads the colors of one object. Mox Amber ("{T}: Add one mana of any
color among legendary creatures and planeswalkers you control") needs the union of colors across
every qualifying permanent, offered as one color choice with no prompt to pick a permanent. The
Mox Amber fixture in `crates/engine/tests/payment_replay.rs` works around this with
`Reference::Single(Selection::SelectAll(..))`, which is exact only while at most one permanent
qualifies.

## Done when

- A mana spec can name a plural color source and produces one mana of any color in the union of
  the colors of every object it selects, in a deterministic color order. It produces no mana when
  the set or its color union is empty.
- Tests with zero, one and several qualifying permanents, including overlapping and distinct
  colors.
- The `payment_replay.rs` Mox Amber fixture uses the plural form.

## Pointers

- `crates/core/src/mana.rs` (`ManaSpec`), `crates/engine/src/resolve/player_action.rs`
  (`AmongColorsOf` arm)
- `crates/engine/tests/payment_replay.rs` (Mox Amber fixture)
- `plugins/builtin/primitives/mana/ofAnyColorOf.syl`

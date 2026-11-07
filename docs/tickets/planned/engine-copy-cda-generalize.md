---
needs: [core-copy-grammar, engine-layers-1-copy-facedown-text]
---
**A copy exception drops the copied object's characteristic-defining ability for every
characteristic it provides or retains, as part of deriving the copiable values.**

## Why

When a copy exception provides or retains a characteristic, any characteristic-defining ability
of the copied object that defines it is not copied, and for color neither is the color indicator,
unless the exception adds types "in addition to its other types" [CR#707.9d]. The engine
implements only the power/toughness case, by a shape-matching filter (`defines_pt`) applied
after the exception: `retain_characteristic` is a no-op for colors and subtypes, and the
`Modify` path drops nothing for them. Devoid [CR#702.114a] and changeling [CR#702.73a] are
characteristic-defining abilities [CR#604.3a], so Croaking Counterpart ("except it's a 1/1 green
Frog") copying Barrage Tyrant makes a colorless Frog instead of a green one, and copying
Avian Changeling keeps changeling (latent while `AllCreatureTypes` is a no-op in the layer pass).

## Done when

- The drop happens inside layer-1 copiable-value derivation for every characteristic
  [CR#707.9d] names (power, toughness, colors with the color indicator, subtypes),
  honoring the "in addition to its other types" exception; `defines_pt` and the per-axis special
  case in `retain_characteristic` are gone.
- Power and toughness behave as now: Quicksilver Gargantuan copying Tarmogoyf is 7/7 with no
  Tarmogoyf CDA, and a copy of that copy stays 7/7 [CR#707.3].
- Tests: Croaking Counterpart copying Barrage Tyrant is a green Frog; Croaking Counterpart
  copying Avian Changeling has no changeling. Core fixtures where the cards are not authored.

## Pointers

- `crates/engine/src/copy.rs`: the module doc's [CR#707.9d] paragraph, the `Modification` apply,
  `defines_pt`, `retain_characteristic`.
- `crates/engine/src/layer.rs`: `base_values`, the `AllCreatureTypes` arm, `devoid_derives_colorless`.
- `docs/tickets/planned/engine-cda-authorable-statics.md`: the core mark that identifies a
  characteristic-defining ability.

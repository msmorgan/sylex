---
needs: []
---
# The remaining no-op layer arms: every creature type, basic land types, and base loyalty/defense

## Why

Three `apply_static` arms in `crates/engine/src/layer.rs` do nothing. `AllCreatureTypes`
(changeling, "This object is every creature type" [CR#702.73a] over the open creature-type list
[CR#205.3m]) leaves a changeling with only its printed subtypes. `BecomeBasicLandType` skips what
[CR#305.7] requires when an effect sets a land's subtype to basic land types: the land loses its
old land types and the abilities from its rules text and gains the new types' mana abilities
(Blood Moon). `BaseLoyalty`/`BaseDefense` have no layer at all (`layer_of` returns `None`), though
loyalty and defense are characteristics [CR#109.3]; `crates/engine/src/copy.rs` already applies
them as copy exceptions (Jace, Mirror Mage; Ob Nixilis, the Adversary).

## Done when

- A changeling permanent derives every creature type, in layer 4 [CR#613.1d]. The test
  `changeling_layers_does_not_panic`, which asserts the fill is absent (`subtypes.len() < 50`),
  is rewritten to assert it.
- Blood Moon makes a nonbasic land a Mountain with only the Mountain mana ability [CR#305.7],
  without removing abilities other effects granted it.
- `BaseLoyalty`/`BaseDefense` either apply in the layer pass or are removed from the layer arms,
  depending on whether a Vintage-legal card needs them outside a copy effect.

## Not in scope

- Marking a static as a characteristic-defining ability so it applies first within its layer
  [CR#613.3]; that is `engine-cda-authorable-statics`.

## Pointers

- `crates/engine/src/layer.rs`: `layer_of`, `apply_static`, the `changeling_layers_does_not_panic`
  test.
- `crates/core/src/continuous.rs`: `Modification`.
- `plugins/canon/cards/Blood Moon.syl` (spelled as a subtype set, not a separate op);
  `v1/plugins/builtin/macros/keyword/Changeling.ron`.

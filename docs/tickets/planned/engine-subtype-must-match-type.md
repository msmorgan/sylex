---
needs: []
---
# Layer 4 adds only subtypes that match one of the object's types

"An object can't gain a subtype that doesn't correspond to one of that object's types"
[CR#205.3d]. The engine's layer-4 subtype change (`Modification::Subtypes` in
`crates/deckmaste_engine/src/layer.rs`, around line 1895) adds or sets any subtype whatever the
object's types: a "becomes a Sliver" effect on a noncreature artifact gives it a creature subtype it
cannot have. `idris-retirement` carried the citation to `primitives/static/typeLine.syl`, which
states the rule without anything enforcing it.

## Change

- In the layer-4 subtype arm, `Add` and `Set` keep a subtype only if one of its correlated types
  (`Subtype::types` [CR#205.3c]) is among the object's types after layer 4's type changes, and
  cite [CR#205.3d] on that check. Types and subtypes changed by one effect in layer 4 apply
  together, so "becomes an artifact creature Golem" keeps Golem: check against the types the
  object has once the effect's type change has applied, not before.
- A subtype absent from the registry has no correlated types (`resolve_subtype`'s name-only
  fallback), so the check cannot judge it. Decide whether such a subtype is refused, kept, or made
  impossible (every subtype registered); record the choice.
- Tests in `deckmaste_engine`: a creature subtype added to a noncreature is not gained; one added
  together with the creature type is; `Set` drops the subtypes that do not match; a subtype
  matching any one of several types is kept.

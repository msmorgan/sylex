---
needs: []
---
# A filter can test whether its candidate shares a creature type with another object

## Why

Coat of Arms ("Each creature gets +1/+1 for each other creature on the battlefield that shares at
least one creature type with it."), Alpha Status, Stoneforge Masterwork, Heirloom Blade, Shared
Animosity, Tajuru Paragon and Titan of Littjara relate two objects by their creature types
[CR#205.3m]. Core has no predicate for that relation. A sharing test written as a finite `Or`
over values, the way `SharesColor` is spelled out over the five colors, does not fit: the
creature types are a long list that grows with new sets, and a changeling has every one of them
[CR#702.73a], which the layer pass derives, not the printed card. In a distributed static the
"it" of the relation is the object being modified: a captured candidate in the count's filter
region, which the layer matcher reads through the count environment.

## Done when

- Core has a predicate relating a filter's candidate to a referenced object by shared creature
  type, read from layered subtypes in the layer pass and in `target::matches`.
- Coat of Arms is a fixture in an engine test: two Goblin Warriors and a Goblin Shaman are each
  +2/+2 (the card's own reminder text), a creature sharing no type is unchanged, and a changeling
  shares a type with each of them.
- Stoneforge Masterwork or Alpha Status shows the same relation with the host (`AttachHostOf`) as
  the other object.

## Pointers

- `crates/core/src/filter.rs`: `Predicate`, `RelationPredicate`, `Where`.
- `crates/engine/src/layer.rs`: `matches_derived`, `static_reference_matches`, `read_register`.
- `crates/engine/src/target.rs`: `where_shares_color_with_carrier` (the finite spelling).
- `crates/engine/tests/layers.rs`: `heroes_podium()`, a count filter excluding the modified object.

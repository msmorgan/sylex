---
needs: []
---

# A card-type noun names a permanent: "target artifact" can't target an artifact card in a hand

## Why

A description of an object that names a card type without a zone, "card" or "spell" means a
permanent of that type on the battlefield [CR#109.2]. The v1 pipeline's type-noun vocabulary
(`TYPE_NOUN_ATOMS` in `v1/crates/pipeline/src/parsers/filter.rs`) gives creature, planeswalker and
battle a battlefield-scoped filter macro (`Creature` is `PermanentOfType(Creature)`). Artifact,
enchantment and land get a bare `Type(<T>)`, which matches a card of that type in any zone. Target
candidates are every object that matches the filter (`target::candidates_region_with_activation`),
so canon's Collective Resistance ("Destroy target artifact", "Destroy target enchantment",
`TargetOne(Type(Artifact))`) offers artifact and enchantment cards in hands, libraries and
graveyards.

Affinity has the same gap. Its macro counts `And([Param(0), ControlledBy(Ref(You))])`, and
`ControlledBy` reads every object's `controller`, so "affinity for artifacts" also counts artifact
cards in its controller's hand.

## Done when

- "artifact", "enchantment" and "land" in an object description lower to a battlefield-scoped
  filter, like "creature". A description with a zone or "card" keeps its own zone [CR#109.2a].
- Collective Resistance's artifact and enchantment modes are corrected in canon. Their target
  menus are exactly the matching permanents on the battlefield, on a board that also has a
  matching card in a hand and a graveyard.
- Affinity counts only the permanents its controller controls. A test with a real affinity card
  shows that an artifact card in hand doesn't reduce the cost.

## Not in scope

- Enchant's quality, which the Enchant macro already scopes to permanents.

## Pointers

- `v1/crates/pipeline/src/parsers/filter.rs` (`TYPE_NOUN_ATOMS`, `head_noun`)
- `v1/plugins/canon/cards/Collective Resistance.ron`
- `v1/plugins/builtin/macros/keyword/Affinity.ron`, `v1/plugins/builtin/macros/filter/PermanentOfType.ron`

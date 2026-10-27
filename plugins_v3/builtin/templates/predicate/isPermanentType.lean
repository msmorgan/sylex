/-- "has a permanent type": an artifact, battle, creature, enchantment, land, or planeswalker
[CR#110.4]; read off the types, so it holds of a card or a token. -/
template isPermanentType : Predicate := {

  -- TODO(spelling): artifact, battle, creature, enchantment, land, or planeswalker; this predicate
  -- needs contextual noun coordination, not one invariant modifier.
  body :=
    Semantics.Templates.Predicate.or
      [Semantics.Templates.Predicate.hasType Semantics.CardType.artifact, Semantics.Templates.Predicate.hasType Semantics.CardType.battle,
        Semantics.Templates.Predicate.hasType Semantics.CardType.creature, Semantics.Templates.Predicate.hasType Semantics.CardType.enchantment,
        Semantics.Templates.Predicate.hasType Semantics.CardType.land, Semantics.Templates.Predicate.hasType Semantics.CardType.planeswalker]
}

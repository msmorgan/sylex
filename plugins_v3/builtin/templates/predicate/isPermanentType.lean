/-- "has a permanent type": an artifact, battle, creature, enchantment, land, or planeswalker
[CR#110.4]; read off the types, so it holds of a card or a token. -/
template isPermanentType : Predicate := {

  -- TODO(spelling): Select and coordinate the applicable permanent-type nouns within the enclosing
  -- description.
  body :=
    or
      [hasType Semantics.CardType.artifact, hasType Semantics.CardType.battle,
        hasType Semantics.CardType.creature, hasType Semantics.CardType.enchantment,
        hasType Semantics.CardType.land, hasType Semantics.CardType.planeswalker]
}

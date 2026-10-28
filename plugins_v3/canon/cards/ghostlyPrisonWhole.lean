/-- Ghostly Prison -/
card := {
  name := "Ghostly Prison",
  cost := [2, white],
  types := [enchantment],
  text := [
    gatedBy (scaledGenericMana (forEach 2 [isCreature, controlledBy they, inCombat attackerOf you]))
      (actor (allOf isCreature)) (attack you)
  ]
}

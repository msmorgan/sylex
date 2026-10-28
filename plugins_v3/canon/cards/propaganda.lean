/-- Propaganda -/
card := {
  name := "Propaganda",
  cost := [2, blue],
  types := [enchantment],
  text := [
    gatedBy (scaledGenericMana (forEach 2 [isCreature, controlledBy they, inCombat attackerOf you]))
      (actor (allOf isCreature)) (attack you)
  ]
}

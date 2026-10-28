card := {
  name := "Soar",
  cost := [1, blue],
  types := [enchantment],
  subtypes := [aura],
  text := [
    asThoughFlashSacrificedAtCleanup,
    enchant isCreature,
    conjunction
      [getsPt (enchanted creature) (up 0) (up 1), abilityGrant (enchanted creature) flying]
      (subject := enchanted creature)
  ]
}

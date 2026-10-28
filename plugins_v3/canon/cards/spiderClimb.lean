/-- Spider Climb -/
card := {
  name := "Spider Climb",
  cost := [green],
  types := [enchantment],
  subtypes := [aura],
  text := [
    asThoughFlashSacrificedAtCleanup,
    enchant isCreature,
    conjunction [getsPt (enchanted creature) (up 0) (up 3), abilityGrant (enchanted creature) reach]
      (subject := enchanted creature)
  ]
}

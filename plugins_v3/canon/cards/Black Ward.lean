/-- Black Ward -/
card := {
  name := "Black Ward",
  cost := [white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    retention (abilityGrant (enchanted creature) (protection (hasColor black))) thisAura
  ]
}

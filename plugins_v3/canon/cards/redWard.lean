/-- Red Ward -/
card := {
  name := "Red Ward",
  cost := [white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    retention (abilityGrant (enchanted creature) (protection (hasColor red))) thisAura
  ]
}

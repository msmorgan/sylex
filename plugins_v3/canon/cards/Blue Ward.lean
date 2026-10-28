card := {
  name := "Blue Ward",
  cost := [white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    retention (abilityGrant (enchanted creature) (protection (hasColor blue))) thisAura
  ]
}

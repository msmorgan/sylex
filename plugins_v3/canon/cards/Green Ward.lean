card := {
  name := "Green Ward",
  cost := [white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    retention (abilityGrant (enchanted creature) (protection (hasColor green))) thisAura
  ]
}

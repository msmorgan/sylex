card := {
  name := "Angelic Gift",
  cost := [1, white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    when (does thisAura enter) (draw 1),
    abilityGrant (enchanted creature) flying
  ]
}

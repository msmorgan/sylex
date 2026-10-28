card := {
  name := "Spiteful Shadows",
  cost := [1, black],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    whenever (does any (dealDamage (to := enchanted creature)))
      (instruct it (dealDamage thatMuch (controllerOf it)))
  ]
}

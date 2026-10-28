card := {
  name := "Binding Agony",
  cost := [1, black],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    whenever (does any (dealDamage (to := enchanted creature)))
      (instruct thisAura (dealDamage thatMuch (controllerOf (that creature))))
  ]
}

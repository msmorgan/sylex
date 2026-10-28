card := {
  name := "Extra Arms",
  cost := [4, red],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    whenever (does (enchanted creature) attack) (instruct it (dealDamage 2 (target isAnyTarget)))
  ]
}

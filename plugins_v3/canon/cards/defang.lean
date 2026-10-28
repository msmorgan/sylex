card := {
  name := "Defang",
  cost := [1, white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    replace (would (by_ (enchanted creature)) (dealDamage anyAmount))
      (instead (preventDamage allDamage))
  ]
}

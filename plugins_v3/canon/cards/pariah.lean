card := {
  name := "Pariah",
  cost := [2, white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    replace (would any (dealDamage anyAmount you))
      (instead (redirectDamage allDamage (enchanted creature)))
  ]
}

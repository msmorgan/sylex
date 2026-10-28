card := {
  name := "Lush Growth",
  cost := [green],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isLand,
    qualityChange (enchanted land) sets (bundle { subtypes := [mountain, forest, plains] })
  ]
}

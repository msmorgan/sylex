card := {
  name := "Blood Moon",
  cost := [2, red],
  types := [enchantment],
  text := [
    qualityChange (allOf [isLand, non (hasSupertype basic)]) sets
      (bundle { subtypes := [mountain] })
  ]
}

card := {
  name := "Magus of the Moon",
  cost := [2, red],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    qualityChange (allOf [isLand, non (hasSupertype basic)]) sets
      (bundle { subtypes := [mountain] })
  ],
  power := 2,
  toughness := 2
}

card := {
  name := "Harbinger of the Seas",
  cost := [1, blue, blue],
  types := [creature],
  subtypes := [merfolk, wizard],
  text := [qualityChange (allOf [isLand, non (hasSupertype basic)]) sets (bundle { subtypes := [island] })],
  power := 2,
  toughness := 2
}

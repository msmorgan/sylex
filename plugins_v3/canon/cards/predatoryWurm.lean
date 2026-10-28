/-- Predatory Wurm -/
card := {
  name := "Predatory Wurm",
  cost := [3, green],
  types := [creature],
  subtypes := [wurm],
  text := [vigilance, onlyWhile (getsPt thisCreature (up 2) (up 2)) (exists_ [garruk, youControl])],
  power := 4,
  toughness := 4
}

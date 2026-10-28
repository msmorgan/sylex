/-- Spirit of the Labyrinth -/
card := {
  name := "Spirit of the Labyrinth",
  cost := [1, white],
  types := [enchantment, creature],
  subtypes := [spirit],
  text := [cant eachPlayer (drawCount (moreThan 1)) (timing := eachTurn)],
  power := 3,
  toughness := 1
}

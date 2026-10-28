card := {
  name := "Tocasia's Welcome",
  cost := [2, white],
  types := [enchantment],
  text := [
    whenever (does (counted (orMore 1) [isCreature, youControl, hasManaValue atMost 3]) enter)
      (draw 1) (limit := oncePerTurn)
  ]
}

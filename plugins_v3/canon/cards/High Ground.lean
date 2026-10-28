card := {
  name := "High Ground",
  cost := [white],
  types := [enchantment],
  text := [
    can (each [isCreature, youControl]) (block (counted (additional (exactly 1)) isCreature))
      (timing := eachCombat)
  ]
}

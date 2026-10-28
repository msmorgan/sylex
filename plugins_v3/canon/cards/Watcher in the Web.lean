card := {
  name := "Watcher in the Web",
  cost := [4, green],
  types := [creature],
  subtypes := [spider],
  text := [
    reach,
    can thisCreature (block (counted (additional (exactly 7)) isCreature)) (timing := eachCombat)
  ],
  power := 2,
  toughness := 5
}

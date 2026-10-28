card := {
  name := "Demon of Death's Gate",
  cost := [6, black, black, black],
  types := [creature],
  subtypes := [demon],
  text := [
    altCost this_
      (compound [payLife 6, (sacrifice (counted (exactly 3) [isCreature, hasColor black]))]),
    flying,
    trample
  ],
  power := 9,
  toughness := 9
}

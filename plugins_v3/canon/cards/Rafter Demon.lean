card := {
  name := "Rafter Demon",
  cost := [2, black, red],
  types := [creature],
  subtypes := [demon],
  text := [
    spectacle [3, black, red],
    when (does thisCreature enter) (if_ := keywordCostPaid spectacle thisCreature)
      (instruct eachOpponent (discard (a (cardIn hand))))
  ],
  power := 4,
  toughness := 2
}

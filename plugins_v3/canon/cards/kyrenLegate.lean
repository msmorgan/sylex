card := {
  name := "Kyren Legate",
  cost := [1, red],
  types := [creature],
  subtypes := [goblin],
  text := [
    onlyWhile (altCost this_)
      [exists_ [isLand, plains, anOpponentControls], exists_ [isLand, mountain, youControl]],
    haste
  ],
  power := 1,
  toughness := 1
}

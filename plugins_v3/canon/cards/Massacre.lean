card := {
  name := "Massacre",
  cost := [2, black, black],
  types := [sorcery],
  text := [
    onlyWhile (altCost this_)
      [exists_ [isLand, plains, anOpponentControls], exists_ [isLand, swamp, youControl]],
    spell (get (allOf isCreature) (down 2) (down 2) untilEndOfTurn)
  ]
}

card := {
  name := "Invigorate",
  cost := [2, green],
  types := [instant],
  text := [
    onlyWhile (altCost this_ (instruct anOpponent (gainLife 3)))
      (exists_ [isLand, forest, youControl]),
    spell (get (target isCreature) (up 4) (up 4) untilEndOfTurn)
  ]
}

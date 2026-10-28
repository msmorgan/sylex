card := {
  name := "Rouse",
  cost := [1, black],
  types := [instant],
  text := [
    onlyWhile (altCost this_ (payLife 2)) (exists_ [isLand, swamp, youControl]),
    spell (get (target isCreature) (up 2) (up 0) untilEndOfTurn)
  ]
}

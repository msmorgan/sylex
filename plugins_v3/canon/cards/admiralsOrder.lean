card := {
  name := "Admiral's Order",
  cost := [1, blue, blue],
  types := [instant],
  text := [
    onlyWhile (altCost this_ [blue]) (did you attack soFarThisTurn),
    spell (counter (target isSpell))
  ]
}

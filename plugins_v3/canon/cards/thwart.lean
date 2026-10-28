card := {
  name := "Thwart",
  cost := [2, blue, blue],
  types := [instant],
  text := [
    altCost this_ (returnToHand (counted (exactly 3) [isLand, island, youControl])),
    spell (counter (target isSpell))
  ]
}

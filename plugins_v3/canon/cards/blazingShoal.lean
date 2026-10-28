card := {
  name := "Blazing Shoal",
  cost := [«variable», red, red],
  types := [instant],
  subtypes := [arcane],
  text := [
    altCost this_ (exile (a [hasColor red, hasManaValue eq (letter x), in_ yourHand])),
    spell (get (target isCreature) (up (letter x)) (up 0) untilEndOfTurn)
  ]
}

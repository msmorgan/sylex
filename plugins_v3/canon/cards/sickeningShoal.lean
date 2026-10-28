card := {
  name := "Sickening Shoal",
  cost := [«variable», black, black],
  types := [instant],
  subtypes := [arcane],
  text := [
    altCost this_ (exile (a [hasColor black, hasManaValue eq (letter x), in_ yourHand])),
    spell (get (target isCreature) (down (letter x)) (down (letter x)) untilEndOfTurn)
  ]
}

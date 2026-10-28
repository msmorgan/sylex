card := {
  name := "Nourishing Shoal",
  cost := [«variable», green, green],
  types := [instant],
  subtypes := [arcane],
  text := [
    altCost this_ (exile (a [hasColor green, hasManaValue eq (letter x), in_ yourHand])),
    spell (gainLife (letter x))
  ]
}

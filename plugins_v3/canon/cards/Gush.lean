card := {
  name := "Gush",
  cost := [4, blue],
  types := [instant],
  text := [altCost this_ (returnToHand (counted (exactly 2) [isLand, island, youControl])), spell (draw 2)]
}

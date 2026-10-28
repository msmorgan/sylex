card := {
  name := "Repeal",
  cost := [«variable», blue],
  types := [instant],
  text := [spell [returnToHand (target [non isLand, isPermanent, hasManaValue eq (letter x)]), draw 1]]
}

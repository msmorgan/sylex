card := {
  name := "Reciprocate",
  cost := [white],
  types := [instant],
  text := [spell (exile (target [isCreature, did which (dealDamage anyAmount you) (within soFarThisTurn)]))]
}

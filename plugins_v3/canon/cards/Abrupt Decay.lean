card := {
  name := "Abrupt Decay",
  cost := [black, green],
  types := [instant],
  text := [
    cant any (counter thisSpell),
    spell (destroy (target [isPermanent, non isLand, hasManaValue atMost 3]))
  ]
}

card := {
  name := "Spell Snare",
  cost := [blue],
  types := [instant],
  text := [spell (counter (target [isSpell, hasManaValue eq 2]))]
}

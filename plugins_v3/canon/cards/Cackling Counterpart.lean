card := {
  name := "Cackling Counterpart",
  cost := [1, blue, blue],
  types := [instant],
  text := [spell (create 1 (copyOf (target [isCreature, youControl])) []), flashback [5, blue, blue]]
}

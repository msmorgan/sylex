card := {
  name := "Oust",
  cost := [white],
  types := [sorcery],
  text := [spell [putInto (target isCreature) (fromTop 2 library), instruct (controllerOf it) (gainLife 3)]]
}

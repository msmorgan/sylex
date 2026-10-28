card := {
  name := "True-Name Nemesis",
  cost := [1, blue, blue],
  types := [creature],
  subtypes := [merfolk, rogue],
  text := [
    entersChoosingPlayer thisCreature (a isPlayer),
    protection (objectsOfPlayer (the isChosenPlayer))
  ],
  power := 3,
  toughness := 1
}

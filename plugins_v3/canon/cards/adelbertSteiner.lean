card := {
  name := "Adelbert Steiner",
  cost := [1, white],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, knight],
  text := [
    lifelink,
    getsPt thisCreature (up (forEach 1 [equipment, youControl]))
      (up (forEach 1 [equipment, youControl]))
  ],
  power := 2,
  toughness := 1
}

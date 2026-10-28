card := {
  name := "Courser of Kruphix",
  cost := [1, green, green],
  types := [enchantment, creature],
  subtypes := [centaur],
  text := [
    visibility reveal you topOfLibrary,
    can you (play (allOf isLand) (from_ := onTop)),
    whenever (does (a [isLand, youControl]) enter) (gainLife 1)
  ],
  power := 2,
  toughness := 4
}

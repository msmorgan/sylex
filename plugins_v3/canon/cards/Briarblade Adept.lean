card := {
  name := "Briarblade Adept",
  cost := [4, black],
  types := [creature],
  subtypes := [elf, assassin],
  text := [
    whenever (does thisCreature attack)
      (get (target [isCreature, opponentsControl]) (down 1) (down 1) untilEndOfTurn),
    encore [3, black]
  ],
  power := 3,
  toughness := 4
}

card := {
  name := "Chub Toad",
  cost := [2, green],
  types := [creature],
  subtypes := [frog],
  text := [
    whenever (does thisCreature block) (or_ := [does any (becomeBlocked thisCreature)])
      (get it (up 2) (up 2) untilEndOfTurn)
  ],
  power := 1,
  toughness := 1
}

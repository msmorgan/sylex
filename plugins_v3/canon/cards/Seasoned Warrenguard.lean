card := {
  name := "Seasoned Warrenguard",
  cost := [white],
  types := [creature],
  subtypes := [rabbit, warrior],
  text := [
    whenever (does thisCreature attack) (while_ := whileTrue (exists_ [isToken, youControl]))
      (get thisCreature (up 2) (up 0) untilEndOfTurn)
  ],
  power := 1,
  toughness := 2
}

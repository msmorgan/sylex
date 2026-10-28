card := {
  name := "Char-Rumbler",
  cost := [2, red, red],
  types := [creature],
  subtypes := [elemental],
  text := [doubleStrike, activated [red] (get thisCreature (up 1) (up 0) untilEndOfTurn)],
  power := some (lit (-1)),
  toughness := 3
}

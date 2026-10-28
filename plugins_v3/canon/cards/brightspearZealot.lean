card := {
  name := "Brightspear Zealot",
  cost := [2, white],
  types := [creature],
  subtypes := [human, soldier],
  text := [
    vigilance,
    onlyWhile (getsPt thisCreature (up 2) (up 0))
      (compareAmt (eventCount (cast (a isSpell)) you soFarThisTurn) atLeast 2)
  ],
  power := 2,
  toughness := 4
}

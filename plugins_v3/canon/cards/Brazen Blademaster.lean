card := {
  name := "Brazen Blademaster",
  cost := [2, red],
  types := [creature],
  subtypes := [orc, pirate],
  text := [
    whenever (does thisCreature attack)
      (while_ := whileTrue (compareAmt (countOf [isArtifact, youControl]) atLeast 2))
      (get it (up 2) (up 1) untilEndOfTurn)
  ],
  power := 2,
  toughness := 3
}

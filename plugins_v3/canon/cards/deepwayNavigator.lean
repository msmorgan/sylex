card := {
  name := "Deepway Navigator",
  cost := [white, blue],
  types := [creature],
  subtypes := [merfolk, wizard],
  text := [
    flash,
    when (does thisCreature enter) (untap (each [merfolk, youControl, isOther])),
    onlyWhile (getsPt (allOf [merfolk, youControl]) (up 1) (up 0))
      (did you (attackWith (counted (orMore 3) merfolk)) soFarThisTurn)
  ],
  power := 2,
  toughness := 2
}

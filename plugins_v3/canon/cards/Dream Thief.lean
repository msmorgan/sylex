card := {
  name := "Dream Thief",
  cost := [2, blue],
  types := [creature],
  subtypes := [faerie, rogue],
  text := [
    flying,
    when (does thisCreature enter)
      (doOnlyIf (draw 1) (did you (cast (a [isSpell, hasColor blue, isOther])) soFarThisTurn))
  ],
  power := 2,
  toughness := 1
}

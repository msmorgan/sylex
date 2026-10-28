/-- Council of the Absolute -/
card := {
  name := "Council of the Absolute",
  cost := [2, white, blue],
  types := [creature],
  subtypes := [human, advisor],
  text := [
    entersChoosingFrom thisCreature cardName (nameOfCard (not (or [isCreature, isLand]))),
    cant yourOpponents (cast (bare [isSpell, named chosenName])),
    costShift (allOf [isSpell, named chosenName, did you (cast which) thisObject]) (less 2)
  ],
  power := 2,
  toughness := 4
}

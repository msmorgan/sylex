/-- Angus Mackenzie -/
card := {
  name := "Angus Mackenzie",
  cost := [green, white, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, cleric],
  text := [
    activatedOnlyDuring (compound [[green, white, blue], tapSymbol])
      (preventAll any thisTurn (some combat)) (beforePart combatDamage none)
  ],
  power := 2,
  toughness := 2
}

/-- Edgewalker -/
card := {
  name := "Edgewalker",
  cost := [1, white, black],
  types := [creature],
  subtypes := [human, cleric],
  text := [
    costShift (allOf [cleric, isSpell, did you (cast which) thisObject])
      (lessBySymbols (symbols [white, black]) (coloredOnly := true))
  ],
  power := 2,
  toughness := 2
}

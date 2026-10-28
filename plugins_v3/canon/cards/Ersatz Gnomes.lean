/-- Ersatz Gnomes -/
card := {
  name := "Ersatz Gnomes",
  cost := [3],
  types := [artifact, creature],
  subtypes := [gnome],
  text := [
    activated tapSymbol (becomeColor (target isSpell) (.some [])),
    activated tapSymbol (becomeColor (target isPermanent) (.some []) untilEndOfTurn)
  ],
  power := 1,
  toughness := 1
}

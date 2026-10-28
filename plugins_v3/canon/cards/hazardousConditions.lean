/-- Hazardous Conditions -/
card := {
  name := "Hazardous Conditions",
  cost := [2, black, green],
  types := [sorcery],
  text := [spell (get (allOf [isCreature, not hasCounter]) (down 2) (down 2) untilEndOfTurn)]
}

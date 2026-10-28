/-- Ertai's Trickery -/
card := {
  name := "Ertai's Trickery",
  cost := [blue],
  types := [instant],
  text := [spell (doOnlyIf (counter (target isSpell)) (keywordCostPaid kicker it))]
}

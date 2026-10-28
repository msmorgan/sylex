/-- Liliana's Caress -/
card := {
  name := "Liliana's Caress",
  cost := [1, black],
  types := [enchantment],
  text := [whenever (does anOpponent (discard (a (cardIn hand)))) (instruct they (loseLife 2))]
}

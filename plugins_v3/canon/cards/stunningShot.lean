/-- Stunning Shot -/
card := {
  name := "Stunning Shot",
  cost := [1, white],
  types := [sorcery],
  text := [
    spell
      [
        putCounters 2 p1p1Counter (targets (upTo 1) [isCreature, youControl]),
        tap (targets (upTo 1) [isCreature, anOpponentControls]),
        putCounters 1 stunCounter it
      ]
  ]
}

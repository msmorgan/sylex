/-- Rites of Flourishing -/
card := {
  name := "Rites of Flourishing",
  cost := [2, green],
  types := [enchantment],
  text := [
    at_ (beginningOfPossessed drawStep eachPlayer) (instruct (that player) (draw 1)),
    can eachPlayer (play (counted (additional (exactly 1)) isLand)) (timing := onEachOfTheirTurns)
  ]
}

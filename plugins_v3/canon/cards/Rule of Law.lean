/-- Rule of Law -/
card := {
  name := "Rule of Law",
  cost := [2, white],
  types := [enchantment],
  text := [cant eachPlayer (cast (counted (moreThan 1) isSpell)) (timing := eachTurn)]
}

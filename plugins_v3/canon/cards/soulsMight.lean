/-- Soul's Might -/
card := {
  name := "Soul's Might",
  cost := [4, green],
  types := [sorcery],
  text := [
    spell
      [putCounters (letter x) p1p1Counter (target isCreature), define x (powerOf (that creature))]
  ]
}

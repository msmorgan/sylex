/-- Bristlepack Sentry -/
card := {
  name := "Bristlepack Sentry",
  cost := [1, green],
  types := [creature],
  subtypes := [plant, wolf],
  text := [
    defender,
    onlyWhile (can thisCreature attack (asThough := holds (is it (not (hasKeyword defender)))))
      (exists_ [isCreature, youControl, hasPower atLeast 4])
  ],
  power := 3,
  toughness := 3
}

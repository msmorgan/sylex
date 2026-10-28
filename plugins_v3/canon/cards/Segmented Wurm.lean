/-- Segmented Wurm: "Whenever this creature becomes the target of a spell or ability, put a
−1/−1 counter on it." -/
card := {
  name := "Segmented Wurm",
  cost := [3, red, green],
  types := [creature],
  subtypes := [wurm],
  text := [
    whenever (does (a (or [isSpell, abilityHead anyOnStack])) (target thisCreature))
      (putCounters 1 m1m1Counter it)
  ],
  power := 5,
  toughness := 5
}

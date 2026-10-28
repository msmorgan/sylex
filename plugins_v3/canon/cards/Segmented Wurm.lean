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

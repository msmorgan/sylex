card := {
  name := "Woodland Champion",
  cost := [1, green],
  types := [creature],
  subtypes := [elf, scout],
  text := [
    whenever (does (counted (orMore 1) [isToken, youControl]) enter)
      (putCounters groupSize p1p1Counter thisCreature)
  ],
  power := 2,
  toughness := 2
}

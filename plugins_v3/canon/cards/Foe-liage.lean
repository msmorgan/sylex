card := {
  name := "Foe-liage",
  cost := [3, green],
  types := [creature],
  subtypes := [plant, mutant],
  text := [
    whenever (does (a isLand) enter) (during := duringPart turn you)
      (putCounters 1 p1p1Counter thisCreature)
  ],
  power := 3,
  toughness := 3
}

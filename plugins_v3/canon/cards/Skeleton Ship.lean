card := {
  name := "Skeleton Ship",
  cost := [3, blue, black],
  supertypes := [legendary],
  types := [creature],
  subtypes := [skeleton],
  text := [
    when (stateHolds (not (exists_ [isLand, island, youControl]))) (sacrifice thisCreature),
    activated tapSymbol (putCounters 1 m1m1Counter (target isCreature))
  ],
  power := 0,
  toughness := 3
}

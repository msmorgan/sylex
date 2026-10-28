card := {
  name := "Grimdancer",
  cost := [1, black, black],
  types := [creature],
  subtypes := [nightmare],
  text := [
    enters thisCreature
      [
        withCounters 2 (distinctChosenKinds [menaceCounter, deathtouchCounter, lifelinkCounter])
          fresh
      ]
  ],
  power := 3,
  toughness := 3
}

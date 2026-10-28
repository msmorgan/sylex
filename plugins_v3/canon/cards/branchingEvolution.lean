card := {
  name := "Branching Evolution",
  cost := [2, green],
  types := [enchantment],
  text := [
    replace (would any (putCounters anyAmount p1p1Counter (a [isCreature, youControl])))
      (instead (putCounters (times 2 thatMuch) p1p1Counter (that creature)))
  ]
}

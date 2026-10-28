card := {
  name := "Hardened Scales",
  cost := [green],
  types := [enchantment],
  text := [
    replace (would any (putCounters anyAmount p1p1Counter (a [isCreature, youControl])))
      (instead (putCounters (plus thatMuch 1) p1p1Counter it))
  ]
}

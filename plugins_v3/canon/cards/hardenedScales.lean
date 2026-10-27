card hardenedScales : Spelled :=
  { name := "Hardened Scales", cost := [pip .green], types := [.enchantment],
    text :=
      [ static (replace (would any (putCounters anyAmount p1p1Counter (a (and [isCreature, youControl])))) (instead (putCounters (plus thatMuch 1) p1p1Counter it))) ] }

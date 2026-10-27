card shalaiAndHallar : Spelled :=
  { name := "Shalai and Hallar", cost := [generic 1, pip .red, pip .green, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [angel, elf],
    text :=
      [ flying, vigilance,
        whenever (does any (putCounters anyAmount p1p1Counter (a (and [isCreature, youControl]))))
          (instruct thisCreature (dealDamage thatMuch (target isOpponent))) ],
    power := stat 3, toughness := stat 3 }

/-- Galloping Lizrog -/
card gallopingLizrog : Spelled :=
  { name := "Galloping Lizrog", cost := [generic 3, pip .green, pip .blue],
    types := [.creature], subtypes := [frog, lizard],
    text :=
      [ trample,
        when (does thisCreature enter)
          (may
            (removeCounters anyNumber p1p1Counter (among (allOf (and [isCreature, youControl]))))
            (putCounters (times 2 thatMuch) p1p1Counter thisCreature)) ],
    power := stat 3, toughness := stat 3 }

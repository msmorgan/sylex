card aboroth : Spelled :=
  { name := "Aboroth", cost := [generic 4, pip .green, pip .green], types := [.creature],
    subtypes := [elemental],
    text :=
      [ cumulativeUpkeep (putCounters 1 m1m1Counter thisCreature) ],
    power := stat 9, toughness := stat 9 }

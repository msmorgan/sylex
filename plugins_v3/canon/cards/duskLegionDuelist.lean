card duskLegionDuelist : Spelled :=
  { name := "Dusk Legion Duelist", cost := [generic 1, pip .white], types := [.creature],
    subtypes := [vampire, soldier],
    text :=
      [ vigilance,
        whenever (does any (putCounters anyAmount p1p1Counter thisCreature))
          (draw 1) (limit := oncePerTurn) ],
    power := stat 2, toughness := stat 2 }

card herdBaloth : Spelled :=
  { name := "Herd Baloth", cost := [generic 3, pip .green, pip .green], types := [.creature],
    subtypes := [beast],
    text :=
      [ whenever (does any (putCounters anyAmount p1p1Counter thisCreature))
          (may (create 1 (creatureToken 4 4 [.green] [beast]))) ],
    power := stat 4, toughness := stat 4 }

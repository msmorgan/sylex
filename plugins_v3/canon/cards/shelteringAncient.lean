card shelteringAncient : Spelled :=
  { name := "Sheltering Ancient", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [treefolk],
    text :=
      [ trample,
        cumulativeUpkeep
          (putCounters 1 p1p1Counter
            (a (and [isCreature, anOpponentControls]))) ],
    power := stat 5, toughness := stat 5 }

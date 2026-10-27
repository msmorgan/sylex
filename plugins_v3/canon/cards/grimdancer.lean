/-- Grimdancer -/
card grimdancer : Spelled :=
  { name := "Grimdancer", cost := [generic 1, pip .black, pip .black], types := [.creature],
    subtypes := [nightmare],
    text :=
      [ static (enters thisCreature
          [withCounters 2
            (distinctChosenKinds [menaceCounter, deathtouchCounter, lifelinkCounter]) .fresh]) ],
    power := stat 3, toughness := stat 3 }

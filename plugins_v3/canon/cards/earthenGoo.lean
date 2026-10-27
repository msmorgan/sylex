/-- Earthen Goo -/
card earthenGoo : Spelled :=
  { name := "Earthen Goo", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [ooze],
    text :=
      [ trample,
        cumulativeUpkeep (either [pip .red] [pip .green]),
        static (getsPt thisCreature
          (up (times 1 (countersOn ageCounter it)))
          (up (times 1 (countersOn ageCounter it)))) ],
    power := stat 2, toughness := stat 2 }

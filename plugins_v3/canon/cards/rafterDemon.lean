/-- Rafter Demon -/
card rafterDemon : Spelled :=
  { name := "Rafter Demon", cost := [generic 2, pip .black, pip .red], types := [.creature],
    subtypes := [demon],
    text :=
      [ spectacle [generic 3, pip .black, pip .red],
        when (does thisCreature enter)
          (if_ := keywordCostPaid spectacle thisCreature)
          (instruct eachOpponent (discard (a (cardIn hand)))) ],
    power := stat 4, toughness := stat 2 }

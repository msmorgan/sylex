/-- Skeleton Ship -/
card skeletonShip : Spelled :=
  { name := "Skeleton Ship", cost := [generic 3, pip .blue, pip .black],
    supertypes := [.legendary], types := [.creature], subtypes := [skeleton],
    text :=
      [ when
          (stateHolds
            (not (exists_ (and [isLand, island, youControl]))))
          (sacrifice thisCreature),
        activated tapSymbol (putCounters 1 m1m1Counter (target isCreature)) ],
    power := stat 0, toughness := stat 3 }

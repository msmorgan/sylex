/-- Shah of Naar Isle -/
card shahOfNaarIsle : Spelled :=
  { name := "Shah of Naar Isle", cost := [generic 3, pip .red], types := [.creature],
    subtypes := [efreet],
    text :=
      [ trample,
        echo [generic 0],
        when (does any (pay anyCost .once .paid (some thisCreature) (some echo)))
          (instruct eachOpponent (may (drawCount (upToOf 3)))) ],
    power := stat 6, toughness := stat 6 }

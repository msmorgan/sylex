card reefShaman : Spelled :=
  { name := "Reef Shaman", cost := [pip .blue], types := [.creature],
    subtypes := [merfolk, shaman],
    text :=
      [ activated tapSymbol
          (establishThroughout (qualityChange (target isLand) .sets
              (chosenQuality
                 (ofYourChoice (.subtype .land) basicTypesOnly))) untilEndOfTurn) ],
    power := stat 0, toughness := stat 2 }

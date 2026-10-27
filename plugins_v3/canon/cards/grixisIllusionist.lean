card grixisIllusionist : Spelled :=
  { name := "Grixis Illusionist", cost := [pip .blue], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ activated tapSymbol
          (establishThroughout (qualityChange (target (and [isLand, youControl])) .sets
              (chosenQuality
                 (ofYourChoice (.subtype .land) basicTypesOnly))) untilEndOfTurn) ],
    power := stat 1, toughness := stat 1 }

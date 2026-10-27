card imagecrafter : Spelled :=
  { name := "Imagecrafter", cost := [pip .blue], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ activated tapSymbol
          (sequentially
            [ choose (a (qualityFrom (.subtype .creature) (typeOtherThan wall))),
              establishThroughout (qualityChange (target isCreature) .sets
                                     (chosenQuality (ofChosen (.subtype
                    .creature)))) untilEndOfTurn ]) ],
    power := stat 1, toughness := stat 1 }

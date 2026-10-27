card unnaturalSelection : Spelled :=
  { name := "Unnatural Selection", cost := [generic 1, pip .blue], types := [.enchantment],
    text :=
      [ activated [generic 1]
          (sequentially
            [ choose (a (qualityFrom (.subtype .creature) (typeOtherThan wall))),
              establishThroughout (qualityChange (target isCreature) .sets
                                     (chosenQuality (ofChosen (.subtype
                    .creature)))) untilEndOfTurn ]) ] }

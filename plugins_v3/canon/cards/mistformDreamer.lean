card mistformDreamer : Spelled :=
  { name := "Mistform Dreamer", cost := [generic 2, pip .blue], types := [.creature],
    subtypes := [illusion],
    text :=
      [ flying,
        activated [generic 1]
          (establishThroughout (qualityChange thisCreature .sets
              (chosenQuality (ofYourChoice (.subtype .creature)))) untilEndOfTurn) ],
    power := stat 2, toughness := stat 1 }

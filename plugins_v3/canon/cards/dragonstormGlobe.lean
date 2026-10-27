card dragonstormGlobe : Spelled :=
  { name := "Dragonstorm Globe", cost := [generic 3], types := [.artifact],
    text :=
      [ static (entersWithAdditionalCounters
          (each (and [dragon, youControl]))
          1 p1p1Counter),
        activated tapSymbol (addMana 1 (ofAnyColor .sameColor)) ] }

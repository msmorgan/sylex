card solGrail : Spelled :=
  { name := "Sol Grail", cost := [generic 3], types := [.artifact],
    text :=
      [ static (entersChoosing thisArtifact .color),
        activated tapSymbol (addMana 1 ofTheChosenColor) ] }

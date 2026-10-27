card endlessAtlas : Spelled :=
  { name := "Endless Atlas", cost := [generic 2], types := [.artifact],
    text :=
      [ activatedOnlyIf (compound [[generic 2], tapSymbol]) (draw 1)
          (thereAre (withTheSameName (counted (orMore 3) (and [isLand, youControl])))) ] }

/-- Mirror Universe -/
card mirrorUniverse : Spelled :=
  { name := "Mirror Universe", cost := [generic 6], types := [.artifact],
    text :=
      [ activatedOnlyDuring (compound [tapSymbol, sacrifice thisArtifact])
          (exchangeLifeTotals (youAnd (target isOpponent)))
          (duringPart .upkeep you) ] }

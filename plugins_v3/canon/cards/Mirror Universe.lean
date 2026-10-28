card := {
  name := "Mirror Universe",
  cost := [6],
  types := [artifact],
  text := [
    activatedOnlyDuring (compound [tapSymbol, sacrifice thisArtifact])
      (exchangeLifeTotals (youAnd (target isOpponent))) (duringPart upkeep you)
  ]
}

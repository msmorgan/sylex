card := {
  name := "Magistrate's Scepter",
  cost := [3],
  types := [artifact],
  text := [
    activated (compound [[4], tapSymbol]) (putCounters 1 chargeCounter thisArtifact),
    activated (compound [tapSymbol, (removeCounters (exactly 3) chargeCounter thisArtifact)])
      (extraTurn 1)
  ]
}

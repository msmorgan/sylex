card := {
  name := "Ratchet Bomb",
  cost := [2],
  types := [artifact],
  text := [
    activated tapSymbol (putCounters 1 chargeCounter thisArtifact),
    activated (compound [tapSymbol, sacrifice thisArtifact])
      (destroy
         (each [non isLand, isPermanent, hasManaValue eq (countersOn chargeCounter thisArtifact)]))
  ]
}

/-- Engineered Explosives (its sunburst [CR#702.44a] written out) -/
card := {
  name := "Engineered Explosives",
  cost := some [«variable»],
  types := [artifact],
  text := [
    entersWithCounters thisArtifact (colorsSpentToCast thisArtifact) chargeCounter,
    activated (compound [[2], sacrifice thisArtifact])
      (destroy
         (each [isPermanent, non isLand, hasManaValue eq (countersOn chargeCounter thisArtifact)]))
  ]
}

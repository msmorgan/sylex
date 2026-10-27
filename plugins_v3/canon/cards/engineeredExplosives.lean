/-- Engineered Explosives (its sunburst [CR#702.44a] written out) -/
card engineeredExplosives : Spelled :=
  { name := "Engineered Explosives", cost := some [.variable], types := [.artifact],
    text :=
      [ static (entersWithCounters thisArtifact (colorsSpentToCast thisArtifact) chargeCounter),
        activated (compound [[generic 2], sacrifice thisArtifact])
          (destroy (each (and [isPermanent, non isLand,
            hasManaValue .eq (countersOn chargeCounter thisArtifact)]))) ] }

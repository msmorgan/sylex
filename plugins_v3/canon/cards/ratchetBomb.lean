card ratchetBomb : Spelled :=
  { name := "Ratchet Bomb", cost := [generic 2], types := [.artifact],
    text :=
      [ activated tapSymbol (putCounters 1 chargeCounter thisArtifact),
        activated (compound [tapSymbol, sacrifice thisArtifact])
          (destroy (each (and [non isLand, isPermanent,
            hasManaValue .eq (countersOn chargeCounter thisArtifact)]))) ] }

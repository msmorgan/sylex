card magistratesScepter : Spelled :=
  { name := "Magistrate's Scepter", cost := [generic 3], types := [.artifact],
    text :=
      [ activated (compound [[generic 4], tapSymbol])
          (putCounters 1 chargeCounter thisArtifact),
        activated
          (compound
            [ tapSymbol,
              (removeCounters (exactly 3) chargeCounter thisArtifact) ])
          (extraTurn 1) ] }

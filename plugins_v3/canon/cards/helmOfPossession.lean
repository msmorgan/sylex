card helmOfPossession : Spelled :=
  { name := "Helm of Possession", cost := [generic 4], types := [.artifact],
    text :=
      [ static (mayDeclineUntap thisArtifact you),
        activated (compound [[generic 2], tapSymbol, sacrifice (a isCreature)])
          (gainControl (target isCreature)
            (forAsLongAs
              (and [ is thisArtifact youControl,
                      is thisArtifact isTapped ]))) ] }

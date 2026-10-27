card coretapper : Spelled :=
  { name := "Coretapper", cost := [generic 2], types := [.artifact, .creature],
    subtypes := [myr],
    text :=
      [ activated tapSymbol (putCounters 1 chargeCounter (target isArtifact)),
        activated (sacrifice thisCreature)
          (putCounters 2 chargeCounter (target isArtifact)) ],
    power := stat 1, toughness := stat 1 }

card crash : Spelled :=
  { name := "Crash", cost := [generic 2, pip .red], types := [.instant],
    text :=
      [ static (altCost this_
          (sacrifice (a (and [isLand, mountain])))),
        spell (destroy (target isArtifact)) ] }

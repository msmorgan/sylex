card moggSalvage : Spelled :=
  { name := "Mogg Salvage", cost := [generic 2, pip .red], types := [.instant],
    text :=
      [ static (onlyWhile (altCost this_)
          (and
            [ exists_ (and [ isLand, island,
                              anOpponentControls ]),
              exists_ (and [ isLand, mountain,
                              youControl ]) ])),
        spell (destroy (target isArtifact)) ] }

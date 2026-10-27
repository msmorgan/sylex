/-- Tezzeret's Gatebreaker -/
card tezzeretsGatebreaker : Spelled :=
  { name := "Tezzeret's Gatebreaker", cost := [generic 4], types := [.artifact],
    text :=
      [ when (does thisArtifact enter)
          (sequentially
            [ lookAt (topSlice 5),
              may
                (sequentially
                  [ reveal
                      (fromAmong (exactly 1) (or [hasColor .blue, isArtifact]) them),
                    putInto (that card) hand ]),
              putInto theRest onBottom (riders := [(inOrder .randomOrder)]) ]),
        activated
          (compound [[generic 5, pip .blue], tapSymbol,
                      (sacrifice thisArtifact)])
          (establishThroughout (cant any (block (allOf (and [isCreature, youControl]))))
             thisTurn) ] }

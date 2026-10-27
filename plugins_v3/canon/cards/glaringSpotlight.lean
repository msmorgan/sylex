/-- Glaring Spotlight -/
card glaringSpotlight : Spelled :=
  { name := "Glaring Spotlight", cost := [generic 1], types := [.artifact],
    text :=
      [ static
          (can (by_ (allOf (and [or [isSpell, abilityHead .anyOnStack], youControl])))
             (target (allOf (and [ isCreature, opponentsControl,
                         hasKeyword hexproof ]))) (asThough := holds (is them (not (hasKeyword hexproof))))),
        activated (compound [[generic 3], sacrifice thisArtifact])
          (sequentially
            [ gain (allOf (and [isCreature, youControl])) hexproof untilEndOfTurn,
              establishThroughout (cant any
                                     (block
                                        (allOf (and [isCreature, youControl])))) thisTurn ]) ] }

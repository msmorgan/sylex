card saheeliFiligreeMaster : Spelled :=
  { name := "Saheeli, Filigree Master", cost := [generic 2, pip .blue, pip .red],
    supertypes := [.legendary], types := [.planeswalker], subtypes := [saheeli],
    text :=
      [ activated (loyaltySymbol (up 1))
          (sequentially
            [ scry 1,
              may
                (tap (a (and [isArtifact, isUntapped, youControl])))
                (draw 1) ]),
        activated (loyaltySymbol (down 2))
          (sequentially
            [ create 2 (written
                { characteristics :=
                  { types := [.artifact, .creature], subtypes := [thopter],
                    text := [flying], power := stat 1, toughness := stat 1 } }),
              gainHaste them untilEndOfTurn ]),
        activated (loyaltySymbol (down 4))
          (getEmblem
            [ static (getsPt (allOf (and [isArtifact, isCreature, youControl]))
                (up 1) (up 1)),
              static (costShift (allOf (and [isArtifact, isSpell,
                  did you (cast which) thisObject])) (less 1)) ]) ],
    loyalty := stat 3 }

card gideonAllyOfZendikar : Spelled :=
  { name := "Gideon, Ally of Zendikar", cost := [generic 2, pip .white, pip .white],
    supertypes := [.legendary], types := [.planeswalker], subtypes := [gideon],
    text :=
      [ activated (loyaltySymbol (up 1))
          (sequentially
            [ establishThroughout (qualityChange thisPlaneswalker .sets
                  (bundle
                    { characteristics :=
                      { types := [.creature],
                        subtypes := [human, soldier, ally],
                        text := [indestructible], power := stat 5, toughness := stat 5 } }
                    (some .planeswalker))) untilEndOfTurn,
              preventAll it thisTurn ]),
        activated (loyaltySymbol (up 0))
          (create 1 (creatureToken 2 2 [.white] [knight, ally])),
        activated (loyaltySymbol (down 4))
          (getEmblem
            [static (getsPt (allOf (and [isCreature, youControl])) (up 1) (up 1))]) ],
    loyalty := stat 4 }

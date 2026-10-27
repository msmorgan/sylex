card elspethSunsChampion : Spelled :=
  { name := "Elspeth, Sun's Champion", cost := [generic 4, pip .white, pip .white],
    supertypes := [.legendary], types := [.planeswalker], subtypes := [elspeth],
    text :=
      [ activated (loyaltySymbol (up 1))
          (create 3 (creatureToken 1 1 [.white] [soldier])),
        activated (loyaltySymbol (down 3))
          (destroy (allOf (and [isCreature, hasPower .atLeast 4]))),
        activated (loyaltySymbol (down 7))
          (getEmblem
            [ static (conjunction [ getsPt (allOf (and [isCreature, youControl])) (up 2) (up 2),
                  abilityGrant them flying ]) ]) ],
    loyalty := stat 4 }

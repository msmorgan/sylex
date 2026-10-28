card := {
  name := "Gideon, Ally of Zendikar",
  cost := [2, white, white],
  supertypes := [legendary],
  types := [planeswalker],
  subtypes := [gideon],
  text := [
    activated (loyaltySymbol (up 1))
      [
        establishThroughout
          (qualityChange thisPlaneswalker sets
             (bundle
                {
                  types := [creature],
                  subtypes := [human, soldier, ally],
                  text := [indestructible],
                  power := 5,
                  toughness := 5
                }
                (some planeswalker)))
          untilEndOfTurn,
        preventAll it thisTurn
      ],
    activated (loyaltySymbol (up 0)) (create 1 (creatureToken 2 2 [white] [knight, ally])),
    activated (loyaltySymbol (down 4))
      (getEmblem [getsPt (allOf [isCreature, youControl]) (up 1) (up 1)])
  ],
  loyalty := 4
}

card := {
  name := "Gideon Jura",
  cost := [3, white, white],
  supertypes := [legendary],
  types := [planeswalker],
  subtypes := [gideon],
  text := [
    activated (loyaltySymbol (up 2))
      (establishThroughout
         (must (allOf [isCreature, controlledBy (target isOpponent)]) (attack thisPlaneswalker))
         (duringNextTurnOf (that player))),
    activated (loyaltySymbol (down 2)) (destroy (target [isCreature, isTapped])),
    activated (loyaltySymbol (up 0))
      [
        establishThroughout
          (qualityChange thisPlaneswalker sets
             (bundle
                { types := [creature], subtypes := [human, soldier], power := 6, toughness := 6 }
                (some planeswalker)))
          untilEndOfTurn,
        preventAll it thisTurn
      ]
  ],
  loyalty := 6
}

card := {
  name := "Saheeli, Filigree Master",
  cost := [2, blue, red],
  supertypes := [legendary],
  types := [planeswalker],
  subtypes := [saheeli],
  text := [
    activated (loyaltySymbol (up 1))
      [scry 1, may (tap (a [isArtifact, isUntapped, youControl])) (draw 1)],
    activated (loyaltySymbol (down 2))
      [
        create 2
          (written
             {
               types := [artifact, creature],
               subtypes := [thopter],
               text := [flying],
               power := 1,
               toughness := 1
             }),
        gainHaste them untilEndOfTurn
      ],
    activated (loyaltySymbol (down 4))
      (getEmblem
         [
           getsPt (allOf [isArtifact, isCreature, youControl]) (up 1) (up 1),
           costShift (allOf [isArtifact, isSpell, did you (cast which) thisObject]) (less 1)
         ])
  ],
  loyalty := 3
}

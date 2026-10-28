card := {
  name := "Elspeth, Sun's Champion",
  cost := [4, white, white],
  supertypes := [legendary],
  types := [planeswalker],
  subtypes := [elspeth],
  text := [
    activated (loyaltySymbol (up 1)) (create 3 (creatureToken 1 1 [white] [soldier])),
    activated (loyaltySymbol (down 3)) (destroy (allOf [isCreature, hasPower atLeast 4])),
    activated (loyaltySymbol (down 7))
      (getEmblem
         [static [getsPt (allOf [isCreature, youControl]) (up 2) (up 2), abilityGrant them flying]])
  ],
  loyalty := 4
}

/-- Liliana of the Veil -/
card := {
  name := "Liliana of the Veil",
  cost := [1, black, black],
  supertypes := [legendary],
  types := [planeswalker],
  subtypes := [liliana],
  text := [
    activated (loyaltySymbol (up 1)) (instruct eachPlayer (discard (a (cardIn hand)))),
    activated (loyaltySymbol (down 2)) (instruct (target isPlayer) (sacrifice (a isCreature))),
    activated (loyaltySymbol (down 6))
      [
        separateIntoPiles (allOf [isPermanent, controlledBy (target isPlayer)]) 2,
        instruct they (sacrifice (allOf [isPermanent, inPile (pileOfChoice they)]))
      ]
  ],
  loyalty := 3
}

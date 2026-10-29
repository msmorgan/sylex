/-- *Call Damage Control*: each mode writes out the printed header, "Return those cards from your
graveyard to your hand.", with its bullet's target. -/
card := {
  name := "Call Damage Control",
  cost := [1, green],
  types := [sorcery],
  text := [
    spell
      (chooseModes (upTo 2)
         [
           returnToHand (target [isArtifact, isCard]) yourGraveyard,
           returnToHand (target [isCreature, isCard]) yourGraveyard,
           returnToHand (target [isEnchantment, isCard]) yourGraveyard,
           returnToHand (target [isLand, isCard]) yourGraveyard
         ])
  ]
}

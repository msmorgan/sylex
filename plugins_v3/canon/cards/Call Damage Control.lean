/-- *Call Damage Control*: the header "Return those cards from your graveyard to your hand." is
the supplied body, written once; each bullet is the target it completes into a mode. -/
card := {
  name := "Call Damage Control",
  cost := [1, green],
  types := [sorcery],
  text := [
    spell
      (chooseModesOver (upTo 2)
         [
           target [isArtifact, isCard],
           target [isCreature, isCard],
           target [isEnchantment, isCard],
           target [isLand, isCard]
         ]
         (fun cards => returnToHand cards yourGraveyard))
  ]
}

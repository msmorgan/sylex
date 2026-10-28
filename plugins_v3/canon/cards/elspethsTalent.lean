card := {
  name := "Elspeth's Talent",
  cost := [2, white, white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isPlaneswalker,
    abilityGrant (enchanted planeswalker)
      (activated (loyaltySymbol (up 1)) (create 3 (creatureToken 1 1 [white] [soldier]))),
    whenever (does you (activate (a [abilityHead loyalty, abilityOf (enchanted planeswalker)])))
      (establishFor (allOf [isCreature, youControl])
         [getsPt them (up 2) (up 2), abilityGrant them vigilance] untilEndOfTurn)
  ]
}

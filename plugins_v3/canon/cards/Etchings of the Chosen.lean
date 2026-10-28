card := {
  name := "Etchings of the Chosen",
  cost := [1, white, black],
  types := [enchantment],
  text := [
    entersChoosing thisEnchantment (subtype creature),
    getsPt (allOf [isCreature, youControl, ofChosen (subtype creature)]) (up 1) (up 1),
    activated (compound [[1], (sacrifice (a [isCreature, ofChosen (subtype creature)]))])
      (gain (target [isCreature, youControl]) indestructible untilEndOfTurn)
  ]
}

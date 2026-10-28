card := {
  name := "Shared Triumph",
  cost := [1, white],
  types := [enchantment],
  text := [
    entersChoosing thisEnchantment (subtype creature),
    getsPt (allOf [isCreature, ofChosen (subtype creature)]) (up 1) (up 1)
  ]
}

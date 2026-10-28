card := {
  name := "Rally the Ranks",
  cost := [1, white],
  types := [enchantment],
  text := [
    entersChoosing thisEnchantment (subtype creature),
    getsPt (allOf [isCreature, youControl, ofChosen (subtype creature)]) (up 1) (up 1)
  ]
}

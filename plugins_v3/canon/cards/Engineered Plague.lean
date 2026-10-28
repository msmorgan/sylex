card := {
  name := "Engineered Plague",
  cost := [2, black],
  types := [enchantment],
  text := [
    entersChoosing thisEnchantment (subtype creature),
    getsPt (allOf [isCreature, ofChosen (subtype creature)]) (down 1) (down 1)
  ]
}

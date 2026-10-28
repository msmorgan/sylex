card := {
  name := "Xenograft",
  cost := [4, blue],
  types := [enchantment],
  text := [
    entersChoosing thisEnchantment (subtype creature),
    qualityChange (allOf [isCreature, youControl]) adds
      (chosenQuality (ofChosen (subtype creature)))
  ]
}

card := {
  name := "Conspiracy",
  cost := [3, black, black],
  types := [enchantment],
  text := [
    entersChoosing thisEnchantment (subtype creature),
    qualityChange
      (and
         [
           allOf [isCreature, youControl],
           allOf [isCreature, isSpell, youControl],
           allOf [isCard, isCreature, youOwn, not (in_ battlefield)]
         ])
      sets (chosenQuality (ofChosen (subtype creature)))
  ]
}

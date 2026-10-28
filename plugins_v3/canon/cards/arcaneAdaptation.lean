card := {
  name := "Arcane Adaptation",
  cost := [2, blue],
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
      adds (chosenQuality (ofChosen (subtype creature)))
  ]
}

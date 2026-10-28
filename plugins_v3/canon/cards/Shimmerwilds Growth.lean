card := {
  name := "Shimmerwilds Growth",
  cost := [1, green],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isLand,
    entersChoosing thisAura color,
    qualityChange (enchanted land) sets (chosenQuality (ofChosen color)),
    whenever (does any (tapForMana (enchanted land)))
      (instruct (controllerOf (that land)) (addMana 1 ofTheChosenColor))
  ]
}

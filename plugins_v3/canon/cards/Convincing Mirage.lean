card := {
  name := "Convincing Mirage",
  cost := [1, blue],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isLand,
    entersChoosingFrom thisAura (subtype land) basicTypesOnly,
    qualityChange (enchanted land) sets (chosenQuality (ofChosen (subtype land)))
  ]
}

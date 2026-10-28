/-- Realmwright -/
card := {
  name := "Realmwright",
  cost := [blue],
  types := [creature],
  subtypes := [vedalken, wizard],
  text := [
    entersChoosingFrom thisCreature (subtype land) basicTypesOnly,
    qualityChange (allOf [isLand, youControl]) adds (chosenQuality (ofChosen (subtype land)))
  ],
  power := 1,
  toughness := 1
}

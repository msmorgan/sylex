card := {
  name := "Grixis Illusionist",
  cost := [blue],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    activated tapSymbol
      (establishThroughout
         (qualityChange (target [isLand, youControl]) sets
            (chosenQuality (ofYourChoice (subtype land) basicTypesOnly)))
         untilEndOfTurn)
  ],
  power := 1,
  toughness := 1
}

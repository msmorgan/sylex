card := {
  name := "Unstable Frontier",
  types := [land],
  text := [
    activated tapSymbol (addMana 1 (manaOf [[colorless]])),
    activated tapSymbol
      (establishThroughout
         (qualityChange (target [isLand, youControl]) sets
            (chosenQuality (ofYourChoice (subtype land) basicTypesOnly)))
         untilEndOfTurn)
  ]
}

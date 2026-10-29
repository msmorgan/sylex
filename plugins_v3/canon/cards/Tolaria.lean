card := {
  name := "Tolaria",
  supertypes := [legendary],
  types := [land],
  text := [
    activated tapSymbol (addMana 1 (manaOf [[of blue]])),
    activatedOnlyDuring (compound [tapSymbol])
      (establishThroughout
         (abilityLoss (target isCreature)
            [written banding, abilities (anyIn { word := bandsWithOther })])
         untilEndOfTurn)
      (duringPart upkeep none)
  ]
}

card := {
  name := "Unnatural Selection",
  cost := [1, blue],
  types := [enchantment],
  text := [
    activated [1]
      [
        choose (a (qualityFrom (subtype creature) (typeOtherThan wall))),
        establishThroughout
          (qualityChange (target isCreature) sets (chosenQuality (ofChosen (subtype creature))))
          untilEndOfTurn
      ]
  ]
}

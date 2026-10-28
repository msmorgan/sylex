card := {
  name := "Standardize",
  cost := [blue, blue],
  types := [instant],
  text := [
    spell
      [
        choose (a (qualityFrom (subtype creature) (typeOtherThan wall))),
        establishThroughout
          (qualityChange (each isCreature) sets (chosenQuality (ofChosen (subtype creature))))
          untilEndOfTurn
      ]
  ]
}

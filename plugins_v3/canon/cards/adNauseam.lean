card := {
  name := "Ad Nauseam",
  cost := [3, black, black],
  types := [instant],
  text := [
    spell
      [
        reveal (topSlice 1),
        putInto (that card) hand,
        loseLife (statOf (stat manaValue) it),
        may (repeat_ anyNumberOfTimes)
      ]
  ]
}

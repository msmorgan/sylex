card := {
  name := "Thawing Glaciers",
  types := [land],
  text := [
    entersTapped thisLand,
    activated (compound [[1], tapSymbol])
      [
        search yourLibrary (a [isLand, hasSupertype basic]),
        putOntoBattlefieldTapped (that card),
        shuffle,
        atNext cleanup (returnToHand thisLand)
      ]
  ]
}

/-- Heart of Yavimaya -/
card := {
  name := "Heart of Yavimaya",
  types := [land],
  text := [
    replace (would thisLand enter)
      (instead
         (doIfDone (sacrifice (a [isLand, forest])) (putOntoBattlefield thisLand)
            (putInto it graveyard))),
    activated tapSymbol (addMana 1 (manaOf [[of green]])),
    activated tapSymbol (get (target isCreature) (up 1) (up 1) untilEndOfTurn)
  ]
}

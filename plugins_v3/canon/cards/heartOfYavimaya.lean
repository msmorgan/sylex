/-- Heart of Yavimaya -/
card heartOfYavimaya : Spelled :=
  { name := "Heart of Yavimaya", types := [.land],
    text :=
      [ static
          (replace (would thisLand enter)
            (instead (doIfDone (sacrifice (a (and [isLand, forest])))
              (putOntoBattlefield thisLand) (putInto it graveyard)))),
        activated tapSymbol (addMana 1 (manaOf [[.of .green]])),
        activated tapSymbol
          (get (target isCreature) (up 1) (up 1) untilEndOfTurn) ] }

/-- Mox Diamond -/
card moxDiamond : Spelled :=
  { name := "Mox Diamond", cost := some [], types := [.artifact],
    text :=
      [ static
          (replace (would thisArtifact enter)
            (instead (may (discard (a (and [isLand, in_ yourHand])))
              (putOntoBattlefield thisArtifact) (putInto it graveyard)))),
        activated tapSymbol (addMana 1 (ofAnyColor .sameColor)) ] }

card := {
  name := "Mox Diamond",
  cost := some [],
  types := [artifact],
  text := [
    replace (would thisArtifact enter)
      (instead
         (may (discard (a [isLand, in_ yourHand])) (putOntoBattlefield thisArtifact)
            (putInto it graveyard))),
    activated tapSymbol (addMana 1 (ofAnyColor sameColor))
  ]
}

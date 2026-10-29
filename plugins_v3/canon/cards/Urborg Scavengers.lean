card := {
  name := "Urborg Scavengers",
  cost := [2, black],
  types := [creature],
  subtypes := [spirit],
  text := [
    whenever (does thisCreature enter) (or_ := [does thisCreature attack])
      [exile (target (cardIn graveyard)), putCounters 1 p1p1Counter thisCreature],
    alsoForKeywords
      (static
         (onlyWhile
            (keywordGrant thisCreature thatKeyword
               [in_ (exiledWith thisCreature), hasKeyword thatKeyword])
            (exists_ [in_ (exiledWith thisCreature), hasKeyword thatKeyword])))
      [
        flying,
        firstStrike,
        doubleStrike,
        deathtouch,
        haste,
        (anyIn { word := hexproof }),
        indestructible,
        lifelink,
        menace,
        reach,
        trample,
        vigilance
      ]
  ],
  power := 2,
  toughness := 2
}

card := {
  name := "Urborg Scavengers",
  cost := [2, black],
  types := [creature],
  subtypes := [spirit],
  text := [
    whenever (does thisCreature enter) (or_ := [does thisCreature attack])
      [exile (target (cardIn graveyard)), putCounters 1 p1p1Counter thisCreature],
    alsoForKeywords
      (fun k => static
         (onlyWhile (keywordGrant thisCreature k [in_ (exiledWith thisCreature), hasKeyword k])
            (exists_ [in_ (exiledWith thisCreature), hasKeyword k])))
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

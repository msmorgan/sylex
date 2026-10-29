card := {
  name := "Cairn Wanderer",
  cost := [4, black],
  types := [creature],
  subtypes := [shapeshifter],
  text := [
    changeling,
    alsoForKeywords
      (static
         (onlyWhile
            (keywordGrant thisCreature thatKeyword
               [isCreature, in_ graveyard, hasKeyword thatKeyword])
            (exists_ [isCreature, in_ graveyard, hasKeyword thatKeyword])))
      [
        flying,
        fear,
        firstStrike,
        doubleStrike,
        deathtouch,
        haste,
        (anyIn { word := landwalk }),
        lifelink,
        (anyIn { word := protection }),
        reach,
        trample,
        shroud,
        vigilance
      ]
  ],
  power := 4,
  toughness := 4
}

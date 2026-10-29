card := {
  name := "Bleeding Effect",
  cost := [2, white, black],
  types := [enchantment],
  text := [
    alsoForKeywords
      (at_ (beginningOfYour combat)
         (doOnlyIf
            (establishThroughout
               (keywordGrant (allOf [isCreature, youControl]) thatKeyword
                  [isCreature, in_ yourGraveyard, hasKeyword thatKeyword])
               untilEndOfTurn)
            (exists_ [isCreature, in_ yourGraveyard, hasKeyword thatKeyword])))
      [
        flying,
        firstStrike,
        doubleStrike,
        deathtouch,
        (anyIn { word := hexproof }),
        indestructible,
        lifelink,
        menace,
        reach,
        trample,
        vigilance
      ]
  ]
}

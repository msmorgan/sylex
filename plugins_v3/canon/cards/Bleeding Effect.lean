card := {
  name := "Bleeding Effect",
  cost := [2, white, black],
  types := [enchantment],
  text := [
    alsoForKeywords
      (fun k => at_ (beginningOfYour combat)
         (doOnlyIf
            (establishThroughout
               (keywordGrant (allOf [isCreature, youControl]) k
                  [isCreature, in_ yourGraveyard, hasKeyword k])
               untilEndOfTurn)
            (exists_ [isCreature, in_ yourGraveyard, hasKeyword k])))
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

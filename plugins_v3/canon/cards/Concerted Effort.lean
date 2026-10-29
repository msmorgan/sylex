card := {
  name := "Concerted Effort",
  cost := [2, white, white],
  types := [enchantment],
  text := [
    alsoForKeywords
      (at_ (beginningOfPossessed upkeep eachPlayer)
         (doOnlyIf
            (establishThroughout
               (keywordGrant (allOf [isCreature, youControl]) thatKeyword
                  [isCreature, youControl, hasKeyword thatKeyword])
               untilEndOfTurn)
            (exists_ [isCreature, youControl, hasKeyword thatKeyword])))
      [
        flying,
        fear,
        firstStrike,
        doubleStrike,
        (anyIn { word := landwalk }),
        (anyIn { word := protection }),
        trample,
        vigilance
      ]
  ]
}

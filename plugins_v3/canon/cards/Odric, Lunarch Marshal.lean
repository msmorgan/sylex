card := {
  name := "Odric, Lunarch Marshal",
  cost := [3, white],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, soldier],
  text := [
    alsoForKeywords
      (at_ (beginningOfPossessed combat eachPlayer)
         (doOnlyIf
            (establishThroughout
               (keywordGrant (allOf [isCreature, youControl]) thatKeyword
                  [isCreature, youControl, hasKeyword thatKeyword])
               untilEndOfTurn)
            (exists_ [isCreature, youControl, hasKeyword thatKeyword])))
      [
        firstStrike,
        flying,
        deathtouch,
        doubleStrike,
        haste,
        (anyIn { word := hexproof }),
        indestructible,
        lifelink,
        menace,
        reach,
        skulk,
        trample,
        vigilance
      ]
  ],
  power := 3,
  toughness := 3
}

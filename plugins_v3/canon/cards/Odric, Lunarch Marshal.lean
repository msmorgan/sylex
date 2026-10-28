card := {
  name := "Odric, Lunarch Marshal",
  cost := [3, white],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, soldier],
  text := [
    alsoForKeywords
      (fun k => at_ (beginningOfPossessed combat eachPlayer)
         (doOnlyIf
            (establishThroughout
               (keywordGrant (allOf [isCreature, youControl]) k
                  [isCreature, youControl, hasKeyword k])
               untilEndOfTurn)
            (exists_ [isCreature, youControl, hasKeyword k])))
      [
        firstStrike,
        flying,
        deathtouch,
        doubleStrike,
        haste,
        (anyIn ⟨hexproof, none⟩),
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

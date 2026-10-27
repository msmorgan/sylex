card odricLunarchMarshal : Spelled :=
  { name := "Odric, Lunarch Marshal", cost := [generic 3, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, soldier],
    text :=
      [ alsoForKeywords
          (fun k => at_ (beginningOfPossessed .combat eachPlayer)
            (doOnlyIf
              (establishThroughout
                (keywordGrant (allOf (and [isCreature, youControl])) k
                  (and [isCreature, youControl, hasKeyword k]))
                untilEndOfTurn)
              (exists_ (and [isCreature, youControl, hasKeyword k]))))
          [ firstStrike, flying, deathtouch, doubleStrike, haste, hexproofAbilities,
            indestructible, lifelink, menace, reach, skulk, trample, vigilance ] ],
    power := stat 3, toughness := stat 3 }

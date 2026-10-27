card consulsLieutenant : Spelled :=
  { name := "Consul's Lieutenant", cost := [pip .white, pip .white], types := [.creature],
    subtypes := [human, soldier],
    text :=
      [ firstStrike,
        renown 1,
        whenever (does thisCreature attack) (if_ := is it (hasDesignation renowned))
          (get (allOf (and [isCreature, isAttacking, youControl, isOther]))
            (up 1) (up 1) untilEndOfTurn) ],
    power := stat 2, toughness := stat 1 }

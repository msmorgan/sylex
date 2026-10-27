/-- Syr Konrad, the Grim -/
card syrKonradTheGrim : Spelled :=
  { name := "Syr Konrad, the Grim", cost := [generic 3, pip .black, pip .black],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, knight],
    text :=
      [ whenever (does (a (and [isCreature, isOther])) die)
          (or_ :=
            [ does any
                (putInto (a (and [isCard, isCreature])) graveyard
                  (from_ := some (not battlefield))),
            does (a (and [isCreature, in_ yourGraveyard])) (leave
                                                                      (from_ := yourGraveyard)) ])
          (instruct this_ (dealDamage 1 eachOpponent)),
        activated [generic 1, pip .black] (instruct eachPlayer (mill 1)) ],
    power := stat 5, toughness := stat 4 }

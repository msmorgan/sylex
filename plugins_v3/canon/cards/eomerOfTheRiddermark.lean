card eomerOfTheRiddermark : Spelled :=
  { name := "Éomer of the Riddermark", cost := [generic 4, pip .red],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, knight],
    text :=
      [ haste,
        whenever (does thisCreature attack)
          (if_ := exists_ (and [ isCreature, youControl,
                           superlative .max (.stat .power) (and [isCreature, isPermanent]) ]))
          (create 1 (creatureToken 1 1 [.white] [human, soldier])) ],
    power := stat 5, toughness := stat 4 }

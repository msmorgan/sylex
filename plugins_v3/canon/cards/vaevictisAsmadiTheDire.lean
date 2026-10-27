/-- Vaevictis Asmadi, the Dire -/
card vaevictisAsmadiTheDire : Spelled :=
  { name := "Vaevictis Asmadi, the Dire",
    cost := [generic 3, pip .black, pip .red, pip .green],
    supertypes := [.legendary], types := [.creature],
    subtypes := [elder, dragon],
    text :=
      [ flying,
        whenever (does thisCreature attack)
          (sequentially
            [ doForEach eachPlayer
                (choose (target (and [isPermanent, controlledBy they]))),
              instruct (those player) (sacrifice (those permanent)),
              doForEach
                (each (and [ isPlayer,
                              did which (sacrifice (a isPermanent)) thisWay
                                ]))
                (sequentially
                  [ instruct they (reveal (slice .top 1 (libraryOf they))),
                    doIf (is it isPermanentCard) (putOntoBattlefield it) ]) ]) ],
    power := stat 6, toughness := stat 6 }

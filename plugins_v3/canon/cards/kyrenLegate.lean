card kyrenLegate : Spelled :=
  { name := "Kyren Legate", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [goblin],
    text :=
      [ static (onlyWhile (altCost this_)
          (and
            [ exists_ (and [ isLand, plains,
                              anOpponentControls ]),
              exists_ (and [ isLand, mountain,
                              youControl ]) ])),
        haste ],
    power := stat 1, toughness := stat 1 }

card massacre : Spelled :=
  { name := "Massacre", cost := [generic 2, pip .black, pip .black], types := [.sorcery],
    text :=
      [ static (onlyWhile (altCost this_)
          (and
            [ exists_ (and [ isLand, plains,
                              anOpponentControls ]),
              exists_ (and [ isLand, swamp,
                              youControl ]) ])),
        spell (get (allOf isCreature) (down 2) (down 2) untilEndOfTurn)
            ] }

card deepwayNavigator : Spelled :=
  { name := "Deepway Navigator", cost := [pip .white, pip .blue], types := [.creature],
    subtypes := [merfolk, wizard],
    text :=
      [ flash,
        when (does thisCreature enter)
          (untap
            (each (and [ merfolk, youControl,
                          isOther ]))),
        static (onlyWhile
          (getsPt (allOf (and [merfolk, youControl]))
            (up 1) (up 0))
          (did you (attackWith (counted (orMore 3)
            merfolk)) soFarThisTurn)) ],
    power := stat 2, toughness := stat 2 }

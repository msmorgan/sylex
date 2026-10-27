card thwart : Spelled :=
  { name := "Thwart", cost := [generic 2, pip .blue, pip .blue], types := [.instant],
    text :=
      [ static (altCost this_
          (returnToHand
            (counted (exactly 3)
              (and [isLand, island, youControl])))),
        spell (counter (target isSpell)) ] }

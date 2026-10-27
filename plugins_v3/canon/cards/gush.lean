card gush : Spelled :=
  { name := "Gush", cost := [generic 4, pip .blue], types := [.instant],
    text :=
      [ static (altCost this_
          (returnToHand
            (counted (exactly 2)
              (and [isLand, island, youControl])))),
        spell (draw 2) ] }

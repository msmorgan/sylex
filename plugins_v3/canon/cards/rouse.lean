card rouse : Spelled :=
  { name := "Rouse", cost := [generic 1, pip .black], types := [.instant],
    text :=
      [ static (onlyWhile (altCost this_ (payLife 2))
          (exists_ (and [isLand, swamp, youControl]))),
        spell (get (target isCreature) (up 2) (up 0) untilEndOfTurn) ]
            }

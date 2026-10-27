/-- Invigorate -/
card invigorate : Spelled :=
  { name := "Invigorate", cost := [generic 2, pip .green], types := [.instant],
    text :=
      [ static (onlyWhile (altCost this_ (instruct anOpponent (gainLife 3)))
          (exists_ (and [isLand, forest, youControl]))),
        spell (get (target isCreature) (up 4) (up 4) untilEndOfTurn) ]
            }

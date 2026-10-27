card admiralsOrder : Spelled :=
  { name := "Admiral's Order", cost := [generic 1, pip .blue, pip .blue], types := [.instant],
    text :=
      [ static (onlyWhile (altCost this_ [pip .blue])
          (did you attack soFarThisTurn)),
        spell (counter (target isSpell)) ] }

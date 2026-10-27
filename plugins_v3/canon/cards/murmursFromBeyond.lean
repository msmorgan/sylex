card murmursFromBeyond : Spelled :=
  { name := "Murmurs from Beyond", cost := [generic 2, pip .blue], types := [.instant],
    subtypes := [arcane],
    text :=
      [ spell (sequentially
          [ reveal (topSlice 3),
            instruct anOpponent (choose (someOf (exactly 1) them)),
            putInto (that card) graveyard,
            putInto theRest hand ]) ] }

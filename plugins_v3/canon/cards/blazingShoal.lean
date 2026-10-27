card blazingShoal : Spelled :=
  { name := "Blazing Shoal", cost := [.variable, pip .red, pip .red], types := [.instant],
    subtypes := [arcane],
    text :=
      [ static (altCost this_
          (exile
            (a (and [hasColor .red, hasManaValue .eq (letter .x),
                      in_ yourHand])))),
        spell (get (target isCreature) (up (letter .x)) (up 0) untilEndOfTurn) ] }

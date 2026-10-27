card sickeningShoal : Spelled :=
  { name := "Sickening Shoal", cost := [.variable, pip .black, pip .black], types := [.instant],
    subtypes := [arcane],
    text :=
      [ static (altCost this_
          (exile
            (a (and [hasColor .black, hasManaValue .eq (letter .x),
                      in_ yourHand])))),
        spell (get (target isCreature) (down (letter .x)) (down (letter .x)) untilEndOfTurn)
              ] }

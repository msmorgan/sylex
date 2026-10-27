card nourishingShoal : Spelled :=
  { name := "Nourishing Shoal", cost := [.variable, pip .green, pip .green], types := [.instant],
    subtypes := [arcane],
    text :=
      [ static (altCost this_
          (exile
            (a (and [hasColor .green, hasManaValue .eq (letter .x),
                      in_ yourHand])))),
        spell (gainLife (letter .x)) ] }

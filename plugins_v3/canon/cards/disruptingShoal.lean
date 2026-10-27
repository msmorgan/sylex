card disruptingShoal : Spelled :=
  { name := "Disrupting Shoal", cost := [.variable, pip .blue, pip .blue], types := [.instant],
    subtypes := [arcane],
    text :=
      [ static (altCost this_
          (exile
            (a (and [hasColor .blue, hasManaValue .eq (letter .x),
                      in_ yourHand])))),
        spell (doOnlyIf (counter (target isSpell))
            (compareAmt (statOf (.stat .manaValue) it) .eq (letter .x))) ] }

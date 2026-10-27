card shiningShoal : Spelled :=
  { name := "Shining Shoal", cost := [.variable, pip .white, pip .white], types := [.instant],
    subtypes := [arcane],
    text :=
      [ static (altCost this_
          (exile
            (a (and [hasColor .white, hasManaValue .eq (letter .x),
                      in_ yourHand])))),
        spell (establishThroughout
              (replace
                (would (aYourChoice isSource)
                  (dealDamage anyAmount (andOr [you, allOf (and [isCreature, youControl])])))
                (instead (redirectDamage (theNext (letter .x)) (target isAnyTarget))))
              thisTurn) ] }

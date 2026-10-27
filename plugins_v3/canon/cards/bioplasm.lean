/-- Bioplasm -/
card bioplasm : Spelled :=
  { name := "Bioplasm", cost := [generic 3, pip .green, pip .green], types := [.creature],
    subtypes := [ooze],
    text :=
      [ whenever (does thisCreature attack)
          (sequentially
            [ exile (topSlice 1),
              doIf (is it (and [isCard, isCreature]))
                (get thisCreature
                  (up (powerOf (the (and [was (exile which), isCreature, isCard]))))
                  (up (toughnessOf it))
                  thisTurn) ]) ],
    power := stat 4, toughness := stat 4 }

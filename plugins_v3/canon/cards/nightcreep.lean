/-- Nightcreep -/
card nightcreep : Spelled :=
  { name := "Nightcreep", cost := [pip .black, pip .black], types := [.instant],
    text :=
      [ spell
          (establishThroughout
             (conjunction [ qualityChange (allOf isCreature) .sets (colored (.some [.black])),
              qualityChange (allOf isLand) .sets
                (bundle { characteristics := { subtypes := [swamp] } }) ]) untilEndOfTurn) ] }

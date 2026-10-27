/-- Brazen Blademaster -/
card brazenBlademaster : Spelled :=
  { name := "Brazen Blademaster", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [orc, pirate],
    text :=
      [ whenever (does thisCreature attack)
          (while_ := whileTrue
            (compareAmt (countOf (and [isArtifact, youControl])) .atLeast
              2))
          (get it (up 2) (up 1) untilEndOfTurn) ],
    power := stat 2, toughness := stat 3 }

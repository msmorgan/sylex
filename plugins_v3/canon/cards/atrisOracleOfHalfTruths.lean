/-- Atris, Oracle of Half-Truths -/
card atrisOracleOfHalfTruths : Spelled :=
  { name := "Atris, Oracle of Half-Truths", cost := [generic 2, pip .blue, pip .black],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, advisor],
    text :=
      [ menace,
        when (does thisCreature enter)
          (sequentially
            [ instruct (target isOpponent) (lookAt (topSlice 3)),
              instruct they (separateIntoPiles them 2 [.faceDown, .faceUp]),
              putInto onePile hand,
              putInto theOther graveyard ]) ],
    power := stat 3, toughness := stat 2 }

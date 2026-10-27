/-- Sphinx of Uthuun -/
card sphinxOfUthuun : Spelled :=
  { name := "Sphinx of Uthuun", cost := [generic 5, pip .blue, pip .blue], types := [.creature],
    subtypes := [sphinx],
    text :=
      [ flying,
        when (does thisCreature enter)
          (sequentially
            [ reveal (topSlice 5),
              instruct anOpponent (separateIntoPiles them 2),
              putInto onePile hand,
              putInto theOther graveyard ]) ],
    power := stat 5, toughness := stat 6 }

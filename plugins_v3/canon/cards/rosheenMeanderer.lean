/-- Rosheen Meanderer -/
card rosheenMeanderer : Spelled :=
  { name := "Rosheen Meanderer", cost := [generic 3, hybridPip .red .green],
    supertypes := [.legendary], types := [.creature],
    subtypes := [giant, shaman],
    text :=
      [ activated tapSymbol
          (addMana 1 (manaOf [[.colorless, .colorless, .colorless, .colorless]])
            [spendOnly [toPay (.containing .variable)]]) ],
    power := stat 4, toughness := stat 4 }

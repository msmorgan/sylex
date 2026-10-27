/-- Overgrown Zealot -/
card overgrownZealot : Spelled :=
  { name := "Overgrown Zealot", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [elf, druid],
    text :=
      [ activated tapSymbol (addMana 1 (ofAnyColor .sameColor)),
        activated tapSymbol
          (addMana 2 (ofAnyColor .sameColor)
            [spendOnly [toTurnFaceUp isPermanent]]) ],
    power := stat 0, toughness := stat 4 }

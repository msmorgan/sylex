/-- Latchkey Faerie -/
card latchkeyFaerie : Spelled :=
  { name := "Latchkey Faerie", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [faerie, rogue],
    text :=
      [ flying,
        prowl [generic 2, pip .blue],
        when (does thisCreature enter) (if_ := keywordCostPaid prowl
                                                         thisCreature)
          (draw 1) ],
    power := stat 3, toughness := stat 1 }

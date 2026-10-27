/-- Tyrant of Valakut -/
card tyrantOfValakut : Spelled :=
  { name := "Tyrant of Valakut", cost := [generic 5, pip .red, pip .red], types := [.creature],
    subtypes := [dragon],
    text :=
      [ surge [generic 3, pip .red, pip .red],
        flying,
        when (does thisCreature enter) (if_ := keywordCostPaid surge
                                                         thisCreature)
          (instruct thisCreature (dealDamage 3 (target isAnyTarget))) ],
    power := stat 5, toughness := stat 4 }

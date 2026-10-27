/-- Krosan Druid -/
card krosanDruid : Spelled :=
  { name := "Krosan Druid", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [centaur, druid],
    text :=
      [ kicker [generic 4, pip .green],
        when (does thisCreature enter) (if_ := keywordCostPaid kicker
                                                         thisCreature)
          (gainLife 10) ],
    power := stat 2, toughness := stat 3 }

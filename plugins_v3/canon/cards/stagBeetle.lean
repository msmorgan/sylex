card stagBeetle : Spelled :=
  { name := "Stag Beetle", cost := [generic 3, pip .green, pip .green], types := [.creature],
    subtypes := [insect],
    text :=
      [ static (conjunction [ entersWithCounters thisCreature (letter .x) p1p1Counter,
            letterDefinition .x (countOf (and [isCreature, isOther])) ]) ],
    power := stat 0, toughness := stat 0 }

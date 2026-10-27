card rakshasaVizier : Spelled :=
  { name := "Rakshasa Vizier", cost := [generic 2, pip .black, pip .green, pip .blue],
    types := [.creature], subtypes := [demon],
    text :=
      [ whenever
          (does any
             (putInto (counted (orMore 1) (cardIn yourGraveyard)) exileZone
                (from_ := yourGraveyard)))
          (putCounters groupSize p1p1Counter thisCreature) ],
    power := stat 4, toughness := stat 4 }

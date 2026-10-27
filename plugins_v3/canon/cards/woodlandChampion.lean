card woodlandChampion : Spelled :=
  { name := "Woodland Champion", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [elf, scout],
    text :=
      [ whenever (does (counted (orMore 1) (and [isToken, youControl])) enter)
          (putCounters groupSize p1p1Counter thisCreature) ],
    power := stat 2, toughness := stat 2 }

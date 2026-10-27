card agelessEntity : Spelled :=
  { name := "Ageless Entity", cost := [generic 3, pip .green, pip .green], types := [.creature],
    subtypes := [elemental],
    text :=
      [ whenever (does you (gainLife anyAmount))
          (putCounters thatMuch p1p1Counter thisCreature) ],
    power := stat 4, toughness := stat 4 }

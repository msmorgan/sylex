card nefariousImp : Spelled :=
  { name := "Nefarious Imp", cost := [generic 2, pip .black], types := [.creature],
    subtypes := [imp],
    text :=
      [ flying,
        whenever
          (does (counted (orMore 1) (and [isPermanent, youControl])) leave)
          (scry 1) ],
    power := stat 2, toughness := stat 1 }

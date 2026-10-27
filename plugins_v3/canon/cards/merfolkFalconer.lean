/-- Merfolk Falconer -/
card merfolkFalconer : Spelled :=
  { name := "Merfolk Falconer", cost := [generic 3, pip .blue, pip .blue], types := [.creature],
    subtypes := [merfolk, wizard],
    text :=
      [ flying,
        whenever
          (does you (cast (a (compareOver isSpell (paidCostRead (.byKeyword kicker)
              none it) .atLeast 1))))
          (scry 2) ],
    power := stat 4, toughness := stat 4 }

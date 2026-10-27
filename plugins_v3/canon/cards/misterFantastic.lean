card misterFantastic : Spelled :=
  { name := "Mister Fantastic, Reed Richards", cost := [generic 3, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, scientist, hero],
    text :=
      [ reach,
        whenever
          (does (counted (orMore 1) (and [isToken, youControl])) enter)
          (may (draw 1)) ],
    power := stat 2, toughness := stat 4 }

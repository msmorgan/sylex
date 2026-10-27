/-- Bard, Heir of Girion -/
card bardHeirOfGirion : Spelled :=
  { name := "Bard, Heir of Girion", cost := [generic 2, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, archer],
    text :=
      [ reach, vigilance,
        static
            (getsPt (allOf (and [isCreature, youControl, isOther])) (up 1)
               (up 1)),
        whenever (does you attack) (draw 1) ],
    power := stat 4, toughness := stat 4 }

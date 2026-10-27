card drudgeSkeletons : Spelled :=
  { name := "Drudge Skeletons", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [skeleton],
    text := [activated [pip .black] (regenerate thisCreature)],
    power := stat 1, toughness := stat 1 }

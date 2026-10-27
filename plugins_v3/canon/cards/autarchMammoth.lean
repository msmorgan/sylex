/-- Autarch Mammoth -/
card autarchMammoth : Spelled :=
  { name := "Autarch Mammoth", cost := [generic 4, pip .green, pip .green], types := [.creature],
    subtypes := [elephant, mount],
    text := [autarchMammothLine, saddle 5],
    power := stat 5, toughness := stat 5 }

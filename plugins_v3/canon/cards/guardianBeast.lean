/-- Guardian Beast -/
card guardianBeast : Spelled :=
  { name := "Guardian Beast", cost := [generic 3, pip .black], types := [.creature],
    subtypes := [beast],
    text :=
      [ static guardianBeastProtection ],
    power := stat 2, toughness := stat 4 }

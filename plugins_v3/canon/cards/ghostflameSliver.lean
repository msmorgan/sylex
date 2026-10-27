/-- Ghostflame Sliver -/
card ghostflameSliver : Spelled :=
  { name := "Ghostflame Sliver", cost := [pip .black, pip .red], types := [.creature],
    subtypes := [sliver],
    text :=
      [ static (qualityChange (allOf sliver) .sets (colored
          (.some []))) ],
    power := stat 2, toughness := stat 2 }

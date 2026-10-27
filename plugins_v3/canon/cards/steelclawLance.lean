/-- Steelclaw Lance -/
card steelclawLance : Spelled :=
  { name := "Steelclaw Lance", cost := [pip .black, pip .red], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (getsPt (equipped creature) (up 2) (up 2)),
        equipQuality knight [generic 1],
        equip [generic 3] ] }

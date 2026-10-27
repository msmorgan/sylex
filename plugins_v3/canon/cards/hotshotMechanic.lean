/-- Hotshot Mechanic: "This creature crews Vehicles as though its power were 2 greater." An
indicative "as though" statement about how it crews, not a permission to crew [CR#609.4]. -/
card hotshotMechanic : Spelled :=
  { name := "Hotshot Mechanic", cost := [pip .white], types := [.artifact, .creature],
    subtypes := [fox, pilot],
    text :=
      [ static (asThough thisCreature (crew (allOf (hasSubtype vehicle))) (greaterBy .power 2)) ],
    power := stat 2, toughness := stat 1 }

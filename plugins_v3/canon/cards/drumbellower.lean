/-- Drumbellower -/
card drumbellower : Spelled :=
  { name := "Drumbellower", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [spirit],
    text :=
      [ flying,
        static (untapDuring (allOf (and [isCreature, youControl]))
          (each isOtherPlayer)) ],
    power := stat 2, toughness := stat 1 }

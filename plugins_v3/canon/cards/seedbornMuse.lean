/-- Seedborn Muse -/
card seedbornMuse : Spelled :=
  { name := "Seedborn Muse", cost := [generic 3, pip .green, pip .green],
    types := [.creature], subtypes := [spirit],
    text :=
      [ static (untapDuring (allOf youControl) (each isOtherPlayer)) ],
    power := stat 2, toughness := stat 4 }

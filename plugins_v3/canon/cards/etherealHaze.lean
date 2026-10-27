card etherealHaze : Spelled :=
  { name := "Ethereal Haze", cost := [pip .white], types := [.instant],
    subtypes := [arcane],
    text := [spell (preventAllBy (allOf isCreature) any thisTurn)] }

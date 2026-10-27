/-- Eyetwitch -/
card eyetwitch : Spelled :=
  { name := "Eyetwitch", cost := [pip .black], types := [.creature], subtypes := [eye, bat],
    text := [flying, when (does thisCreature die) learn],
    power := stat 1, toughness := stat 1 }

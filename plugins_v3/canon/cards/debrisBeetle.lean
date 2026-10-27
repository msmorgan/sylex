/-- Debris Beetle -/
card debrisBeetle : Spelled :=
  { name := "Debris Beetle", cost := [generic 2, pip .black, pip .green], types := [.artifact],
    subtypes := [vehicle],
    text :=
      [ trample,
        when (does thisVehicle enter)
          (sequentially [instruct eachOpponent (loseLife 3), gainLife 3]),
        crew 2 ],
    power := stat 6, toughness := stat 6 }

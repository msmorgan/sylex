/-- Scheming Aspirant -/
card schemingAspirant : Spelled :=
  { name := "Scheming Aspirant", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [phyrexian, advisor],
    text :=
      [ whenever (does you proliferate)
          (sequentially [instruct eachOpponent (loseLife 2), gainLife 2]) ],
    power := stat 1, toughness := stat 3 }

card adrixAndNev : Spelled :=
  { name := "Adrix and Nev, Twincasters", cost := [generic 2, pip .green, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [merfolk, wizard],
    text :=
      [ ward [generic 2],
        static (replace (would any (createObject anyAmount (token (copyOf (counted (orMore 1) isToken)) [])
            you)) (instead (create (times 2 groupSize) asThose []))) ],
    power := stat 2, toughness := stat 2 }

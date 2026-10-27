card timeSieve : Spelled :=
  { name := "Time Sieve", cost := [pip .blue, pip .black], types := [.artifact],
    text :=
      [ activated (compound [tapSymbol, sacrifice (counted (exactly 5) isArtifact)])
          (extraTurn 1) ] }

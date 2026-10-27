/-- Pir, Imaginative Rascal -/
card pirImaginativeRascal : Spelled :=
  { name := "Pir, Imaginative Rascal", cost := [generic 2, pip .green],
    supertypes := [.legendary], types := [.creature], subtypes := [human],
    text :=
      [ partnerWith (named "Toothy, Imaginary Friend"),
        pirDistributive ],
    power := stat 1, toughness := stat 1 }

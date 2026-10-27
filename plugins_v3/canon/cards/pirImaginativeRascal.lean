/-- Pir, Imaginative Rascal -/
card pirImaginativeRascal : Spelled :=
  { name := "Pir, Imaginative Rascal", cost := [generic 2, pip .green],
    supertypes := [.legendary], types := [.creature], subtypes := [human],
    text :=
      [ partnerWith (named "Toothy, Imaginary Friend"),
        (static (replace
                      (would any
                         (putCounters anyAmount anyKind
                            (a (and [isPermanent, controlledBy (youOr (a isTeammate))]))))
                                (instead (putCounters (plus thatMuch 1) thoseCounters (that
                                  permanent))))) ],
    power := stat 1, toughness := stat 1 }

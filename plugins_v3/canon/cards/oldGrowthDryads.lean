/-- Old-Growth Dryads -/
card oldGrowthDryads : Spelled :=
  { name := "Old-Growth Dryads", cost := [pip .green], types := [.creature],
    subtypes := [dryad],
    text :=
      [ when (does thisCreature enter)
          (instruct eachOpponent (may
            (sequentially
              [ search (libraryOf they) (a (and [isLand, hasSupertype .basic])),
                putOntoBattlefieldTapped it,
                shuffle ]))) ],
    power := stat 3, toughness := stat 3 }

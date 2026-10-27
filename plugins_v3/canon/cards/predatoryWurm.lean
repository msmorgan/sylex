/-- Predatory Wurm -/
card predatoryWurm : Spelled :=
  { name := "Predatory Wurm", cost := [generic 3, pip .green], types := [.creature],
    subtypes := [wurm],
    text :=
      [ vigilance,
        static (onlyWhile (getsPt thisCreature (up 2) (up 2))
          (exists_ (and [garruk, youControl]))) ],
    power := stat 4, toughness := stat 4 }

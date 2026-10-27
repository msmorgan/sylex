/-- Bristlepack Sentry -/
card bristlepackSentry : Spelled :=
  { name := "Bristlepack Sentry", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [plant, wolf],
    text :=
      [ defender,
        static (onlyWhile
          (can thisCreature attack
             (asThough :=
                holds (is it (not (hasKeyword defender)))))
          (exists_ (and [ isCreature, youControl,
                           hasPower .atLeast 4 ]))) ],
    power := stat 3, toughness := stat 3 }

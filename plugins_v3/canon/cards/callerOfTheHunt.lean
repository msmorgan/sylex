/-- Caller of the Hunt -/
card callerOfTheHunt : Spelled :=
  { name := "Caller of the Hunt", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [human],
    text :=
      [ static (addedCost (choose (a (quality (.subtype .creature)))) false),
        static (ptDefinition thisCreature .bothEach
          (countOf (and [isCreature, ofChosen (.subtype .creature)]))) ] }

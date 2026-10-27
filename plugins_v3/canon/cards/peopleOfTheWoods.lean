card peopleOfTheWoods : Spelled :=
  { name := "People of the Woods", cost := [pip .green, pip .green], types := [.creature],
    subtypes := [human],
    text :=
      [ static (ptDefinition thisCreature .toughnessAlone
          (countOf (and [forest, youControl]))) ],
    power := stat 1 }

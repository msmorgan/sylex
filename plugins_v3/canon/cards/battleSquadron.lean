card battleSquadron : Spelled :=
  { name := "Battle Squadron", cost := [generic 3, pip .red, pip .red], types := [.creature],
    subtypes := [goblin],
    text :=
      [ flying,
        static (ptDefinition thisCreature .bothEach (countOf (and [isCreature, youControl]))) ] }
